use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;

use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "Nova Audio/Video Converter")]
#[command(version = "0.1.0")]
#[command(about = "A modern, high-performance audio and video converter", long_about = None)]
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
            convert_files(input, format, output.as_ref(), *recursive, quality, codec.as_ref(), *jobs)?;
        }
        Commands::Clean {
            input,
            metadata,
            optimize,
            recursive,
        } => {
            clean_files(input, *metadata, *optimize, *recursive)?;
        }
        Commands::Info { input } => {
            show_info(input)?;
        }
        Commands::Formats => {
            list_formats();
        }
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", "╔══════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║   Nova Audio/Video Converter v0.1.0     ║".bright_cyan());
    println!("{}", "║   High-Performance Media Converter       ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════╝".bright_cyan());
    println!();
}

fn convert_files(
    input: &Path,
    format: &str,
    output_dir: Option<&PathBuf>,
    recursive: bool,
    quality: &str,
    codec: Option<&String>,
    jobs: usize,
) -> Result<()> {
    check_ffmpeg()?;
    
    let output_dir = output_dir.map(|p| p.as_path()).unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(output_dir)?;
    
    let files = collect_files(input, recursive)?;
    
    if files.is_empty() {
        println!("{}", "No media files found!".yellow());
        return Ok(());
    }
    
    println!("{} Found {} file(s) to convert", "✓".green(), files.len());
    println!();
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .context("Failed to create thread pool")?;
    
    pool.install(|| {
        files.par_iter().for_each(|file| {
            match convert_file(file, format, output_dir, quality, codec) {
                Ok(_) => {
                    println!("{} Converted: {}", "✓".green(), file.display());
                }
                Err(e) => {
                    eprintln!("{} Failed to convert {}: {}", "✗".red(), file.display(), e);
                }
            }
        });
    });
    
    println!();
    println!("{} Conversion completed!", "✓".green());
    
    Ok(())
}

fn convert_file(
    input: &Path,
    format: &str,
    output_dir: &Path,
    quality: &str,
    codec: Option<&String>,
) -> Result<()> {
    let file_stem = input.file_stem().context("Invalid filename")?;
    let output_file = output_dir.join(format!("{}.{}", file_stem.to_string_lossy(), format));
    
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i")
        .arg(input)
        .arg("-y")
        .arg("-loglevel")
        .arg("error");
    
    if is_audio_format(format) {
        cmd.arg("-b:a").arg(quality);
    } else {
        if let Some(c) = codec {
            cmd.arg("-c:v").arg(c);
        }
        cmd.arg("-b:a").arg(quality);
    }
    
    cmd.arg(&output_file)
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    
    let output = cmd.output().context("Failed to execute ffmpeg")?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("FFmpeg error: {}", error);
    }
    
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
            cmd.arg("-c:v").arg("copy")
                .arg("-c:a").arg("copy");
        }
        
        cmd.arg(&output_file)
            .stdout(Stdio::null())
            .stderr(Stdio::piped());
        
        match cmd.output() {
            Ok(output) if output.status.success() => {
                println!("{} Cleaned: {}", "✓".green(), file.display());
                std::fs::remove_file(file)?;
                std::fs::rename(&output_file, file)?;
            }
            Ok(output) => {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("{} Failed to clean {}: {}", "✗".red(), file.display(), error);
            }
            Err(e) => {
                eprintln!("{} Failed to clean {}: {}", "✗".red(), file.display(), e);
            }
        }
    }
    
    println!();
    println!("{} Cleaning completed!", "✓".green());
    
    Ok(())
}

fn show_info(input: &Path) -> Result<()> {
    check_ffmpeg()?;
    
    println!("{} File: {}", "ℹ".bright_blue(), input.display());
    println!();
    
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
            let codec_type = stream.get("codec_type").and_then(|c| c.as_str()).unwrap_or("unknown");
            let codec_name = stream.get("codec_name").and_then(|c| c.as_str()).unwrap_or("unknown");
            println!("  {} Stream #{}: {} ({})", "→".bright_blue(), i, codec_type, codec_name);
            
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
            "mp3" | "mp4" | "avi" | "mkv" | "flac" | "wav" | "ogg" | "webm" | 
            "mov" | "flv" | "wmv" | "m4a" | "aac" | "wma" | "m4v" | "3gp"
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
    Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("FFmpeg is not installed. Please install FFmpeg to use this tool.")?;
    
    Ok(())
}
