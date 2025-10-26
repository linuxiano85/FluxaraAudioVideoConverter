use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

mod ai;
mod audio;
mod capture;
mod ffmpeg;
mod video;
mod gui; // Add this line

#[derive(Parser)]
#[command(name = "Fluxara AVC")]
#[command(version = "0.1.0")]
#[command(about = "Fluxara AVC – Linux-first analog restoration & conversion with FFmpeg", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert audio/video files to different formats
    Convert {
        /// Input file or directory
        #[arg(short, long)]
        input: PathBuf,

        /// Output format (mp3, mp4, avi, mkv, flac, wav, ogg, webm, etc.)
        #[arg(short, long)]
        format: String,

        /// Output directory (default: current directory)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Process directories recursively
        #[arg(short, long)]
        recursive: bool,

        /// Audio quality (64k, 128k, 192k, 256k, 320k)
        #[arg(short = 'q', long, default_value = "192k")]
        quality: String,

        /// Video codec (libx264, libx265, libvpx, etc.)
        #[arg(short = 'c', long)]
        codec: Option<String>,

        /// Number of parallel jobs
        #[arg(short = 'j', long, default_value = "4")]
        jobs: usize,
    },
    /// Enhance audio with denoise, normalization, and compression
    EnhanceAudio {
        /// Input file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Enable denoising (afftdn)
        #[arg(long, default_value = "true")]
        denoise: bool,

        /// Enable loudness normalization
        #[arg(long, default_value = "true")]
        normalize: bool,

        /// High-pass filter frequency (Hz)
        #[arg(long, default_value = "80")]
        highpass: u32,

        /// Low-pass filter frequency (Hz, optional)
        #[arg(long)]
        lowpass: Option<u32>,

        /// Notch filter for hum removal (50 or 60 Hz)
        #[arg(long)]
        notch: Option<u32>,

        /// Enable compressor
        #[arg(long, default_value = "true")]
        compressor: bool,

        /// Enable noise gate
        #[arg(long, default_value = "true")]
        gate: bool,

        /// Process audio only (no video stream)
        #[arg(long)]
        audio_only: bool,
    },
    /// Enhance video with deinterlace, stabilization, denoise, and sharpening
    EnhanceVideo {
        /// Input file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Enable deinterlacing (bwdif)
        #[arg(long, default_value = "true")]
        deinterlace: bool,

        /// Enable stabilization (deshake)
        #[arg(long)]
        stabilize: bool,

        /// Denoise type: none, hqdn3d, nlmeans
        #[arg(long, default_value = "hqdn3d")]
        denoise: String,

        /// Enable sharpening
        #[arg(long, default_value = "true")]
        sharpen: bool,

        /// Enable color adjustment
        #[arg(long, default_value = "true")]
        color: bool,

        /// Scale width
        #[arg(long)]
        width: Option<u32>,

        /// Scale height
        #[arg(long)]
        height: Option<u32>,

        /// Display aspect ratio (e.g., 4:3, 16:9)
        #[arg(long)]
        aspect: Option<String>,
    },
    /// VHS Rescue: One-click preset for analog capture cleanup
    VhsRescue {
        /// Input file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Notch filter for hum removal (50 or 60 Hz)
        #[arg(long)]
        notch: Option<u32>,
    },
    /// List available V4L2 video and ALSA audio capture devices
    CaptureList,
    /// Capture video and audio from V4L2/ALSA devices
    Capture {
        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Video device (e.g., /dev/video0)
        #[arg(long, default_value = "/dev/video0")]
        video_device: String,

        /// Audio device (e.g., hw:1,0)
        #[arg(long, default_value = "hw:1,0")]
        audio_device: String,

        /// Output format: mp4 or mkv
        #[arg(long, default_value = "mp4")]
        format: String,

        /// Enable deinterlacing
        #[arg(long, default_value = "true")]
        deinterlace: bool,

        /// Enable stabilization
        #[arg(long)]
        stabilize: bool,

        /// Denoise type: none, hqdn3d, nlmeans
        #[arg(long)]
        denoise: Option<String>,

        /// Video bitrate (e.g., 5M)
        #[arg(long)]
        vbitrate: Option<String>,

        /// CRF value (18-28, lower = better quality)
        #[arg(long)]
        crf: Option<u32>,

        /// Video width
        #[arg(long)]
        width: Option<u32>,

        /// Video height
        #[arg(long)]
        height: Option<u32>,

        /// Frame rate
        #[arg(long)]
        fps: Option<u32>,

        /// Audio bitrate (e.g., 192k)
        #[arg(long, default_value = "192k")]
        abitrate: String,

        /// Archival mode (lossless/near-lossless)
        #[arg(long)]
        archival: bool,
    },
    /// Clean and optimize media files
    Clean {
        /// Input file or directory
        #[arg(short, long)]
        input: PathBuf,

        /// Remove metadata
        #[arg(short, long)]
        metadata: bool,

        /// Optimize file size
        #[arg(short = 'o', long)]
        optimize: bool,

        /// Process recursively
        #[arg(short, long)]
        recursive: bool,
    },
    /// Get information about media files
    Info {
        /// Input file
        #[arg(short, long)]
        input: PathBuf,
    },
    /// List supported formats
    Formats,
    /// Launch the graphical user interface
    Gui,
}

