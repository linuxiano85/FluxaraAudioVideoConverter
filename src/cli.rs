//! Modulo per la definizione della struttura CLI e dei sottocomandi.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Struttura principale per la riga di comando.
/// Definisce il nome dell'applicazione, la versione e la descrizione.
#[derive(Parser)]
#[command(name = "Fluxara AVC")]
#[command(version = "0.1.0")]
#[command(about = "Fluxara AVC – Linux-first analog restoration & conversion with FFmpeg", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum che definisce i vari sottocomandi disponibili nell'applicazione.
#[derive(Subcommand)]
pub enum Commands {
    /// Converti file audio/video in diversi formati
    Convert {
        /// File o directory di input
        #[arg(short, long)]
        input: PathBuf,

        /// Formato di output (mp3, mp4, avi, mkv, flac, wav, ogg, webm, ecc.)
        #[arg(short, long)]
        format: String,

        /// Directory di output (default: directory corrente)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Elabora le directory in modo ricorsivo
        #[arg(short, long)]
        recursive: bool,

        /// Qualità audio (64k, 128k, 192k, 256k, 320k)
        #[arg(short = 'q', long, default_value = "192k")]
        quality: String,

        /// Codec video (libx264, libx265, libvpx, ecc.)
        #[arg(short = 'c', long)]
        codec: Option<String>,

        /// Numero di processi paralleli
        #[arg(short = 'j', long, default_value = "4")]
        jobs: usize,
    },
    /// Migliora l'audio con denoise, normalizzazione e compressione
    EnhanceAudio {
        /// File di input
        #[arg(short, long)]
        input: PathBuf,

        /// File di output
        #[arg(short, long)]
        output: PathBuf,

        /// Abilita il denoising (afftdn)
        #[arg(long, default_value = "true")]
        denoise: bool,

        /// Abilita la normalizzazione del volume
        #[arg(long, default_value = "true")]
        normalize: bool,

        /// Frequenza del filtro passa-alto (Hz)
        #[arg(long, default_value = "80")]
        highpass: u32,

        /// Frequenza del filtro passa-basso (Hz, opzionale)
        #[arg(long)]
        lowpass: Option<u32>,

        /// Filtro notch per la rimozione del ronzio (50 o 60 Hz)
        #[arg(long)]
        notch: Option<u32>,

        /// Abilita il compressore
        #[arg(long, default_value = "true")]
        compressor: bool,

        /// Abilita il noise gate
        #[arg(long, default_value = "true")]
        gate: bool,

        /// Soglia del noise gate (es. -60.0)
        #[arg(long)]
        gate_threshold: Option<f32>,

        /// Elabora solo l'audio (nessuno stream video)
        #[arg(long)]
        audio_only: bool,
    },
    /// Migliora il video con deinterlace, stabilizzazione, denoise e sharpening
    EnhanceVideo {
        /// File di input
        #[arg(short, long)]
        input: PathBuf,

        /// File di output
        #[arg(short, long)]
        output: PathBuf,

        /// Abilita il deinterlacing (bwdif)
        #[arg(long, default_value = "true")]
        deinterlace: bool,

        /// Abilita la stabilizzazione (deshake)
        #[arg(long)]
        stabilize: bool,

        /// Tipo di denoise: none, hqdn3d, nlmeans
        #[arg(long, default_value = "hqdn3d")]
        denoise: String,

        /// Abilita lo sharpening
        #[arg(long, default_value = "true")]
        sharpen: bool,

        /// Abilita la regolazione del colore
        #[arg(long, default_value = "true")]
        color: bool,

        /// Larghezza di scala
        #[arg(long)]
        width: Option<u32>,

        /// Altezza di scala
        #[arg(long)]
        height: Option<u32>,

        /// Rapporto d'aspetto del display (es. 4:3, 16:9)
        #[arg(long)]
        aspect: Option<String>,
    },
    /// VHS Rescue: preset one-click per la pulizia della cattura analogica
    VhsRescue {
        /// File di input
        #[arg(short, long)]
        input: PathBuf,

        /// File di output
        #[arg(short, long)]
        output: PathBuf,

        /// Filtro notch per la rimozione del ronzio (50 o 60 Hz)
        #[arg(long)]
        notch: Option<u32>,
    },
    /// Elenca i dispositivi di cattura video V4L2 e audio ALSA disponibili
    CaptureList,
    /// Cattura video e audio da dispositivi V4L2/ALSA
    Capture {
        /// File di output
        #[arg(short, long)]
        output: PathBuf,

        /// Dispositivo video (es. /dev/video0)
        #[arg(long, default_value = "/dev/video0")]
        video_device: String,

        /// Dispositivo audio (es. hw:1,0)
        #[arg(long, default_value = "hw:1,0")]
        audio_device: String,

        /// Formato di output: mp4 o mkv
        #[arg(long, default_value = "mp4")]
        format: String,

        /// Abilita il deinterlacing
        #[arg(long, default_value = "true")]
        deinterlace: bool,

        /// Abilita la stabilizzazione
        #[arg(long)]
        stabilize: bool,

        /// Tipo di denoise: none, hqdn3d, nlmeans
        #[arg(long)]
        denoise: Option<String>,

        /// Bitrate video (es. 5M)
        #[arg(long)]
        vbitrate: Option<String>,

        /// Valore CRF (18-28, inferiore = migliore qualità)
        #[arg(long)]
        crf: Option<u32>,

        /// Larghezza video
        #[arg(long)]
        width: Option<u32>,

        /// Altezza video
        #[arg(long)]
        height: Option<u32>,

        /// Frame rate
        #[arg(long)]
        fps: Option<u32>,

        /// Bitrate audio (es. 192k)
        #[arg(long, default_value = "192k")]
        abitrate: String,

        /// Modalità di archiviazione (lossless/quasi-lossless)
        #[arg(long)]
        archival: bool,
    },
    /// Pulisci e ottimizza i file multimediali
    Clean {
        /// File o directory di input
        #[arg(short, long)]
        input: PathBuf,

        /// Rimuovi metadati
        #[arg(short, long)]
        metadata: bool,

        /// Ottimizza la dimensione del file
        #[arg(short = 'o', long)]
        optimize: bool,

        /// Elabora ricorsivamente
        #[arg(short, long)]
        recursive: bool,
    },
    /// Ottieni informazioni sui file multimediali
    Info {
        /// File di input
        #[arg(short, long)]
        input: PathBuf,
    },
    /// Elenca i formati supportati
    Formats,
    /// Avvia l'interfaccia grafica
    Gui,
}
