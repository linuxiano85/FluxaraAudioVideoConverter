//! Modulo per la logica di conversione dei file audio/video.

use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use futures::stream::{self, StreamExt};
use std::path::{Path, PathBuf};

use crate::ffmpeg;
use crate::file_utils::{collect_files, is_audio_format}; // Importa le utilità sui file

/// Converte uno o più file multimediali nel formato specificato.
/// Supporta la conversione ricorsiva delle directory e la gestione parallela dei job.
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
        println!("{}", "Nessun file multimediale trovato!".yellow());
        return Ok(());
    }

    println!("{} Trovato {} file(s) da convertire", "✓".green(), files.len());
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
            let file_clone = file.clone(); // Clona PathBuf per la closure

            tokio::spawn(async move {
                let result = convert_file(
                    &file_clone, // Usa la copia
                    &format_clone,
                    &output_dir_clone,
                    &quality_clone,
                    codec_clone.as_ref(),
                )
                .await;
                pb_clone.inc(1);
                match result {
                    Ok(_) => {
                        pb_clone.println(format!("{} Convertito: {}", "✓".green(), file.display()));
                        Ok(())
                    }
                    Err(e) => {
                        pb_clone.println(format!(
                            "{} Errore durante la conversione di {}: {}",
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

    pb.finish_with_message("Conversione completata!");

    println!();
    println!("{} Conversione completata!", "✓".green());

    Ok(())
}

/// Converte un singolo file multimediale.
async fn convert_file(
    input: &Path,
    format: &str,
    output_dir: &Path,
    quality: &str,
    codec: Option<&String>,
) -> Result<()> {
    let file_stem = input.file_stem().context("Nome file non valido")?;
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
        .context("Impossibile eseguire ffmpeg in modo asincrono")?;

    Ok(())
}

/// Controlla se FFmpeg è installato e accessibile.
fn check_ffmpeg() -> Result<()> {
    ffmpeg::check_ffmpeg()
}