fn main() -> Result<()> {
    print_banner();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert {
            input,
            format,
            output,
            recursive,
            quality,
            codec,
            jobs,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                convert_files(
                    input,
                    format,
                    output.as_ref(),
                    *recursive,
                    quality,
                    codec.as_ref(),
                    *jobs,
                )
                .await
            })?;
        }
        Commands::EnhanceAudio {
            input,
            output,
            denoise,
            normalize,
            highpass,
            lowpass,
            notch,
            audio_only,
            compressor,
            gate,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let opts = audio::AudioEnhanceOptions {
                    denoise: *denoise,
                    normalize: *normalize,
                    highpass_freq: Some(*highpass),
                    lowpass_freq: *lowpass,
                    notch_freq: *notch,
                    compressor: *compressor,
                    gate: *gate,
                    gate_threshold: -50.0,
                };
                println!("{} Enhancing audio...", "✓".green());
                if *audio_only {
                    audio::enhance_audio_only(input, output, &opts)
                } else {
                    audio::enhance_audio(input, output, &opts)
                }
            })?;
            println!("{} Audio enhancement completed!", "✓".green());
        }
        Commands::EnhanceVideo {
            input,
            output,
            deinterlace,
            stabilize,
            denoise,
            sharpen,
            color,
            width,
            height,
            aspect,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let denoise_type = match denoise.as_str() {
                    "none" => video::DenoiseType::None,
                    "nlmeans" => video::DenoiseType::Nlmeans,
                    _ => video::DenoiseType::Hqdn3d,
                };
                let opts = video::VideoEnhanceOptions {
                    deinterlace: *deinterlace,
                    stabilize: *stabilize,
                    denoise: denoise_type,
                    sharpen: *sharpen,
                    color_adjust: *color,
                    scale_width: *width,
                    scale_height: *height,
                    aspect_ratio: aspect.clone(),
                };
                println!("{} Enhancing video...", "✓".green());
                video::enhance_video(input, output, &opts)
            })?;
            println!("{} Video enhancement completed!", "✓".green());
        }
        Commands::VhsRescue {
            input,
            output,
            notch,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                println!("{} Starting VHS Rescue...", "🎬".bright_cyan());
                video::vhs_rescue(input, output, *notch)
            })?;
            println!("{} VHS Rescue completed!", "✓".green());
        }
        Commands::CaptureList => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                println!("{} Available V4L2 Video Devices:", "📹".bright_cyan());
                match capture::list_video_devices() {
                    Ok(devices) => {
                        if devices.is_empty() {
                            println!("  {}", "No video devices found".yellow());
                        } else {
                            for dev in devices {
                                println!("  • {}", dev.green());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("  {} {}", "✗".red(), e);
                    }
                }
                println!();
                println!("{} Available ALSA Audio Devices:", "🎤".bright_cyan());
                match capture::list_audio_devices() {
                    Ok(devices) => {
                        if devices.is_empty() {
                            println!("  {}", "No audio devices found".yellow());
                        } else {
                            for dev in devices {
                                println!("  • {}", dev.green());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("  {} {}", "✗".red(), e);
                    }
                }
                Ok(())
            })?;
        }
        Commands::Capture {
            output,
            video_device,
            audio_device,
            format,
            deinterlace,
            stabilize,
            denoise,
            vbitrate,
            crf,
            width,
            height,
            fps,
            abitrate,
            archival,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let capture_format = match format.as_str() {
                    "mkv" => capture::CaptureFormat::Mkv,
                    _ => capture::CaptureFormat::Mp4,
                };
                let opts = capture::CaptureOptions {
                    format: capture_format,
                    video_device: video_device.clone(),
                    audio_device: audio_device.clone(),
                    deinterlace: *deinterlace,
                    stabilize: *stabilize,
                    denoise: denoise.clone(),
                    video_bitrate: vbitrate.clone(),
                    crf: *crf,
                    width: *width,
                    height: *height,
                    fps: *fps,
                    audio_bitrate: abitrate.clone(),
                    archival_mode: *archival,
                };
                println!(
                    "{} Starting capture from {} and {}...",
                    "📹".bright_cyan(),
                    video_device,
                    audio_device
                );
                println!("{}", "Press Ctrl+C to stop recording".yellow());
                capture::capture(output, &opts)
            })?;
            println!("{} Capture completed!", "✓".green());
        }
        Commands::Clean {
            input,
            metadata,
            optimize,
            recursive,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                clean_files(input, *metadata, *optimize, *recursive)
            })?;
        }
        Commands::Info { input } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                show_info(input)
            })?;
        }
        Commands::Formats => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                list_formats();
                Ok(())
            })?;
        }
        Commands::Gui => {
            println!("{} Launching GUI...", "🚀".bright_magenta());
            gui::run().context("Failed to launch GUI")?;
        }
    }

    Ok(())
}

