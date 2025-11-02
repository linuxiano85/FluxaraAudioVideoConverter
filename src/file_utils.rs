//! Modulo per le utilità di gestione dei file, come la raccolta e la verifica del tipo di file.

use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Raccoglie un elenco di file multimediali da un dato percorso.
/// Se il percorso è una directory e `recursive` è true, cerca ricorsivamente.
pub fn collect_files(input: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
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

/// Verifica se un dato percorso punta a un file multimediale supportato.
pub fn is_media_file(path: &Path) -> bool {
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

/// Verifica se un dato formato è un formato audio.
pub fn is_audio_format(format: &str) -> bool {
    matches!(
        format.to_lowercase().as_str(),
        "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "wma"
    )
}
