#![allow(dead_code)]

use crate::ffmpeg;
use anyhow::Result;
use std::path::Path;

/// Audio enhancement options
#[derive(Debug, Clone)]
pub struct AudioEnhanceOptions {
    pub denoise: bool,
    pub normalize: bool,
    pub highpass_freq: Option<u32>,
    pub lowpass_freq: Option<u32>,
    pub notch_freq: Option<u32>, // 50 or 60 Hz hum removal
    pub compressor: bool,
    pub gate: bool,
    pub gate_threshold: f32,
}

impl Default for AudioEnhanceOptions {
    fn default() -> Self {
        Self {
            denoise: true,
            normalize: true,
            highpass_freq: Some(80), // Remove low rumble
            lowpass_freq: None,
            notch_freq: None, // User must specify 50 or 60
            compressor: true,
            gate: true,
            gate_threshold: -50.0,
        }
    }
}

/// Build audio filtergraph for enhancement
pub fn build_audio_filters(opts: &AudioEnhanceOptions) -> Vec<String> {
    let mut filters = Vec::new();

    // High-pass filter to remove rumble
    if let Some(freq) = opts.highpass_freq {
        filters.push(format!("highpass=f={}", freq));
    }

    // Low-pass filter if specified
    if let Some(freq) = opts.lowpass_freq {
        filters.push(format!("lowpass=f={}", freq));
    }

    // Notch filter for hum removal (50/60 Hz)
    if let Some(freq) = opts.notch_freq {
        filters.push(format!("equalizer=f={}:width_type=h:width=10:g=-20", freq));
    }

    // Denoise using afftdn (FFT denoiser)
    if opts.denoise {
        filters.push("afftdn=nf=-25".to_string());
    }

    // Gate to reduce background noise
    if opts.gate {
        filters.push(format!(
            "agate=threshold={}dB:ratio=3:attack=20:release=250",
            opts.gate_threshold
        ));
    }

    // Compressor for consistent levels (conservative settings)
    if opts.compressor {
        filters.push(
            "acompressor=threshold=-18dB:ratio=3:attack=20:release=250:makeup=2dB".to_string(),
        );
    }

    // Loudness normalization (EBU R128)
    if opts.normalize {
        filters.push("loudnorm=I=-16:TP=-1.5:LRA=11".to_string());
    }

    filters
}

/// Enhance audio in a file
pub fn enhance_audio(input: &Path, output: &Path, opts: &AudioEnhanceOptions) -> Result<()> {
    ffmpeg::check_ffmpeg()?;

    let filters = build_audio_filters(opts);
    let filter_str = filters.join(",");

    let args = vec![
        "-i",
        input.to_str().unwrap(),
        "-af",
        &filter_str,
        "-c:v",
        "copy", // Copy video stream if present
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

/// Enhance audio stream only (no video)
pub fn enhance_audio_only(input: &Path, output: &Path, opts: &AudioEnhanceOptions) -> Result<()> {
    ffmpeg::check_ffmpeg()?;

    let filters = build_audio_filters(opts);
    let filter_str = filters.join(",");

    let args = vec![
        "-i",
        input.to_str().unwrap(),
        "-af",
        &filter_str,
        "-c:a",
        "flac", // Use lossless for audio-only
        "-y",
        output.to_str().unwrap(),
    ];

    ffmpeg::execute_ffmpeg(&args)?;
    Ok(())
}