fn print_banner() {
    println!("{}", "╔══════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║        Fluxara AVC v0.1.0               ║".bright_cyan());
    println!("{}", "║   Linux-first Analog Restoration        ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════╝".bright_cyan());
    println!();
}

use indicatif::{ProgressBar, ProgressStyle};
use futures::stream::{self, StreamExt};

pub async fn convert_files(
    input: &Path,
    format: &str,
    output_dir: Option<&PathBuf>,
    recursive: bool,
    quality: &str,
    codec: Option<&String>,
    jobs: usize,
) -> Result<()> {
    check_ffmpeg()?;

    let output_dir = output_dir
        .map(|p| p.as_path())
        .unwrap_or_else(|| Path::new("."));
    tokio::fs::create_dir_all(output_dir)
        .await
        .context("Failed to create output directory")?;

    let files = collect_files(input, recursive)?;

    if files.is_empty() {
        println!("{}", "No media files found!".yellow());
        return Ok(());
    }

    println!("{} Found {} file(s) to convert", "✓".green(), files.len());
    println!();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
            )?
            .progress_chars("#>-"),
    );

    stream::iter(files)
        .map(|file| {
            let output_dir_clone = output_dir.to_path_buf();
            let format_clone = format.to_string();
            let quality_clone = quality.to_string();
            let codec_clone = codec.map(|s| s.to_string());
            let pb_clone = pb.clone();

            tokio::spawn(async move {
                let result = convert_file(
                    &file,
                    &format_clone,
                    &output_dir_clone,
                    &quality_clone,
                    codec_clone.as_ref(),
                )
                .await;
                pb_clone.inc(1);
                match result {
                    Ok(_) => {
                        pb_clone.println(format!("{} Converted: {}", "✓".green(), file.display()));
                        Ok(())
                    }
                    Err(e) => {
                        pb_clone.println(format!(
                            "{} Failed to convert {}: {}",
                            "✗".red(),
                            file.display(),
                            e
                        ));
                        Err(e)
                    }
                }
            })
        })
        .buffer_unordered(jobs)
        .collect::<Vec<_>>()
        .await;

    pb.finish_with_message("Conversion completed!");

    println!();
    println!("{} Conversion completed!", "✓".green());

    Ok(())
}

async fn convert_file(
    input: &Path,
    format: &str,
    output_dir: &Path,
    quality: &str,
    codec: Option<&String>,
) -> Result<()> {
    let file_stem = input.file_stem().context("Invalid filename")?;
    let output_file = output_dir.join(format!("{}.{}", file_stem.to_string_lossy(), format));

    let mut args = vec![
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-y".to_string(),
        "-loglevel".to_string(),
        "error".to_string(),
    ];

    if is_audio_format(format) {
        args.push("-b:a".to_string());
        args.push(quality.to_string());
    } else {
        if let Some(c) = codec {
            args.push("-c:v".to_string());
            args.push(c.to_string());
        }
        args.push("-b:a".to_string());
        args.push(quality.to_string());
    }

    args.push(output_file.to_string_lossy().to_string());

    ffmpeg::execute_ffmpeg_async(args)
        .await
        .context("Failed to execute ffmpeg asynchronously")?;

    Ok(())
}

