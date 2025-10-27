//! Modulo per la visualizzazione delle informazioni sui file multimediali.

use anyhow::Result;
use colored::*;
use std::path::Path;

use crate::ffmpeg;

/// Mostra informazioni dettagliate su un file multimediale.
pub async fn show_info(input: &Path) -> Result<()> {
    println!("{} File: {}", "ℹ".bright_blue(), input.display());
    println!();

    let info = ffmpeg::get_media_info(input)?;

    if let Some(format) = info.get("format") {
        if let Some(duration) = format.get("duration").and_then(|d| d.as_str()) {
            let duration: f64 = duration.parse().unwrap_or(0.0);
            println!("{} Durata: {:.2} secondi", "•".bright_blue(), duration);
        }
        if let Some(size) = format.get("size").and_then(|s| s.as_str()) {
            let size: u64 = size.parse().unwrap_or(0);
            println!("{} Dimensione: {} MB", "•".bright_blue(), size / 1_000_000);
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
                    println!("    Risoluzione: {}x{}", width, height);
                }
            }
        }
    }

    Ok(())
}
