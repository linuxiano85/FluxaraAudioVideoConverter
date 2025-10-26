use iced::{
    widget::{button, column, row, text, Space, text_input, checkbox, Column},
    theme, Alignment, Element, Length, Application, Settings, Theme, Color,
    Command, Subscription,
};
use std::fmt;
use std::path::PathBuf;
use crate::convert_files;
use anyhow::Result;
use crate::audio::{self, AudioEnhanceOptions};
use crate::video::{self, VideoEnhanceOptions, DenoiseType};
use crate::capture::{self, CaptureFormat, CaptureOptions};
use crate::clean_files;
use crate::show_info;
use crate::list_formats;
use rfd::AsyncFileDialog;


pub fn run() -> iced::Result {
    App::run(Settings::default())
}

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
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct App {
    current_page: Page,
    convert_input_path: String,
    convert_output_format: String,
    convert_output_dir: String,
    convert_recursive: bool,
    convert_quality: String,
    convert_codec: String,
    convert_jobs: String,
    enhance_audio_input: String,
    enhance_audio_output: String,
    enhance_audio_denoise: bool,
    enhance_audio_normalize: bool,
    enhance_audio_highpass: String,
    enhance_audio_lowpass: String,
    enhance_audio_notch: String,
    enhance_audio_compressor: bool,
    enhance_audio_gate: bool,
    enhance_audio_only: bool,
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
}

