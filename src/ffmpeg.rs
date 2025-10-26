#![allow(dead_code)]

use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::process::Command as TokioCommand;

/// Check if FFmpeg is available in the system
pub fn check_ffmpeg() -> Result<()> {
    which::which("ffmpeg")
        .context("FFmpeg is not installed. Please install FFmpeg to use this tool.")?;
    Ok(())
}

/// Check if FFprobe is available in the system
pub fn check_ffprobe() -> Result<()> {
    which::which("ffprobe")
        .context("FFprobe is not installed. Please install FFmpeg to use this tool.")?;
    Ok(())
}

/// Execute FFmpeg command synchronously
pub fn execute_ffmpeg(args: &[&str]) -> Result<()> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

    let output = cmd.output().context("Failed to execute ffmpeg")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("FFmpeg error: {}", error);
    }

    Ok(())
}

/// Execute FFmpeg command asynchronously
pub async fn execute_ffmpeg_async(args: Vec<String>) -> Result<()> {
    let mut cmd = TokioCommand::new("ffmpeg");
    cmd.args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd.output().await.context("Failed to execute ffmpeg")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("FFmpeg error: {}", error);
    }

    Ok(())
}

/// Get media file info using ffprobe
pub fn get_media_info(input: &Path) -> Result<serde_json::Value> {
    check_ffprobe()?;

    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(input)
        .output()
        .context("Failed to execute ffprobe")?;

    if !output.status.success() {
        anyhow::bail!("Failed to get file information");
    }

    let info: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    Ok(info)
}

/// Build a filtergraph string from multiple filters
pub fn build_filtergraph(filters: &[&str]) -> String {
    filters.join(",")
}

/// Build a complex filtergraph with multiple inputs/outputs
pub fn build_complex_filtergraph(
    video_filters: &[&str],
    audio_filters: &[&str],
) -> (String, String) {
    let vf = if video_filters.is_empty() {
        String::new()
    } else {
        video_filters.join(",")
    };

    let af = if audio_filters.is_empty() {
        String::new()
    } else {
        audio_filters.join(",")
    };

    (vf, af)
}

/// Probe device capabilities using v4l2-ctl
pub fn probe_device_caps(device: &str) -> Result<DeviceCaps> {
    let output = Command::new("v4l2-ctl")
        .arg("-d")
        .arg(device)
        .arg("--list-formats-ext")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            parse_v4l2_output(&stdout)
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            anyhow::bail!("v4l2-ctl failed: {} (device: {})", stderr.trim(), device)
        }
        Err(e) => {
            eprintln!("v4l2-ctl not available: {}. Install v4l-utils for better detection.", e);
            Ok(DeviceCaps { width: 640, height: 480, fps: 30, formats: vec!["yuyv422".into()] })
        }
    }
}

/// Parse the output of v4l2-ctl --list-formats-ext
fn parse_v4l2_output(output: &str) -> Result<DeviceCaps> {
    let mut formats = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut fps_num: Option<f32> = None;

    for line in output.lines() {
        if line.contains("Pixel Format") {
            if let Some(format) = line.split('\'').nth(1) {
                formats.push(format.to_string());
            }
        } else if line.contains("Size: Discrete") {
            if let Some(resolution) = line.split_whitespace().last() {
                let mut parts = resolution.split('x');
                if let (Some(w), Some(h)) = (parts.next(), parts.next()) {
                    width = w.parse().unwrap_or(0);
                    height = h.parse().unwrap_or(0);
                }
            }
        } else if line.contains("Interval: Discrete") {
            if let Some(fps_str) = line.split('(').nth(1) {
                if let Some(fps_val) = fps_str.split_whitespace().next() {
                    if let Ok(v) = fps_val.parse::<f32>() {
                        if v.is_finite() && v > 0.0 && v < 1000.0 {
                            fps_num = Some(v);
                        }
                    }
                }
            }
        }
    }

    let fps_u32 = fps_num.map(|v| v.round() as u32).unwrap_or(30);
    Ok(DeviceCaps {
        width,
        height,
        fps: fps_u32,
        formats,
    })
}

#[derive(Debug, Clone)]
pub struct DeviceCaps {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub formats: Vec<String>,
}