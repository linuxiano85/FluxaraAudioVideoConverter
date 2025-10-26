use crate::ffmpeg;
use anyhow::Result;
use std::path::Path;

/// Video enhancement options
#[derive(Debug, Clone)]
pub struct VideoEnhanceOptions {
    pub deinterlace: bool,
    pub stabilize: bool,
    pub denoise: DenoiseType,
    pub sharpen: bool,
    pub color_adjust: bool,
    pub scale_width: Option<u32>,
    pub scale_height: Option<u32>,
    pub aspect_ratio: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DenoiseType {
    None,
    Hqdn3d,  // Fast, conservative
    Nlmeans, // High quality, slower
}

impl Default for VideoEnhanceOptions {
    fn default() -> Self {
        Self {
            deinterlace: true,
            stabilize: false, // Can be slow, off by default
            denoise: DenoiseType::Hqdn3d,
            sharpen: true,
            color_adjust: true,
            scale_width: None,
            scale_height: None,
            aspect_ratio: None,
        }
    }
}

/// Build video filtergraph for enhancement
pub fn build_video_filters(opts: &VideoEnhanceOptions) -> Vec<String> {
    let mut filters = Vec::new();

    // Deinterlace using bwdif (Bob Weaver Deinterlacing Filter)
    if opts.deinterlace {
        filters.push("bwdif=mode=send_field:parity=auto:deint=all".to_string());
    }

    // Video stabilization
    if opts.stabilize {
        // Note: vidstabtransform requires vidstabdetect first, which needs two passes
        // For simplicity, we'll use deshake which is single-pass
        filters.push("deshake".to_string());
    }

    // Denoise
    match opts.denoise {
        DenoiseType::Hqdn3d => {
            // Conservative denoising: luma, chroma, luma_temporal, chroma_temporal
            filters.push("hqdn3d=4:3:6:4.5".to_string());
        }
        DenoiseType::Nlmeans => {
            // High-quality but slower
            filters.push("nlmeans=s=3.0".to_string());
        }
        DenoiseType::None => {}
    }

    // Sharpen using unsharp
    if opts.sharpen {
        // Conservative sharpening: luma_amount, chroma_amount
        filters.push("unsharp=5:5:0.8:3:3:0.4".to_string());
    }

    // Color adjustments (EQ filter)
    if opts.color_adjust {
        // Slight brightness and saturation boost typical for VHS
        filters.push("eq=brightness=0.02:saturation=1.1".to_string());
    }

    // Scale if specified
    if let (Some(w), Some(h)) = (opts.scale_width, opts.scale_height) {
        filters.push(format!("scale={}:{}:flags=lanczos", w, h));
    }

    // Set display aspect ratio
    if let Some(ref dar) = opts.aspect_ratio {
        filters.push(format!("setdar={}", dar));
    }

    filters
}

/// Enhance video in a file
pub fn enhance_video(input: &Path, output: &Path, opts: &VideoEnhanceOptions) -> Result<()> {
    ffmpeg::check_ffmpeg()?;

    let filters = build_video_filters(opts);
    let filter_str = filters.join(",");

    let args = vec![
        "-i",
        input.to_str().unwrap(),
        "-vf",
        &filter_str,
        "-c:v",
        "libx264",
        "-preset",
        "medium",
        "-crf",
        "18",
        "-c:a",
        "copy", // Copy audio stream
        "-y",
        output.to_str().unwrap(),
    ];

    ffmpeg::execute_ffmpeg(&args)?;
    Ok(())
}

/// VHS rescue preset - combines video and audio enhancement
pub fn vhs_rescue(input: &Path, output: &Path, notch_freq: Option<u32>) -> Result<()> {
    ffmpeg::check_ffmpeg()?;

    // Video filters for VHS
    let video_opts = VideoEnhanceOptions {
        deinterlace: true,
        stabilize: true,
        denoise: DenoiseType::Hqdn3d,
        sharpen: true,
        color_adjust: true,
        scale_width: None,
        scale_height: None,
        aspect_ratio: Some("4:3".to_string()), // Typical VHS
    };

    // Audio filters for VHS
    let audio_opts = crate::audio::AudioEnhanceOptions {
        denoise: true,
        normalize: true,
        highpass_freq: Some(80),
        lowpass_freq: Some(15000), // Remove high-freq noise
        notch_freq,
        compressor: true,
        gate: true,
        gate_threshold: Some(-50.0),
    };

    let vf = build_video_filters(&video_opts).join(",");
    let af = crate::audio::build_audio_filters(&audio_opts).join(",");

    let args = vec![
        "-i",
        input.to_str().unwrap(),
        "-vf",
        &vf,
        "-af",
        &af,
        "-c:v",
        "libx264",
        "-preset",
        "slow", // Better quality for archival
        "-crf",
        "18",
        "-c:a",
        "aac",
        "-b:a",
        "192k",
        "-y",
        output.to_str().unwrap(),
    ];

    ffmpeg::execute_ffmpeg(&args)?;
    Ok(())
}
