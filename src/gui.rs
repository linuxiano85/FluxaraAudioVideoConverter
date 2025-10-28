use iced::{
    widget::{button, column, row, text, Space, ProgressBar},
    Alignment, Element, Length, Application, Settings, Theme, Color,
    Command, Subscription, Renderer,
};
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc; // Importa Arc
use anyhow::Result;
use crate::audio::{self, AudioEnhanceOptions};
use crate::video::{self, VideoEnhanceOptions, DenoiseType};
use crate::capture::{self, CaptureFormat, CaptureOptions};
use crate::clean_files;
use crate::show_info;
use crate::list_formats;
use rfd::AsyncFileDialog; // Aggiunto l'import per AsyncFileDialog
use crate::i18n::{self, Language, translate};

mod pages; // Declare the pages module
mod theme; // Declare the theme module
use theme::AppTheme; // Import AppTheme from the new module
use pages::convert::messages::ConvertPageMessage;
use pages::convert::state::ConvertPageState;
use pages::convert::update as convert_update;
use pages::convert::view as convert_view;

/// Punto di ingresso principale per l'applicazione GUI.
/// Avvia l'applicazione Iced.
pub fn run() -> iced::Result {
    App::run(Settings::default())
}

/// Enum che rappresenta le diverse pagine dell'applicazione.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Struttura principale dell'applicazione che contiene lo stato di tutte le pagine.
struct App {
    current_page: Page,
    convert_page_state: ConvertPageState, // Stato della pagina di conversione
    enhance_audio_input: String,
    enhance_audio_output: String,
    enhance_audio_denoise: bool,
    enhance_audio_normalize: bool,
    enhance_audio_highpass: String,
    enhance_audio_lowpass: String,
    enhance_audio_notch: String,
    enhance_audio_compressor: bool,
    enhance_audio_gate: bool,
    enhance_audio_gate_threshold: String, // Nuovo campo per la soglia del gate
    enhance_audio_only: bool,
    video_devices: Arc<Vec<String>>, // Lista dei dispositivi video disponibili
    audio_devices: Arc<Vec<String>>, // Lista dei dispositivi audio disponibili
    enhance_video_input: String,
    enhance_video_output: String,
    enhance_video_deinterlace: bool,
    enhance_video_stabilize: bool,
    enhance_video_denoise_type: String,
    enhance_video_sharpen: bool,
    enhance_video_color: bool,
    enhance_video_width: String,
    enhance_video_height: String,
    enhance_video_aspect: String,
    vhs_rescue_input: String,
    vhs_rescue_output: String,
    vhs_rescue_notch: String,
    capture_output: String,
    capture_video_device: String,
    capture_audio_device: String,
    capture_format: String,
    capture_deinterlace: bool,
    capture_stabilize: bool,
    capture_denoise: String,
    capture_vbitrate: String,
    capture_crf: String,
    capture_width: String,
    capture_height: String,
    capture_fps: String,
    capture_abitrate: String,
    capture_archival: bool,
    clean_input: String,
    clean_metadata: bool,
    clean_optimize: bool,
    clean_recursive: bool,
    info_input: String,
    status_message: String,
    error_message: Option<String>, // Nuovo campo per i messaggi di errore
    conversion_progress: f32,
    is_converting: bool,
    current_theme: AppTheme,
    current_language: Language,
}