fn clean_files(input: &Path, remove_metadata: bool, optimize: bool, recursive: bool) -> Result<()> {
    check_ffmpeg()?;

    let files = collect_files(input, recursive)?;

    if files.is_empty() {
        println!("{}", "No media files found!".yellow());
        return Ok(());
    }

    println!("{} Found {} file(s) to clean", "✓".green(), files.len());
    println!();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
            )?
            .progress_chars("#>-"),
    );

    for file in &files {
        let output_file = file.with_extension(format!(
            "cleaned.{}",
            file.extension().unwrap_or_default().to_string_lossy()
        ));

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-i")
            .arg(file)
            .arg("-y")
            .arg("-loglevel")
            .arg("error");

        if remove_metadata {
            cmd.arg("-map_metadata").arg("-1");
        }

        if optimize {
            cmd.arg("-c:v").arg("copy").arg("-c:a").arg("copy");
        }

        cmd.arg(&output_file)
            .stdout(Stdio::null())
            .stderr(Stdio::piped());

        match cmd.output() {
            Ok(output) if output.status.success() => {
                pb.println(format!("{} Cleaned: {}", "✓".green(), file.display()));
                std::fs::remove_file(file)?;
                std::fs::rename(&output_file, file)?;
            }
            Ok(output) => {
                let error = String::from_utf8_lossy(&output.stderr);
                pb.println(format!(
                    "{} Failed to clean {}: {}",
                    "✗".red(),
                    file.display(),
                    error
                ));
            }
            Err(e) => {
                pb.println(format!("{} Failed to clean {}: {}", "✗".red(), file.display(), e));
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("Cleaning completed!");

    println!();
    println!("{} Cleaning completed!", "✓".green());

    Ok(())
}

fn show_info(input: &Path) -> Result<()> {
    println!("{} File: {}", "ℹ".bright_blue(), input.display());
    println!();

    let info = ffmpeg::get_media_info(input)?;

    if let Some(format) = info.get("format") {
        if let Some(duration) = format.get("duration").and_then(|d| d.as_str()) {
            let duration: f64 = duration.parse().unwrap_or(0.0);
            println!("{} Duration: {:.2} seconds", "•".bright_blue(), duration);
        }
        if let Some(size) = format.get("size").and_then(|s| s.as_str()) {
            let size: u64 = size.parse().unwrap_or(0);
            println!("{} Size: {} MB", "•".bright_blue(), size / 1_000_000);
        }
        if let Some(bit_rate) = format.get("bit_rate").and_then(|b| b.as_str()) {
            let bit_rate: u64 = bit_rate.parse().unwrap_or(0);
            println!("{} Bit rate: {} kbps", "•".bright_blue(), bit_rate / 1000);
        }
    }

    if let Some(streams) = info.get("streams").and_then(|s| s.as_array()) {
        println!();
        println!("{} Streams:", "•".bright_blue());
        for (i, stream) in streams.iter().enumerate() {
            let codec_type = stream
                .get("codec_type")
                .and_then(|c| c.as_str())
                .unwrap_or("unknown");
            let codec_name = stream
                .get("codec_name")
                .and_then(|c| c.as_str())
                .unwrap_or("unknown");
            println!(
                "  {} Stream #{}: {} ({})",
                "→".bright_blue(),
                i,
                codec_type,
                codec_name
            );

            if codec_type == "video" {
                if let (Some(width), Some(height)) = (
                    stream.get("width").and_then(|w| w.as_i64()),
                    stream.get("height").and_then(|h| h.as_i64()),
                ) {
                    println!("    Resolution: {}x{}", width, height);
                }
            }
        }
    }

    Ok(())
}

fn list_formats() {
    println!("{}", "Supported Audio Formats:".bright_yellow());
    println!("  • MP3  - MPEG Audio Layer 3");
    println!("  • AAC  - Advanced Audio Coding");
    println!("  • FLAC - Free Lossless Audio Codec");
    println!("  • WAV  - Waveform Audio File Format");
    println!("  • OGG  - Ogg Vorbis");
    println!("  • M4A  - MPEG-4 Audio");
    println!("  • WMA  - Windows Media Audio");
    println!();
    println!("{}", "Supported Video Formats:".bright_yellow());
    println!("  • MP4  - MPEG-4 Part 14");
    println!("  • AVI  - Audio Video Interleave");
    println!("  • MKV  - Matroska Video");
    println!("  • WEBM - WebM Video");
    println!("  • MOV  - QuickTime Movie");
    println!("  • FLV  - Flash Video");
    println!("  • WMV  - Windows Media Video");
    println!();
}

fn collect_files(input: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if input.is_file() {
        files.push(input.to_path_buf());
    } else if input.is_dir() {
        if recursive {
            for entry in WalkDir::new(input).follow_links(true) {
                let entry = entry?;
                if entry.file_type().is_file() && is_media_file(entry.path()) {
                    files.push(entry.path().to_path_buf());
                }
            }
        } else {
            for entry in std::fs::read_dir(input)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && is_media_file(&path) {
                    files.push(path);
                }
            }
        }
    }

    Ok(files)
}

fn is_media_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "mp3"
                | "mp4"
                | "avi"
                | "mkv"
                | "flac"
                | "wav"
                | "ogg"
                | "webm"
                | "mov"
                | "flv"
                | "wmv"
                | "m4a"
                | "aac"
                | "wma"
                | "m4v"
                | "3gp"
        )
    } else {
        false
    }
}

fn is_audio_format(format: &str) -> bool {
    matches!(
        format.to_lowercase().as_str(),
        "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "wma"
    )
}

fn check_ffmpeg() -> Result<()> {
    ffmpeg::check_ffmpeg()
}
