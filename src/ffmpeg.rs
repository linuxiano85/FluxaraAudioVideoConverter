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

/// Convert media files
pub fn convert_media(
    input: &Path,
    output: &Path,
    format: &str,
    quality: &str,
    codec: &str,
    recursive: bool,
) -> Result<()> {
    check_ffmpeg()?;

    let mut args = vec![
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-c:v".to_string(),
        codec.to_string(),
        "-b:v".to_string(),
        quality.to_string(),
        "-f".to_string(),
        format.to_string(),
        output.to_string_lossy().to_string(),
    ];

    // Add recursive processing if needed (this part might need more complex logic for directory traversal)
    if recursive {
        // For simplicity, this example assumes input is a single file.
        // Real recursive conversion would involve iterating directories and calling convert_media for each file.
        eprintln!("Warning: Recursive conversion not fully implemented in this example.");
    }

    execute_ffmpeg(&args.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;
    Ok(())
}

/// Build a filtergraph string from multiple filters
#[allow(dead_code)]
pub fn build_filtergraph(filters: &[&str]) -> String {
    filters.join(",")
}

/// Build a complex filtergraph with multiple inputs/outputs
#[allow(dead_code)]
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeviceCaps {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub formats: Vec<String>,
}

/// Probe device capabilities (stub for now - would need v4l2-ctl integration)
#[allow(dead_code)]
pub fn probe_device_caps(_device: &str) -> Result<DeviceCaps> {
    // For now, return default capabilities
    // In a full implementation, this would call v4l2-ctl or parse ffmpeg output
    Ok(DeviceCaps {
        width,
        height,
        fps: fps_u32,
        formats,
    })
}