/// Funzione di utilità per generare un percorso di output predefinito.
/// Prende un percorso di input e un suffisso, e crea un nuovo nome di file
/// aggiungendo il suffisso prima dell'estensione.
fn generate_default_output_path(input_path_str: &str, suffix: &str) -> String {
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

/// Enum che definisce tutti i messaggi che l'applicazione può elaborare.
/// Questi messaggi vengono inviati all'applicazione per aggiornare il suo stato.
#[derive(Debug, Clone)]
pub enum Message {
    PageChanged(Page),
    ConvertPage(ConvertPageMessage), // Messaggi specifici per la pagina di conversione
    EnhanceAudioInputChanged(String),
    EnhanceAudioOutputChanged(String),
    EnhanceAudioDenoiseToggled(bool),
    EnhanceAudioNormalizeToggled(bool),
    EnhanceAudioHighpassChanged(String),
    EnhanceAudioLowpassChanged(String),
    EnhanceAudioNotchChanged(String),
    EnhanceAudioCompressorToggled(bool),
    EnhanceAudioGateToggled(bool),
    EnhanceAudioGateThresholdChanged(String), // Nuovo messaggio per la soglia del gate
    EnhanceAudioOnlyToggled(bool),
    EnhanceAudioButtonPressed,
    EnhanceAudioBrowseInput,
    EnhanceAudioBrowseOutput,
    EnhanceAudioInputFileSelected(Option<PathBuf>),
    EnhanceAudioOutputFileSelected(Option<PathBuf>),
    EnhanceVideoInputChanged(String),
    EnhanceVideoOutputChanged(String),
    EnhanceVideoDeinterlaceToggled(bool),
    EnhanceVideoStabilizeToggled(bool),
    EnhanceVideoDenoiseTypeChanged(String),
    EnhanceVideoSharpenToggled(bool),
    EnhanceVideoColorToggled(bool),
    EnhanceVideoWidthChanged(String),
    EnhanceVideoHeightChanged(String),
    EnhanceVideoAspectChanged(String),
    EnhanceVideoButtonPressed,
    EnhanceVideoBrowseInput,
    EnhanceVideoBrowseOutput,
    EnhanceVideoInputFileSelected(Option<PathBuf>),
    EnhanceVideoOutputFileSelected(Option<PathBuf>),
    VhsRescueInputChanged(String),
    VhsRescueOutputChanged(String),
    VhsRescueNotchChanged(String),
    VhsRescueButtonPressed,
    VhsRescueBrowseInput,
    VhsRescueBrowseOutput,
    VhsRescueInputFileSelected(Option<PathBuf>),
    VhsRescueOutputFileSelected(Option<PathBuf>),
    CaptureOutputChanged(String),
    CaptureVideoDeviceChanged(String),
    CaptureAudioDeviceChanged(String),
    CaptureFormatChanged(String),
    CaptureDeinterlaceToggled(bool),
    CaptureStabilizeToggled(bool),
    CaptureDenoiseChanged(String),
    CaptureVbitrateChanged(String),
    CaptureCrfChanged(String),
    CaptureWidthChanged(String),
    CaptureHeightChanged(String),
    CaptureFpsChanged(String),
    CaptureAbitrateChanged(String),
    CaptureArchivalToggled(bool),
    CaptureButtonPressed,
    CaptureBrowseOutput,
    CaptureOutputFileSelected(Option<PathBuf>),
    VideoDevicesLoaded(Vec<String>), // Messaggio per caricare i dispositivi video
    AudioDevicesLoaded(Vec<String>), // Messaggio per caricare i dispositivi audio
    CleanInputChanged(String),
    CleanMetadataToggled(bool),
    CleanOptimizeToggled(bool),
    CleanRecursiveToggled(bool),
    CleanButtonPressed,
    CleanBrowseInput,
    CleanInputFileSelected(Option<PathBuf>),
    InfoInputChanged(String),
    InfoButtonPressed,
    InfoBrowseInput,
    InfoInputFileSelected(Option<PathBuf>),
    FormatsButtonPressed,
    ConversionStarted,
    ConversionProgressed(f32),
    ConversionCompleted(Result<(), String>),
    UpdateStatus(String),
    ThemeChanged(AppTheme),
    LanguageChanged(Language),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    /// Inizializza lo stato dell'applicazione.
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App {
                current_page: Page::Home,
                convert_page_state: ConvertPageState::default(), // Inizializza lo stato della pagina di conversione
                enhance_audio_input: String::new(),
                error_message: None, // Inizializza il messaggio di errore a None
                enhance_audio_output: String::new(),
                video_devices: Arc::new(Vec::new()), // Inizializza liste vuote
                audio_devices: Arc::new(Vec::new()), // Inizializza liste vuote
                enhance_audio_denoise: true,
                enhance_audio_normalize: true,
                enhance_audio_highpass: "80".to_string(),
                enhance_audio_lowpass: String::new(),
                enhance_audio_notch: String::new(),
                enhance_audio_compressor: true,
                enhance_audio_gate: true,
                enhance_audio_gate_threshold: "-50.0".to_string(), // Valore predefinito
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
            },
            Command::batch(vec![
                Command::perform(async { crate::capture::list_video_devices().unwrap_or_default() }, Message::VideoDevicesLoaded),
                Command::perform(async { crate::capture::list_audio_devices().unwrap_or_default() }, Message::AudioDevicesLoaded),
            ]),
        )
    }

    /// Restituisce il titolo della finestra dell'applicazione.
    fn title(&self) -> String {
        translate(&format!("title_{:?}", self.current_page))
    }

    /// Aggiorna lo stato dell'applicazione in base al messaggio ricevuto.
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PageChanged(page) => {
                self.current_page = page;
                Command::none()
            },
            Message::ConvertPage(convert_msg) => {
                convert_update::update(&mut self.convert_page_state, convert_msg)
            },
            Message::EnhanceAudioInputChanged(path) => {
                self.enhance_audio_input = path;
                Command::none()
            },
            Message::EnhanceAudioOutputChanged(path) => {
                self.enhance_audio_output = path;
                Command::none()
            },
            Message::EnhanceAudioDenoiseToggled(toggle) => {
                self.enhance_audio_denoise = toggle;
                Command::none()
            },
            Message::EnhanceAudioNormalizeToggled(toggle) => {
                self.enhance_audio_normalize = toggle;
                Command::none()
            },
            Message::EnhanceAudioHighpassChanged(freq) => {
                self.enhance_audio_highpass = freq;
                Command::none()
            },
            Message::EnhanceAudioLowpassChanged(freq) => {
                self.enhance_audio_lowpass = freq;
                Command::none()
            },
            Message::EnhanceAudioNotchChanged(freq) => {
                self.enhance_audio_notch = freq;
                Command::none()
            },
            Message::EnhanceAudioCompressorToggled(toggle) => {
                self.enhance_audio_compressor = toggle;
                Command::none()
            },
            Message::EnhanceAudioGateToggled(toggle) => {
                self.enhance_audio_gate = toggle;
                Command::none()
            },
            Message::VideoDevicesLoaded(devices) => {
                self.video_devices = Arc::new(devices);
                Command::none()
            },
            Message::AudioDevicesLoaded(devices) => {
                self.audio_devices = Arc::new(devices);
                Command::none()
            },
            Message::EnhanceAudioGateThresholdChanged(threshold) => {
                self.enhance_audio_gate_threshold = threshold;
                Command::none()
            },
            Message::EnhanceAudioOnlyToggled(toggle) => {
                self.enhance_audio_only = toggle;
                Command::none()
            },
            Message::EnhanceAudioButtonPressed => {
                println!("Enhance Audio button pressed!");
                let input_path = PathBuf::from(&self.enhance_audio_input);
                let output_path_str = if self.enhance_audio_output.is_empty() {
                    generate_default_output_path(&self.enhance_audio_input, "enhanced_audio")
                } else {
                    self.enhance_audio_output.clone()
                };
                let output_path = PathBuf::from(&output_path_str);
                let highpass_freq = self.enhance_audio_highpass.parse::<u32>().ok();
                let lowpass_freq = self.enhance_audio_lowpass.parse::<u32>().ok();
                let notch_freq = self.enhance_audio_notch.parse::<u32>().ok();

                let opts = AudioEnhanceOptions {
                    denoise: self.enhance_audio_denoise,
                    normalize: self.enhance_audio_normalize,
                    highpass_freq,
                    lowpass_freq,
                    notch_freq,
                    compressor: self.enhance_audio_compressor,
                    gate: self.enhance_audio_gate,
                    gate_threshold: self.enhance_audio_gate_threshold.parse::<f32>().ok(),
                };

                let audio_only_clone = self.enhance_audio_only;
                Command::perform(
                    async move {
                        let result = if audio_only_clone {
                            audio::enhance_audio_only(&input_path, &output_path, &opts)
                        } else {
                            audio::enhance_audio(&input_path, &output_path, &opts)
                        };
                        result
                    },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Audio enhancement successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Audio enhancement failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            }
            Message::EnhanceAudioBrowseInput => {
                let current_path = self.enhance_audio_input.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::EnhanceAudioInputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::EnhanceAudioBrowseOutput => {
                let current_path = self.enhance_audio_output.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::EnhanceAudioOutputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::EnhanceAudioInputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.enhance_audio_input = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::EnhanceAudioOutputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.enhance_audio_output = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::EnhanceVideoInputChanged(path) => {
                self.enhance_video_input = path;
                Command::none()
            },
            Message::EnhanceVideoOutputChanged(path) => {
                self.enhance_video_output = path;
                Command::none()
            },
            Message::EnhanceVideoDeinterlaceToggled(toggle) => {
                self.enhance_video_deinterlace = toggle;
                Command::none()
            },
            Message::EnhanceVideoStabilizeToggled(toggle) => {
                self.enhance_video_stabilize = toggle;
                Command::none()
            },
            Message::EnhanceVideoDenoiseTypeChanged(denoise_type) => {
                self.enhance_video_denoise_type = denoise_type;
                Command::none()
            },
            Message::EnhanceVideoSharpenToggled(toggle) => {
                self.enhance_video_sharpen = toggle;
                Command::none()
            },
            Message::EnhanceVideoColorToggled(toggle) => {
                self.enhance_video_color = toggle;
                Command::none()
            },
            Message::EnhanceVideoWidthChanged(width) => {
                self.enhance_video_width = width;
                Command::none()
            },
            Message::EnhanceVideoHeightChanged(height) => {
                self.enhance_video_height = height;
                Command::none()
            },
            Message::EnhanceVideoAspectChanged(aspect) => {
                self.enhance_video_aspect = aspect;
                Command::none()
            },
            Message::EnhanceVideoButtonPressed => {
                println!("Enhance Video button pressed!");
                let input_path = PathBuf::from(&self.enhance_video_input);
                let output_path_str = if self.enhance_video_output.is_empty() {
                    generate_default_output_path(&self.enhance_video_input, "enhanced_video")
                } else {
                    self.enhance_video_output.clone()
                };
                let output_path = PathBuf::from(&output_path_str);
                let width = self.enhance_video_width.parse::<u32>().ok();
                let height = self.enhance_video_height.parse::<u32>().ok();
                let aspect_ratio = if self.enhance_video_aspect.is_empty() {
                    None
                } else {
                    Some(self.enhance_video_aspect.clone())
                };

                let denoise_type = match self.enhance_video_denoise_type.as_str() {
                    "none" => DenoiseType::None,
                    "nlmeans" => DenoiseType::Nlmeans,
                    _ => DenoiseType::Hqdn3d,
                };

                let opts = VideoEnhanceOptions {
                    deinterlace: self.enhance_video_deinterlace,
                    stabilize: self.enhance_video_stabilize,
                    denoise: denoise_type,
                    sharpen: self.enhance_video_sharpen,
                    color_adjust: self.enhance_video_color,
                    scale_width: width,
                    scale_height: height,
                    aspect_ratio,
                };

                Command::perform(
                    async move { video::enhance_video(&input_path, &output_path, &opts) },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Video enhancement successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Video enhancement failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            }
            Message::EnhanceVideoBrowseInput => {
                let current_path = self.enhance_video_input.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::EnhanceVideoInputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::EnhanceVideoBrowseOutput => {
                let current_path = self.enhance_video_output.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::EnhanceVideoOutputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::EnhanceVideoInputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.enhance_video_input = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::EnhanceVideoOutputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.enhance_video_output = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::VhsRescueInputChanged(path) => {
                self.vhs_rescue_input = path;
                Command::none()
            },
            Message::VhsRescueOutputChanged(path) => {
                self.vhs_rescue_output = path;
                Command::none()
            },
            Message::VhsRescueNotchChanged(freq) => {
                self.vhs_rescue_notch = freq;
                Command::none()
            },
            Message::VhsRescueButtonPressed => {
                println!("VHS Rescue button pressed!");
                let input_path = PathBuf::from(&self.vhs_rescue_input);
                let output_path = PathBuf::from(&self.vhs_rescue_output);
                let notch_freq = self.vhs_rescue_notch.parse::<u32>().ok();

                Command::perform(
                    async move { video::vhs_rescue(&input_path, &output_path, notch_freq) },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("VHS Rescue successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("VHS Rescue failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            }
            Message::VhsRescueBrowseInput => {
                let current_path = self.vhs_rescue_input.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::VhsRescueInputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::VhsRescueBrowseOutput => {
                let current_path = self.vhs_rescue_output.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::VhsRescueOutputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::VhsRescueInputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.vhs_rescue_input = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::VhsRescueOutputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.vhs_rescue_output = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::CaptureOutputChanged(path) => {
                self.capture_output = path;
                Command::none()
            },
            Message::CaptureVideoDeviceChanged(device) => {
                self.capture_video_device = device;
                Command::none()
            },
            Message::CaptureAudioDeviceChanged(device) => {
                self.capture_audio_device = device;
                Command::none()
            },
            Message::CaptureFormatChanged(format) => {
                self.capture_format = format;
                Command::none()
            },
            Message::CaptureDeinterlaceToggled(toggle) => {
                self.capture_deinterlace = toggle;
                Command::none()
            },
            Message::CaptureStabilizeToggled(toggle) => {
                self.capture_stabilize = toggle;
                Command::none()
            },
            Message::CaptureDenoiseChanged(denoise) => {
                self.capture_denoise = denoise;
                Command::none()
            },
            Message::CaptureVbitrateChanged(bitrate) => {
                self.capture_vbitrate = bitrate;
                Command::none()
            },
            Message::CaptureCrfChanged(crf) => {
                self.capture_crf = crf;
                Command::none()
            },
            Message::CaptureWidthChanged(width) => {
                self.capture_width = width;
                Command::none()
            },
            Message::CaptureHeightChanged(height) => {
                self.capture_height = height;
                Command::none()
            },
            Message::CaptureFpsChanged(fps) => {
                self.capture_fps = fps;
                Command::none()
            },
            Message::CaptureAbitrateChanged(bitrate) => {
                self.capture_abitrate = bitrate;
                Command::none()
            },
            Message::CaptureArchivalToggled(toggle) => {
                self.capture_archival = toggle;
                Command::none()
            },
            Message::CaptureButtonPressed => {
                println!("Capture button pressed!");
                let output_path = PathBuf::from(&self.capture_output);
                let video_device_clone = self.capture_video_device.clone();
                let audio_device_clone = self.capture_audio_device.clone();
                let format_clone = self.capture_format.clone();
                let deinterlace_clone = self.capture_deinterlace;
                let stabilize_clone = self.capture_stabilize;
                let denoise_clone = if self.capture_denoise.is_empty() {
                    None
                } else {
                    Some(self.capture_denoise.clone())
                };
                let vbitrate_clone = if self.capture_vbitrate.is_empty() {
                    None
                } else {
                    Some(self.capture_vbitrate.clone())
                };
                let crf_clone = self.capture_crf.parse::<u32>().ok();
                let width_clone = self.capture_width.parse::<u32>().ok();
                let height_clone = self.capture_height.parse::<u32>().ok();
                let fps_clone = self.capture_fps.parse::<u32>().ok();
                let abitrate_clone = self.capture_abitrate.clone();
                let archival_clone = self.capture_archival;

                let capture_format = match format_clone.as_str() {
                    "mkv" => CaptureFormat::Mkv,
                    _ => CaptureFormat::Mp4,
                };

                let opts = CaptureOptions {
                    format: capture_format,
                    video_device: video_device_clone,
                    audio_device: audio_device_clone,
                    deinterlace: deinterlace_clone,
                    stabilize: stabilize_clone,
                    denoise: denoise_clone,
                    video_bitrate: vbitrate_clone,
                    crf: crf_clone,
                    width: width_clone,
                    height: height_clone,
                    fps: fps_clone,
                    audio_bitrate: abitrate_clone,
                    archival_mode: archival_clone,
                };

                Command::perform(
                    async move { capture::capture(&output_path, &opts) },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Capture successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Capture failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            }
            Message::CaptureBrowseOutput => {
                let current_path = self.capture_output.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::CaptureOutputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::CaptureOutputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.capture_output = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::CleanInputChanged(path) => {
                self.clean_input = path;
                Command::none()
            },
            Message::CleanMetadataToggled(toggle) => {
                self.clean_metadata = toggle;
                Command::none()
            },
            Message::CleanOptimizeToggled(toggle) => {
                self.clean_optimize = toggle;
                Command::none()
            },
            Message::CleanRecursiveToggled(toggle) => {
                self.clean_recursive = toggle;
                Command::none()
            },
            Message::CleanButtonPressed => {
                println!("Clean button pressed!");
                let input_path = PathBuf::from(&self.clean_input);
                let metadata_clone = self.clean_metadata;
                let optimize_clone = self.clean_optimize;
                let recursive_clone = self.clean_recursive;

                Command::perform(
                    async move { clean_files(&input_path, metadata_clone, optimize_clone, recursive_clone).await },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Cleaning successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Cleaning failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            }
            Message::CleanBrowseInput => {
                let current_path = self.clean_input.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::CleanInputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::CleanInputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.clean_input = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::InfoInputChanged(path) => {
                self.info_input = path;
                Command::none()
            },
            Message::InfoButtonPressed => {
                println!("Info button pressed!");
                let input_path = PathBuf::from(&self.info_input);
                Command::perform(
                    async move { show_info(&input_path).await },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Info retrieval successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Info retrieval failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            }
            Message::InfoBrowseInput => {
                let current_path = self.info_input.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::InfoInputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::InfoInputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.info_input = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::FormatsButtonPressed => {
                println!("Formats button pressed!");
                Command::perform(
                    async move {
                        list_formats();
                        Ok::<(), anyhow::Error>(())
                    },
                    |result: Result<()>| {
                        match result {
                            Ok(_) => {
                                println!("Formats listed successfully!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Formats listing failed: {:?}", e);
                                Message::UpdateStatus(format!("Error: {}", e))
                            }
                        }
                    },
                )
            },
            Message::ConversionStarted => {
                self.is_converting = true;
                self.conversion_progress = 0.0;
                self.status_message = "Conversion started...".to_string();
                Command::none()
            },
            Message::ConversionProgressed(progress) => {
                self.conversion_progress = progress;
                Command::none()
            },
            Message::ConversionCompleted(result) => {
                self.is_converting = false;
                self.conversion_progress = 0.0;
                match result {
                    Ok(_) => self.status_message = "Conversion completed successfully!".to_string(),
                    Err(e) => self.status_message = format!("Conversion failed: {}", e),
                }
                Command::none()
            },
            Message::UpdateStatus(message) => {
                self.status_message = message;
                Command::none()
            },
            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
                Command::none()
            },
            Message::LanguageChanged(language) => {
                self.current_language = language;
                i18n::set_language(language); // Set the global language
                Command::none()
            },
        }
    }


    fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        let sidebar = column![
            nav_button(Page::Home, self.current_page, &self.current_theme),
            nav_button(Page::Convert, self.current_page, &self.current_theme),
            nav_button(Page::EnhanceAudio, self.current_page, &self.current_theme),
            nav_button(Page::EnhanceVideo, self.current_page, &self.current_theme),
            nav_button(Page::VhsRescue, self.current_page, &self.current_theme),
            nav_button(Page::Capture, self.current_page, &self.current_theme),
            nav_button(Page::Clean, self.current_page, &self.current_theme),
            nav_button(Page::Info, self.current_page, &self.current_theme),
            nav_button(Page::Formats, self.current_page, &self.current_theme),
            nav_button(Page::Settings, self.current_page, &self.current_theme),
        ]
        .spacing(10)
        .width(Length::Fixed(200.0))
        .padding(20);

        let content_widget: Element<'_, Message, Theme, Renderer> = match self.current_page {
            Page::Home => pages::home::home_page().into(),
            Page::Convert => convert_view::convert_page(
                &self.convert_page_state,
            ).into(),
            Page::EnhanceAudio => pages::enhance_audio::enhance_audio_page(
                &self.enhance_audio_input,
                &self.enhance_audio_output,
                self.enhance_audio_denoise,
                self.enhance_audio_normalize,
                &self.enhance_audio_highpass,
                &self.enhance_audio_lowpass,
                &self.enhance_audio_notch,
                self.enhance_audio_compressor,
                self.enhance_audio_gate,
                &self.enhance_audio_gate_threshold,
                self.enhance_audio_only,
            ).into(),
            Page::EnhanceVideo => pages::enhance_video::enhance_video_page(
                &self.enhance_video_input,
                &self.enhance_video_output,
                self.enhance_video_deinterlace,
                self.enhance_video_stabilize,
                &self.enhance_video_denoise_type,
                self.enhance_video_sharpen,
                self.enhance_video_color,
                &self.enhance_video_width,
                &self.enhance_video_height,
                &self.enhance_video_aspect,
            ).into(),
            Page::VhsRescue => pages::vhs_rescue::vhs_rescue_page(
                &self.vhs_rescue_input,
                &self.vhs_rescue_output,
                &self.vhs_rescue_notch,
            ).into(),
            Page::Capture => pages::capture::capture_page(
                &self.capture_output,
                &self.capture_video_device,
                &self.video_devices, // Passa la lista dei dispositivi video
                &self.capture_audio_device,
                &self.audio_devices, // Passa la lista dei dispositivi audio
                &self.capture_format,
                self.capture_deinterlace,
                self.capture_stabilize,
                &self.capture_denoise,
                &self.capture_vbitrate,
                &self.capture_crf,
                &self.capture_width,
                &self.capture_height,
                &self.capture_fps,
                &self.capture_abitrate,
                self.capture_archival,
            ).into(),
            Page::Clean => pages::clean::clean_page(
                &self.clean_input,
                self.clean_metadata,
                self.clean_optimize,
                self.clean_recursive,
            ).into(),
            Page::Info => pages::info::info_page(
                &self.info_input,
            ).into(),
            Page::Formats => pages::formats::formats_page().into(),
            Page::Settings => pages::settings::settings_page(self.current_theme, self.current_language).into(),
        };

        let content: Element<Message, Theme, Renderer> = content_widget.into();

        let status_bar: Element<'_, Message, Theme, Renderer> = row![
            text::<'_, Theme, Renderer>(&self.status_message).size(16).style(iced::theme::Text::Color(self.current_theme.into())),
            if self.is_converting {
                ProgressBar::<Theme>::new(0.0..=100.0, self.conversion_progress)
                    .width(Length::Fixed(200.0))
                    .style(iced::theme::ProgressBar::Custom(Box::new(self.current_theme) as Box<dyn widget::progress_bar::StyleSheet<Style = Theme>>))
                    .into()
            } else {
                Space::with_width(Length::Fixed(200.0)).into()
            },
            if let Some(error) = &self.error_message {
                text::<'_, Theme, Renderer>(error).size(16).style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0))).into()
            } else {
                Space::with_width(Length::Fixed(0.0)).into()
            }
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .padding(10)
        .width(Length::Fill)
        .into();

        column![
            row![sidebar, content]
                .spacing(20)
                .align_items(Alignment::Start)
                .height(Length::Fill),
            status_bar,
        ]
        .into()
    }

    /// Restituisce il tema corrente dell'applicazione.
    fn theme(&self) -> Theme {
        Theme::from(self.current_theme) // Conversione diretta
    }

    /// Gestisce le sottoscrizioni per eventi esterni.
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

/// Crea un pulsante di navigazione per la sidebar.
/// Il pulsante sarà evidenziato se la pagina corrispondente è quella corrente.
fn nav_button(page: Page, current_page: Page, app_theme: &AppTheme) -> Element<'static, Message, Theme, Renderer> {
    let is_active = page == current_page;
    let button_style = if is_active {
        iced::theme::Button::Primary
    } else {
        iced::theme::Button::Secondary
    };

    button(text(translate(&format!("nav_{}", page.to_string().to_lowercase()))).size(20).horizontal_alignment(iced::alignment::Horizontal::Center).style(iced::theme::Text::Color((*app_theme).into())))
        .on_press(Message::PageChanged(page))
        .padding(10)
        .width(Length::Fill)
        .style(button_style)
        .into()
}