#[derive(Debug, Clone)]
enum Message {
    PageChanged(Page),
    ConvertInputPathChanged(String),
    ConvertOutputFormatChanged(String),
    ConvertOutputDirChanged(String),
    ConvertRecursiveToggled(bool),
    ConvertQualityChanged(String),
    ConvertCodecChanged(String),
    ConvertJobsChanged(String),
    ConvertButtonPressed,
    ConvertBrowseInput,
    ConvertBrowseOutput,
    ConvertInputFileSelected(Option<PathBuf>),
    ConvertOutputDirectorySelected(Option<PathBuf>),
    EnhanceAudioInputChanged(String),
    EnhanceAudioOutputChanged(String),
    EnhanceAudioDenoiseToggled(bool),
    EnhanceAudioNormalizeToggled(bool),
    EnhanceAudioHighpassChanged(String),
    EnhanceAudioLowpassChanged(String),
    EnhanceAudioNotchChanged(String),
    EnhanceAudioCompressorToggled(bool),
    EnhanceAudioGateToggled(bool),
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
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App {
                current_page: Page::Home,
                convert_input_path: String::new(),
                convert_output_format: "mp4".to_string(),
                convert_output_dir: String::new(),
                convert_recursive: false,
                convert_quality: "192k".to_string(),
                convert_codec: String::new(),
                convert_jobs: "4".to_string(),
                enhance_audio_input: String::new(),
                enhance_audio_output: String::new(),
                enhance_audio_denoise: true,
                enhance_audio_normalize: true,
                enhance_audio_highpass: "80".to_string(),
                enhance_audio_lowpass: String::new(),
                enhance_audio_notch: String::new(),
                enhance_audio_compressor: true,
                enhance_audio_gate: true,
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Fluxara AVC - {}", self.current_page)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PageChanged(page) => {
                self.current_page = page;
                Command::none()
            },
            Message::ConvertInputPathChanged(path) => {
                self.convert_input_path = path;
                Command::none()
            },
            Message::ConvertOutputFormatChanged(format) => {
                self.convert_output_format = format;
                Command::none()
            },
            Message::ConvertOutputDirChanged(dir) => {
                self.convert_output_dir = dir;
                Command::none()
            },
            Message::ConvertRecursiveToggled(toggle) => {
                self.convert_recursive = toggle;
                Command::none()
            },
            Message::ConvertQualityChanged(quality) => {
                self.convert_quality = quality;
                Command::none()
            },
            Message::ConvertCodecChanged(codec) => {
                self.convert_codec = codec;
                Command::none()
            },
            Message::ConvertJobsChanged(jobs) => {
                self.convert_jobs = jobs;
                Command::none()
            },
            Message::ConvertButtonPressed => {
                println!("Convert button pressed!");
                let input_path = PathBuf::from(&self.convert_input_path);
                let output_dir = if self.convert_output_dir.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(&self.convert_output_dir))
                };
                let codec_option = if self.convert_codec.is_empty() {
                    None
                } else {
                    Some(self.convert_codec.clone())
                };
                let jobs_parsed = self.convert_jobs.parse::<usize>().unwrap_or(4);

                let format_clone = self.convert_output_format.clone();
                let quality_clone = self.convert_quality.clone();
                let recursive_clone = self.convert_recursive;
                Command::perform(
                    async move {
                        convert_files(
                            &input_path,
                            &format_clone,
                            output_dir.as_ref(),
                            recursive_clone,
                            &quality_clone,
                            codec_option.as_ref(),
                            jobs_parsed,
                        )
                        .await
                    },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Conversion successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Conversion failed: {:?}", e);
                                Message::PageChanged(Page::Home)
                            }
                        }
                    },
                )
            }
            Message::ConvertBrowseInput => {
                let current_path = self.convert_input_path.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_file()
                            .await
                    },
                    |file_handle| Message::ConvertInputFileSelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::ConvertBrowseOutput => {
                let current_path = self.convert_output_dir.clone();
                Command::perform(
                    async move {
                        AsyncFileDialog::new()
                            .set_directory(&current_path)
                            .pick_folder()
                            .await
                    },
                    |file_handle| Message::ConvertOutputDirectorySelected(file_handle.map(|f| f.path().to_path_buf())),
                )
            }
            Message::ConvertInputFileSelected(path) => {
                if let Some(path_buf) = path {
                    self.convert_input_path = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::ConvertOutputDirectorySelected(path) => {
                if let Some(path_buf) = path {
                    self.convert_output_dir = path_buf.to_string_lossy().into_owned();
                }
                Command::none()
            }
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
            Message::EnhanceAudioOnlyToggled(toggle) => {
                self.enhance_audio_only = toggle;
                Command::none()
            },
            Message::EnhanceAudioButtonPressed => {
                println!("Enhance Audio button pressed!");
                let input_path = PathBuf::from(&self.enhance_audio_input);
                let output_path = PathBuf::from(&self.enhance_audio_output);
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
                    gate_threshold: -50.0,
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
                                Message::PageChanged(Page::Home)
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
                let output_path = PathBuf::from(&self.enhance_video_output);
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
                                Message::PageChanged(Page::Home)
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
                                Message::PageChanged(Page::Home)
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
                                Message::PageChanged(Page::Home)
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
                    async move { clean_files(&input_path, metadata_clone, optimize_clone, recursive_clone) },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Cleaning successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Cleaning failed: {:?}", e);
                                Message::PageChanged(Page::Home)
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
                    async move { show_info(&input_path) },
                    |result| {
                        match result {
                            Ok(_) => {
                                println!("Info retrieval successful!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Info retrieval failed: {:?}", e);
                                Message::PageChanged(Page::Home)
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
                        Ok(())
                    },
                    |result: Result<()>| {
                        match result {
                            Ok(_) => {
                                println!("Formats listed successfully!");
                                Message::PageChanged(Page::Home)
                            }
                            Err(e) => {
                                eprintln!("Formats listing failed: {:?}", e);
                                Message::PageChanged(Page::Home)
                            }
                        }
                    },
                )
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = column![
            nav_button(Page::Home, self.current_page),
            nav_button(Page::Convert, self.current_page),
            nav_button(Page::EnhanceAudio, self.current_page),
            nav_button(Page::EnhanceVideo, self.current_page),
            nav_button(Page::VhsRescue, self.current_page),
            nav_button(Page::Capture, self.current_page),
            nav_button(Page::Clean, self.current_page),
            nav_button(Page::Info, self.current_page),
            nav_button(Page::Formats, self.current_page),
        ]
        .spacing(10)
        .width(Length::Fixed(200.0))
        .padding(20);

        let content_widget = match self.current_page {
            Page::Home => home_page(),
            Page::Convert => convert_page(
                &self.convert_input_path,
                &self.convert_output_format,
                &self.convert_output_dir,
                self.convert_recursive,
                &self.convert_quality,
                &self.convert_codec,
                &self.convert_jobs,
            ),
            Page::EnhanceAudio => enhance_audio_page(
                &self.enhance_audio_input,
                &self.enhance_audio_output,
                self.enhance_audio_denoise,
                self.enhance_audio_normalize,
                &self.enhance_audio_highpass,
                &self.enhance_audio_lowpass,
                &self.enhance_audio_notch,
                self.enhance_audio_compressor,
                self.enhance_audio_gate,
                self.enhance_audio_only,
            ),
            Page::EnhanceVideo => enhance_video_page(
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
            ),
            Page::VhsRescue => vhs_rescue_page(
                &self.vhs_rescue_input,
                &self.vhs_rescue_output,
                &self.vhs_rescue_notch,
            ),
            Page::Capture => capture_page(
                &self.capture_output,
                &self.capture_video_device,
                &self.capture_audio_device,
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
            ),
            Page::Clean => clean_page(
                &self.clean_input,
                self.clean_metadata,
                self.clean_optimize,
                self.clean_recursive,
            ),
            Page::Info => info_page(
                &self.info_input,
            ),
            Page::Formats => formats_page(),
        };

        let content: Element<Message> = content_widget
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        row![sidebar, content]
            .spacing(20)
            .align_items(Alignment::Start)
            .into()
    }
}

fn nav_button(page: Page, current_page: Page) -> Element<'static, Message> {
    let is_active = page == current_page;
    let button_style = if is_active {
        theme::Button::Primary
    } else {
        theme::Button::Secondary
    };

    button(text(page.to_string()).size(20).horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(Message::PageChanged(page))
        .padding(10)
        .width(Length::Fill)
        .style(button_style)
        .into()
}

fn home_page() -> Column<'static, Message> {
    column![
        text("Welcome to Fluxara AVC!").size(40).style(Color::from_rgb8(0x00, 0xFF, 0x00)),
        text("Your Linux-first analog restoration & conversion tool.").size(25).style(Color::from_rgb8(0x00, 0xCC, 0xFF)),
        Space::with_height(Length::Fixed(50.0)),
        text("Select a function from the sidebar to get started.").size(20).style(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn convert_page(
    input_path: &str,
    output_format: &str,
    output_dir: &str,
    recursive: bool,
    quality: &str,
    codec: &str,
    jobs: &str,
) -> Column<'static, Message> {
    column![
        row![
            text_input("Input File or Directory", input_path)
                .on_input(Message::ConvertInputPathChanged)
                .padding(10),
            button("Browse").on_press(Message::ConvertBrowseInput),
        ]
        .spacing(10),
        row![
            text_input("Output Directory (optional)", output_dir)
                .on_input(Message::ConvertOutputDirChanged)
                .padding(10),
            button("Browse").on_press(Message::ConvertBrowseOutput),
        ]
        .spacing(10),
        text_input("Output Format (mp4, mp3, etc.)", output_format)
            .on_input(Message::ConvertOutputFormatChanged)
            .padding(10),
        checkbox("Process directories recursively", recursive)
            .on_toggle(Message::ConvertRecursiveToggled),
        text_input("Audio Quality (e.g., 192k)", quality)
            .on_input(Message::ConvertQualityChanged)
            .padding(10),
        text_input("Video Codec (e.g., libx264, optional)", codec)
            .on_input(Message::ConvertCodecChanged)
            .padding(10),
        text_input("Parallel Jobs (default: 4)", jobs)
            .on_input(Message::ConvertJobsChanged)
            .padding(10),
        button("Start Conversion").on_press(Message::ConvertButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn enhance_audio_page(
    input: &str,
    output: &str,
    denoise: bool,
    normalize: bool,
    highpass: &str,
    lowpass: &str,
    notch: &str,
    compressor: bool,
    gate: bool,
    audio_only: bool,
) -> Column<'static, Message> {
    column![
        text("Enhance Audio").size(30),
        row![
            text_input("Input File", input)
                .on_input(Message::EnhanceAudioInputChanged)
                .padding(10),
            button("Browse").on_press(Message::EnhanceAudioBrowseInput),
        ]
        .spacing(10),
        row![
            text_input("Output File", output)
                .on_input(Message::EnhanceAudioOutputChanged)
                .padding(10),
            button("Browse").on_press(Message::EnhanceAudioBrowseOutput),
        ]
        .spacing(10),
        checkbox("Enable Denoising", denoise)
            .on_toggle(Message::EnhanceAudioDenoiseToggled),
        checkbox("Enable Loudness Normalization", normalize)
            .on_toggle(Message::EnhanceAudioNormalizeToggled),
        text_input("High-pass Filter Frequency (Hz)", highpass)
            .on_input(Message::EnhanceAudioHighpassChanged)
            .padding(10),
        text_input("Low-pass Filter Frequency (Hz, optional)", lowpass)
            .on_input(Message::EnhanceAudioLowpassChanged)
            .padding(10),
        text_input("Notch Filter (50 or 60 Hz, optional)", notch)
            .on_input(Message::EnhanceAudioNotchChanged)
            .padding(10),
        checkbox("Enable Compressor", compressor)
            .on_toggle(Message::EnhanceAudioCompressorToggled),
        checkbox("Enable Noise Gate", gate)
            .on_toggle(Message::EnhanceAudioGateToggled),
        checkbox("Process Audio Only", audio_only)
            .on_toggle(Message::EnhanceAudioOnlyToggled),
        button("Start Audio Enhancement").on_press(Message::EnhanceAudioButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn enhance_video_page(
    input: &str,
    output: &str,
    deinterlace: bool,
    stabilize: bool,
    denoise_type: &str,
    sharpen: bool,
    color: bool,
    width: &str,
    height: &str,
    aspect: &str,
) -> Column<'static, Message> {
    column![
        text("Enhance Video").size(30),
        row![
            text_input("Input File", input)
                .on_input(Message::EnhanceVideoInputChanged)
                .padding(10),
            button("Browse").on_press(Message::EnhanceVideoBrowseInput),
        ]
        .spacing(10),
        row![
            text_input("Output File", output)
                .on_input(Message::EnhanceVideoOutputChanged)
                .padding(10),
            button("Browse").on_press(Message::EnhanceVideoBrowseOutput),
        ]
        .spacing(10),
        checkbox("Enable Deinterlacing", deinterlace)
            .on_toggle(Message::EnhanceVideoDeinterlaceToggled),
        checkbox("Enable Stabilization", stabilize)
            .on_toggle(Message::EnhanceVideoStabilizeToggled),
        text_input("Denoise Type (none, hqdn3d, nlmeans)", denoise_type)
            .on_input(Message::EnhanceVideoDenoiseTypeChanged)
            .padding(10),
        checkbox("Enable Sharpening", sharpen)
            .on_toggle(Message::EnhanceVideoSharpenToggled),
        checkbox("Enable Color Adjustment", color)
            .on_toggle(Message::EnhanceVideoColorToggled),
        text_input("Scale Width (optional)", width)
            .on_input(Message::EnhanceVideoWidthChanged)
            .padding(10),
        text_input("Scale Height (optional)", height)
            .on_input(Message::EnhanceVideoHeightChanged)
            .padding(10),
        text_input("Display Aspect Ratio (e.g., 4:3, 16:9)", aspect)
            .on_input(Message::EnhanceVideoAspectChanged)
            .padding(10),
        button("Start Video Enhancement").on_press(Message::EnhanceVideoButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn vhs_rescue_page(
    input: &str,
    output: &str,
    notch: &str,
) -> Column<'static, Message> {
    column![
        text("VHS Rescue").size(30),
        row![
            text_input("Input File", input)
                .on_input(Message::VhsRescueInputChanged)
                .padding(10),
            button("Browse").on_press(Message::VhsRescueBrowseInput),
        ]
        .spacing(10),
        row![
            text_input("Output File", output)
                .on_input(Message::VhsRescueOutputChanged)
                .padding(10),
            button("Browse").on_press(Message::VhsRescueBrowseOutput),
        ]
        .spacing(10),
        text_input("Notch Filter (50 or 60 Hz, optional)", notch)
            .on_input(Message::VhsRescueNotchChanged)
            .padding(10),
        button("Start VHS Rescue").on_press(Message::VhsRescueButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn capture_page(
    output: &str,
    video_device: &str,
    audio_device: &str,
    format: &str,
    deinterlace: bool,
    stabilize: bool,
    denoise: &str,
    vbitrate: &str,
    crf: &str,
    width: &str,
    height: &str,
    fps: &str,
    abitrate: &str,
    archival: bool,
) -> Column<'static, Message> {
    column![
        text("Capture Devices").size(30),
        row![
            text_input("Output File", output)
                .on_input(Message::CaptureOutputChanged)
                .padding(10),
            button("Browse").on_press(Message::CaptureBrowseOutput),
        ]
        .spacing(10),
        text_input("Video Device (e.g., /dev/video0)", video_device)
            .on_input(Message::CaptureVideoDeviceChanged)
            .padding(10),
        text_input("Audio Device (e.g., hw:1,0)", audio_device)
            .on_input(Message::CaptureAudioDeviceChanged)
            .padding(10),
        text_input("Output Format (mp4 or mkv)", format)
            .on_input(Message::CaptureFormatChanged)
            .padding(10),
        checkbox("Enable Deinterlacing", deinterlace)
            .on_toggle(Message::CaptureDeinterlaceToggled),
        checkbox("Enable Stabilization", stabilize)
            .on_toggle(Message::CaptureStabilizeToggled),
        text_input("Denoise Type (none, hqdn3d, nlmeans, optional)", denoise)
            .on_input(Message::CaptureDenoiseChanged)
            .padding(10),
        text_input("Video Bitrate (e.g., 5M, optional)", vbitrate)
            .on_input(Message::CaptureVbitrateChanged)
            .padding(10),
        text_input("CRF Value (18-28, lower = better quality, optional)", crf)
            .on_input(Message::CaptureCrfChanged)
            .padding(10),
        text_input("Video Width (optional)", width)
            .on_input(Message::CaptureWidthChanged)
            .padding(10),
        text_input("Video Height (optional)", height)
            .on_input(Message::CaptureHeightChanged)
            .padding(10),
        text_input("Frame Rate (optional)", fps)
            .on_input(Message::CaptureFpsChanged)
            .padding(10),
        text_input("Audio Bitrate (e.g., 192k)", abitrate)
            .on_input(Message::CaptureAbitrateChanged)
            .padding(10),
        checkbox("Archival Mode (lossless/near-lossless)", archival)
            .on_toggle(Message::CaptureArchivalToggled),
        button("Start Capture").on_press(Message::CaptureButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn clean_page(
    input: &str,
    metadata: bool,
    optimize: bool,
    recursive: bool,
) -> Column<'static, Message> {
    column![
        text("Clean Media Files").size(30),
        row![
            text_input("Input File or Directory", input)
                .on_input(Message::CleanInputChanged)
                .padding(10),
            button("Browse").on_press(Message::CleanBrowseInput),
        ]
        .spacing(10),
        checkbox("Remove Metadata", metadata)
            .on_toggle(Message::CleanMetadataToggled),
        checkbox("Optimize File Size", optimize)
            .on_toggle(Message::CleanOptimizeToggled),
        checkbox("Process Recursively", recursive)
            .on_toggle(Message::CleanRecursiveToggled),
        button("Start Cleaning").on_press(Message::CleanButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn info_page(input: &str) -> Column<'static, Message> {
    column![
        text("Media File Information").size(30),
        row![
            text_input("Input File", input)
                .on_input(Message::InfoInputChanged)
                .padding(10),
            button("Browse").on_press(Message::InfoBrowseInput),
        ]
        .spacing(10),
        button("Get Info").on_press(Message::InfoButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn formats_page() -> Column<'static, Message> {
    column![
        text("Supported Formats").size(30),
        button("List Formats").on_press(Message::FormatsButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
