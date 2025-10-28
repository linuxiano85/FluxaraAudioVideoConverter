//! Modulo per la pulizia e l'ottimizzazione dei file multimediali.

use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::ffmpeg;
use crate::file_utils::collect_files; // Importa le utilità sui file

/// Pulisce e ottimizza uno o più file multimediali.
/// Supporta la rimozione dei metadati e l'ottimizzazione della dimensione del file.
pub async fn clean_files(input: &Path, remove_metadata: bool, optimize: bool, recursive: bool) -> Result<()> {
    check_ffmpeg()?;

    let files = collect_files(input, recursive)?;

    if files.is_empty() {
        println!("{}", "Nessun file multimediale trovato!".yellow());
        return Ok(());
    }

    println!("{} Trovato {} file(s) da pulire", "✓".green(), files.len());
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
                pb.println(format!("{} Pulito: {}", "✓".green(), file.display()));
                std::fs::remove_file(file)?;
                std::fs::rename(&output_file, file)?;
            }
            Ok(output) => {
                let error = String::from_utf8_lossy(&output.stderr);
                pb.println(format!(
                    "{} Errore durante la pulizia di {}: {}",
                    "✗".red(),
                    file.display(),
                    error
                ));
            }
            Err(e) => {
                pb.println(format!("{} Errore durante la pulizia di {}: {}", "✗".red(), file.display(), e));
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("Pulizia completata!");

    println!();
    println!("{} Pulizia completata!", "✓".green());

    Ok(())
}

/// Controlla se FFmpeg è installato e accessibile.
fn check_ffmpeg() -> Result<()> {
    ffmpeg::check_ffmpeg()
}
