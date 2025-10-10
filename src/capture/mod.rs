use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use crate::ffmpeg;

/// Capture format options
#[derive(Debug, Clone)]
pub enum CaptureFormat {
    Mp4,        // H.264 + AAC
    Mkv,        // Matroska container
}

/// Capture options
#[derive(Debug, Clone)]
pub struct CaptureOptions {
    pub format: CaptureFormat,
    pub video_device: String,        // e.g., "/dev/video0"
    pub audio_device: String,        // e.g., "hw:1,0"
    pub deinterlace: bool,
    pub stabilize: bool,
    pub denoise: Option<String>,     // "hqdn3d" or "nlmeans"
    pub video_bitrate: Option<String>, // e.g., "5M"
    pub crf: Option<u32>,            // CRF value (18-28)
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<u32>,
    pub audio_bitrate: String,       // e.g., "192k"
    pub archival_mode: bool,         // Lossless/near-lossless
}

impl Default for CaptureOptions {
    fn default() -> Self {
        Self {
            format: CaptureFormat::Mp4,
            video_device: "/dev/video0".to_string(),
            audio_device: "hw:1,0".to_string(),
            deinterlace: true,
            stabilize: false,
            denoise: Some("hqdn3d".to_string()),
            video_bitrate: None,
            crf: Some(23),
            width: Some(720),
            height: Some(480),
            fps: Some(30),
            audio_bitrate: "192k".to_string(),
            archival_mode: false,
        }
    }
}

/// List available V4L2 video devices
pub fn list_video_devices() -> Result<Vec<String>> {
    let mut devices = Vec::new();

    // Try v4l2-ctl first
    if which::which("v4l2-ctl").is_ok() {
        let output = Command::new("v4l2-ctl")
            .arg("--list-devices")
            .output()
            .context("Failed to execute v4l2-ctl")?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.trim().starts_with("/dev/video") {
                    devices.push(line.trim().to_string());
                }
            }
        }
    }

    // Fallback: try ffmpeg device listing
    if devices.is_empty() {
        let output = Command::new("ffmpeg")
            .arg("-f")
            .arg("v4l2")
            .arg("-list_devices")
            .arg("true")
            .arg("-i")
            .arg("dummy")
            .stderr(Stdio::piped())
            .output();

        if let Ok(output) = output {
            let error_str = String::from_utf8_lossy(&output.stderr);
            for line in error_str.lines() {
                if line.contains("/dev/video") {
                    if let Some(dev) = line.split_whitespace().find(|s| s.starts_with("/dev/video")) {
                        devices.push(dev.to_string());
                    }
                }
            }
        }
    }

    // If still empty, check if /dev/video0 exists
    if devices.is_empty() && std::path::Path::new("/dev/video0").exists() {
        devices.push("/dev/video0".to_string());
    }

    Ok(devices)
}

/// List available ALSA audio devices
pub fn list_audio_devices() -> Result<Vec<String>> {
    let mut devices = Vec::new();

    // Try arecord first
    if which::which("arecord").is_ok() {
        let output = Command::new("arecord")
            .arg("-l")
            .output()
            .context("Failed to execute arecord")?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("card") && line.contains("device") {
                    // Parse lines like: "card 1: Device [USB Audio Device], device 0: USB Audio [USB Audio]"
                    if let Some(card_str) = line.split("card").nth(1) {
                        if let Some(card_num) = card_str.split(':').next() {
                            if let Some(device_str) = line.split("device").nth(1) {
                                if let Some(device_num) = device_str.split(':').next() {
                                    let card = card_num.trim();
                                    let device = device_num.trim();
                                    devices.push(format!("hw:{},{}", card, device));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback: add default device
    if devices.is_empty() {
        devices.push("hw:0,0".to_string());
    }

    Ok(devices)
}

/// Capture video and audio to file
pub fn capture(output: &Path, opts: &CaptureOptions) -> Result<()> {
    ffmpeg::check_ffmpeg()?;

    let mut args = vec![
        "-f".to_string(),
        "v4l2".to_string(),
        "-thread_queue_size".to_string(),
        "512".to_string(),
    ];

    // Video input settings
    if let Some(size) = opts.width.zip(opts.height) {
        args.push("-video_size".to_string());
        args.push(format!("{}x{}", size.0, size.1));
    }
    if let Some(fps) = opts.fps {
        args.push("-framerate".to_string());
        args.push(fps.to_string());
    }

    args.push("-i".to_string());
    args.push(opts.video_device.clone());

    // Audio input settings
    args.push("-f".to_string());
    args.push("alsa".to_string());
    args.push("-thread_queue_size".to_string());
    args.push("512".to_string());
    args.push("-i".to_string());
    args.push(opts.audio_device.clone());

    // Build video filters
    let mut vfilters = Vec::new();
    if opts.deinterlace {
        vfilters.push("bwdif".to_string());
    }
    if opts.stabilize {
        vfilters.push("deshake".to_string());
    }
    if let Some(ref denoise) = opts.denoise {
        if denoise == "hqdn3d" {
            vfilters.push("hqdn3d=4:3:6:4.5".to_string());
        } else if denoise == "nlmeans" {
            vfilters.push("nlmeans=s=3.0".to_string());
        }
    }

    if !vfilters.is_empty() {
        args.push("-vf".to_string());
        args.push(vfilters.join(","));
    }

    // Video codec settings
    if opts.archival_mode {
        // Archival mode: high quality, intra-only or near-lossless
        args.push("-c:v".to_string());
        args.push("libx264".to_string());
        args.push("-preset".to_string());
        args.push("ultrafast".to_string());
        args.push("-crf".to_string());
        args.push("18".to_string());
        args.push("-tune".to_string());
        args.push("grain".to_string());
        args.push("-g".to_string());
        args.push("1".to_string()); // Intra-only
    } else {
        args.push("-c:v".to_string());
        args.push("libx264".to_string());
        args.push("-preset".to_string());
        args.push("medium".to_string());

        if let Some(ref bitrate) = opts.video_bitrate {
            args.push("-b:v".to_string());
            args.push(bitrate.clone());
        } else if let Some(crf) = opts.crf {
            args.push("-crf".to_string());
            args.push(crf.to_string());
        }
    }

    // Audio codec settings
    if opts.archival_mode {
        // Archival mode: lossless PCM
        match opts.format {
            CaptureFormat::Mkv => {
                args.push("-c:a".to_string());
                args.push("pcm_s16le".to_string());
            }
            CaptureFormat::Mp4 => {
                // MP4 doesn't support PCM well, use AAC at high bitrate
                args.push("-c:a".to_string());
                args.push("aac".to_string());
                args.push("-b:a".to_string());
                args.push("320k".to_string());
            }
        }
    } else {
        args.push("-c:a".to_string());
        args.push("aac".to_string());
        args.push("-b:a".to_string());
        args.push(opts.audio_bitrate.clone());
    }

    // Output format
    match opts.format {
        CaptureFormat::Mp4 => {
            args.push("-f".to_string());
            args.push("mp4".to_string());
        }
        CaptureFormat::Mkv => {
            args.push("-f".to_string());
            args.push("matroska".to_string());
        }
    }

    args.push("-y".to_string());
    args.push(output.to_str().unwrap().to_string());

    // Execute capture
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    ffmpeg::execute_ffmpeg(&args_str)?;

    Ok(())
}
