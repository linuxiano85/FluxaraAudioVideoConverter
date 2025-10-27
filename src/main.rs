use anyhow::{Context, Result};
use colored::*;
use clap::Parser; // Importa Parser

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod ai;
mod audio;
mod capture;
mod ffmpeg;
mod video;
mod gui;
mod i18n;
mod cli; // Nuovo modulo per la CLI
mod converter; // Nuovo modulo per la logica di conversione
mod cleaner; // Nuovo modulo per la pulizia dei file
mod info_cmd; // Nuovo modulo per le informazioni sui file
mod formats_cmd; // Nuovo modulo per l'elenco dei formati
mod file_utils; // Nuovo modulo per le utilità sui file

use cli::{Cli, Commands}; // Importa Cli e Commands dal nuovo modulo
use converter::convert_files; // Importa convert_files dal nuovo modulo
use cleaner::clean_files; // Importa clean_files dal nuovo modulo
use info_cmd::show_info; // Importa show_info dal nuovo modulo
use formats_cmd::list_formats; // Importa list_formats dal nuovo modulo
use file_utils::{collect_files, is_media_file, is_audio_format}; // Importa le utilità sui file

fn main() -> Result<()> {
    print_banner();
    check_ffmpeg()?; // Ensure FFmpeg is available

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
                    *recursive, // Dereferenzia il valore booleano
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
            gate_threshold,
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
                    gate_threshold: *gate_threshold,
                };
                println!("{} Miglioramento audio in corso...", "✓".green());
                if *audio_only {
                    audio::enhance_audio_only(input, output, &opts)
                } else {
                    audio::enhance_audio(input, output, &opts)
                }
            })?;
            println!("{} Miglioramento audio completato!", "✓".green());
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
                println!("{} Miglioramento video in corso...", "✓".green());
                video::enhance_video(input, output, &opts)
            })?;
            println!("{} Miglioramento video completato!", "✓".green());
        }
        Commands::VhsRescue {
            input,
            output,
            notch,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                println!("{} Avvio VHS Rescue...", "🎬".bright_cyan());
                video::vhs_rescue(input, output, *notch)
            })?;
            println!("{} VHS Rescue completato!", "✓".green());
        }
        Commands::CaptureList => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                println!("{} Dispositivi video V4L2 disponibili:", "📹".bright_cyan());
                match capture::list_video_devices() {
                    Ok(devices) => {
                        if devices.is_empty() {
                            println!("  {}", "Nessun dispositivo video trovato".yellow());
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
                println!("{} Dispositivi audio ALSA disponibili:", "🎤".bright_cyan());
                match capture::list_audio_devices() {
                    Ok(devices) => {
                        if devices.is_empty() {
                            println!("  {}", "Nessun dispositivo audio trovato".yellow());
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
                Ok::<(), anyhow::Error>(())
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
                    "{} Avvio cattura da {} e {}...",
                    "📹".bright_cyan(),
                    video_device,
                    audio_device
                );
                println!("{}", "Premi Ctrl+C per interrompere la registrazione".yellow());
                capture::capture(output, &opts)
            })?;
            println!("{} Cattura completata!", "✓".green());
        }
        Commands::Clean {
            input,
            metadata,
            optimize,
            recursive,
        } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                clean_files(input, *metadata, *optimize, *recursive).await
            })?;
        }
        Commands::Info { input } => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                show_info(input).await
            })?;
        }
        Commands::Formats => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let formats = list_formats();
                println!("{}", "Formati audio/video supportati:".bright_yellow());
                for format in formats {
                    println!("  • {}", format.green());
                }
                Ok::<(), anyhow::Error>(())
            })?;
        }
        Commands::Gui => {
            println!("{} Avvio GUI...", "🚀".bright_magenta());
            gui::run().context("Impossibile avviare la GUI")?;
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

fn check_ffmpeg() -> Result<()> {
    ffmpeg::check_ffmpeg()
}
