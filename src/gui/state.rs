use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use crate::i18n::{self, Language};
use crate::gui::pages::convert::state::ConvertPageState;
use crate::gui::theme::AppTheme;

/// Enum che rappresenta le diverse pagine dell'applicazione.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Convert,
    EnhanceAudio,
    EnhanceVideo,
    VhsRescue,
    Capture,
    Clean,
    Info,
    Formats,
    Settings,
}

impl fmt::Display for Page {
    /// Implementa la visualizzazione del nome della pagina.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Struttura principale dell'applicazione che contiene lo stato di tutte le pagine.
pub struct App {
    pub current_page: Page,
    pub convert_page_state: ConvertPageState, // Stato della pagina di conversione
    pub enhance_audio_input: String,
    pub enhance_audio_output: String,
    pub enhance_audio_denoise: bool,
    pub enhance_audio_normalize: bool,
    pub enhance_audio_highpass: String,
    pub enhance_audio_lowpass: String,
    pub enhance_audio_notch: String,
    pub enhance_audio_compressor: bool,
    pub enhance_audio_gate: bool,
    pub enhance_audio_gate_threshold: String, // Nuovo campo per la soglia del gate
    pub enhance_audio_only: bool,
    pub video_devices: Arc<Vec<String>>, // Lista dei dispositivi video disponibili
    pub audio_devices: Arc<Vec<String>>, // Lista dei dispositivi audio disponibili
    pub enhance_video_input: String,
    pub enhance_video_output: String,
    pub enhance_video_deinterlace: bool,
    pub enhance_video_stabilize: bool,
    pub enhance_video_denoise_type: String,
    pub enhance_video_sharpen: bool,
    pub enhance_video_color: bool,
    pub enhance_video_width: String,
    pub enhance_video_height: String,
    pub enhance_video_aspect: String,
    pub vhs_rescue_input: String,
    pub vhs_rescue_output: String,
    pub vhs_rescue_notch: String,
    pub capture_output: String,
    pub capture_video_device: String,
    pub capture_audio_device: String,
    pub capture_format: String,
    pub capture_deinterlace: bool,
    pub capture_stabilize: bool,
    pub capture_denoise: String,
    pub capture_vbitrate: String,
    pub capture_crf: String,
    pub capture_width: String,
    pub capture_height: String,
    pub capture_fps: String,
    pub capture_abitrate: String,
    pub capture_archival: bool,
    pub clean_input: String,
    pub clean_metadata: bool,
    pub clean_optimize: bool,
    pub clean_recursive: bool,
    pub info_input: String,
    pub status_message: String,
    pub error_message: Option<String>, // Nuovo campo per i messaggi di errore
    pub conversion_progress: f32,
    pub is_converting: bool,
    pub current_theme: AppTheme,
    pub current_language: Language,
}

impl Default for App {
    /// Inizializza lo stato predefinito dell'applicazione.
    fn default() -> Self {
        App {
            current_page: Page::Home,
            convert_page_state: ConvertPageState::default(),
            enhance_audio_input: String::new(),
            error_message: None,
            enhance_audio_output: String::new(),
            video_devices: Arc::new(Vec::new()),
            audio_devices: Arc::new(Vec::new()),
            enhance_audio_denoise: true,
            enhance_audio_normalize: true,
            enhance_audio_highpass: "80".to_string(),
            enhance_audio_lowpass: String::new(),
            enhance_audio_notch: String::new(),
            enhance_audio_compressor: true,
            enhance_audio_gate: true,
            enhance_audio_gate_threshold: "-50.0".to_string(),
            enhance_audio_only: false,
            enhance_video_input: String::new(),
            enhance_video_output: String::new(),
            enhance_video_deinterlace: true,
            enhance_video_stabilize: false,
            enhance_video_denoise_type: "hqdn3d".to_string(),
            enhance_video_sharpen: true,
            enhance_video_color: true,
            enhance_video_width: String::new(),
            enhance_video_height: String::new(),
            enhance_video_aspect: String::new(),
            vhs_rescue_input: String::new(),
            vhs_rescue_output: String::new(),
            vhs_rescue_notch: String::new(),
            capture_output: String::new(),
            capture_video_device: "/dev/video0".to_string(),
            capture_audio_device: "hw:1,0".to_string(),
            capture_format: "mp4".to_string(),
            capture_deinterlace: true,
            capture_stabilize: false,
            capture_denoise: String::new(),
            capture_vbitrate: String::new(),
            capture_crf: String::new(),
            capture_width: String::new(),
            capture_height: String::new(),
            capture_fps: String::new(),
            capture_abitrate: "192k".to_string(),
            capture_archival: false,
            clean_input: String::new(),
            clean_metadata: false,
            clean_optimize: false,
            clean_recursive: false,
            info_input: String::new(),
            status_message: "Ready".to_string(),
            conversion_progress: 0.0,
            is_converting: false,
            current_theme: AppTheme::Dark,
            current_language: i18n::Language::En,
        }
    }
}

/// Funzione di utilità per generare un percorso di output predefinito.
/// Prende un percorso di input e un suffisso, e crea un nuovo nome di file
/// aggiungendo il suffisso prima dell'estensione.
pub fn generate_default_output_path(input_path_str: &str, suffix: &str) -> String {
    let input_path = PathBuf::from(input_path_str);
    if let Some(parent) = input_path.parent() {
        if let Some(stem) = input_path.file_stem() {
            if let Some(extension) = input_path.extension() {
                let new_filename = format!("{}_{}.{}", stem.to_string_lossy(), suffix, extension.to_string_lossy());
                return parent.join(new_filename).to_string_lossy().into_owned();
            }
        }
    }
    // Fallback se la manipolazione del percorso fallisce
    format!("{}_output", input_path_str)
}
