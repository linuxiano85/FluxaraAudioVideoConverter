use iced::{Command};
use std::path::PathBuf;
use anyhow::Result;
use crate::audio::{self, AudioEnhanceOptions};
use crate::video::{self, VideoEnhanceOptions, DenoiseType};
use crate::capture::{self, CaptureFormat, CaptureOptions};
use crate::clean_files;
use crate::show_info;
use crate::list_formats;
use rfd::AsyncFileDialog;
use crate::i18n;

use crate::gui::state::{App, Page, generate_default_output_path};
use crate::gui::Message;
use crate::gui::pages::convert::update as convert_update;

/// Aggiorna lo stato dell'applicazione in base al messaggio ricevuto.
///
/// Questa funzione gestisce tutti i messaggi dell'applicazione, modificando lo stato
/// dell'applicazione (`App`) e restituendo un `Command` per eseguire effetti collaterali.
///
/// # Argomenti
///
/// * `app` - Un riferimento mutabile allo stato dell'applicazione.
/// * `message` - Il messaggio da elaborare.
///
/// # Restituisce
///
/// Un `Command<Message>` che può essere `Command::none()` se non ci sono effetti collaterali,
/// o un comando per eseguire un'operazione asincrona o un batch di comandi.
pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::PageChanged(page) => {
            app.current_page = page;
            Command::none()
        },
        Message::ConvertPage(convert_msg) => {
            convert_update::update(&mut app.convert_page_state, convert_msg)
        },
        Message::EnhanceAudioInputChanged(path) => {
            app.enhance_audio_input = path;
            Command::none()
        },
        Message::EnhanceAudioOutputChanged(path) => {
            app.enhance_audio_output = path;
            Command::none()
        },
        Message::EnhanceAudioDenoiseToggled(toggle) => {
            app.enhance_audio_denoise = toggle;
            Command::none()
        },
        Message::EnhanceAudioNormalizeToggled(toggle) => {
            app.enhance_audio_normalize = toggle;
            Command::none()
        },
        Message::EnhanceAudioHighpassChanged(freq) => {
            app.enhance_audio_highpass = freq;
            Command::none()
        },
        Message::EnhanceAudioLowpassChanged(freq) => {
            app.enhance_audio_lowpass = freq;
            Command::none()
        },
        Message::EnhanceAudioNotchChanged(freq) => {
            app.enhance_audio_notch = freq;
            Command::none()
        },
        Message::EnhanceAudioCompressorToggled(toggle) => {
            app.enhance_audio_compressor = toggle;
            Command::none()
        },
        Message::EnhanceAudioGateToggled(toggle) => {
            app.enhance_audio_gate = toggle;
            Command::none()
        },
        Message::VideoDevicesLoaded(devices) => {
            app.video_devices = std::sync::Arc::new(devices);
            Command::none()
        },
        Message::AudioDevicesLoaded(devices) => {
            app.audio_devices = std::sync::Arc::new(devices);
            Command::none()
        },
        Message::EnhanceAudioGateThresholdChanged(threshold) => {
            app.enhance_audio_gate_threshold = threshold;
            Command::none()
        },
        Message::EnhanceAudioOnlyToggled(toggle) => {
            app.enhance_audio_only = toggle;
            Command::none()
        },
        Message::EnhanceAudioButtonPressed => {
            println!("Enhance Audio button pressed!");
            let input_path = PathBuf::from(&app.enhance_audio_input);
            let output_path_str = if app.enhance_audio_output.is_empty() {
                generate_default_output_path(&app.enhance_audio_input, "enhanced_audio")
            } else {
                app.enhance_audio_output.clone()
            };
            let output_path = PathBuf::from(&output_path_str);
            let highpass_freq = app.enhance_audio_highpass.parse::<u32>().ok();
            let lowpass_freq = app.enhance_audio_lowpass.parse::<u32>().ok();
            let notch_freq = app.enhance_audio_notch.parse::<u32>().ok();

            let opts = AudioEnhanceOptions {
                denoise: app.enhance_audio_denoise,
                normalize: app.enhance_audio_normalize,
                highpass_freq,
                lowpass_freq,
                notch_freq,
                compressor: app.enhance_audio_compressor,
                gate: app.enhance_audio_gate,
                gate_threshold: app.enhance_audio_gate_threshold.parse::<f32>().ok(),
            };

            let audio_only_clone = app.enhance_audio_only;
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
            let current_path = app.enhance_audio_input.clone();
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
            let current_path = app.enhance_audio_output.clone();
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
                app.enhance_audio_input = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::EnhanceAudioOutputFileSelected(path) => {
            if let Some(path_buf) = path {
                app.enhance_audio_output = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::EnhanceVideoInputChanged(path) => {
            app.enhance_video_input = path;
            Command::none()
        },
        Message::EnhanceVideoOutputChanged(path) => {
            app.enhance_video_output = path;
            Command::none()
        },
        Message::EnhanceVideoDeinterlaceToggled(toggle) => {
            app.enhance_video_deinterlace = toggle;
            Command::none()
        },
        Message::EnhanceVideoStabilizeToggled(toggle) => {
            app.enhance_video_stabilize = toggle;
            Command::none()
        },
        Message::EnhanceVideoDenoiseTypeChanged(denoise_type) => {
            app.enhance_video_denoise_type = denoise_type;
            Command::none()
        },
        Message::EnhanceVideoSharpenToggled(toggle) => {
            app.enhance_video_sharpen = toggle;
            Command::none()
        },
        Message::EnhanceVideoColorToggled(toggle) => {
            app.enhance_video_color = toggle;
            Command::none()
        },
        Message::EnhanceVideoWidthChanged(width) => {
            app.enhance_video_width = width;
            Command::none()
        },
        Message::EnhanceVideoHeightChanged(height) => {
            app.enhance_video_height = height;
            Command::none()
        },
        Message::EnhanceVideoAspectChanged(aspect) => {
            app.enhance_video_aspect = aspect;
            Command::none()
        },
        Message::EnhanceVideoButtonPressed => {
            println!("Enhance Video button pressed!");
            let input_path = PathBuf::from(&app.enhance_video_input);
            let output_path_str = if app.enhance_video_output.is_empty() {
                generate_default_output_path(&app.enhance_video_input, "enhanced_video")
            } else {
                app.enhance_video_output.clone()
            };
            let output_path = PathBuf::from(&output_path_str);
            let width = app.enhance_video_width.parse::<u32>().ok();
            let height = app.enhance_video_height.parse::<u32>().ok();
            let aspect_ratio = if app.enhance_video_aspect.is_empty() {
                None
            } else {
                Some(app.enhance_video_aspect.clone())
            };

            let denoise_type = match app.enhance_video_denoise_type.as_str() {
                "none" => DenoiseType::None,
                "nlmeans" => DenoiseType::Nlmeans,
                _ => DenoiseType::Hqdn3d,
            };

            let opts = VideoEnhanceOptions {
                deinterlace: app.enhance_video_deinterlace,
                stabilize: app.enhance_video_stabilize,
                denoise: denoise_type,
                sharpen: app.enhance_video_sharpen,
                color_adjust: app.enhance_video_color,
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
            let current_path = app.enhance_video_input.clone();
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
            let current_path = app.enhance_video_output.clone();
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
                app.enhance_video_input = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::EnhanceVideoOutputFileSelected(path) => {
            if let Some(path_buf) = path {
                app.enhance_video_output = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::VhsRescueInputChanged(path) => {
            app.vhs_rescue_input = path;
            Command::none()
        },
        Message::VhsRescueOutputChanged(path) => {
            app.vhs_rescue_output = path;
            Command::none()
        },
        Message::VhsRescueNotchChanged(freq) => {
            app.vhs_rescue_notch = freq;
            Command::none()
        },
        Message::VhsRescueButtonPressed => {
            println!("VHS Rescue button pressed!");
            let input_path = PathBuf::from(&app.vhs_rescue_input);
            let output_path = PathBuf::from(&app.vhs_rescue_output);
            let notch_freq = app.vhs_rescue_notch.parse::<u32>().ok();

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
            let current_path = app.vhs_rescue_input.clone();
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
            let current_path = app.vhs_rescue_output.clone();
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
                app.vhs_rescue_input = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::VhsRescueOutputFileSelected(path) => {
            if let Some(path_buf) = path {
                app.vhs_rescue_output = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::CaptureOutputChanged(path) => {
            app.capture_output = path;
            Command::none()
        },
        Message::CaptureVideoDeviceChanged(device) => {
            app.capture_video_device = device;
            Command::none()
        },
        Message::CaptureAudioDeviceChanged(device) => {
            app.capture_audio_device = device;
            Command::none()
        },
        Message::CaptureFormatChanged(format) => {
            app.capture_format = format;
            Command::none()
        },
        Message::CaptureDeinterlaceToggled(toggle) => {
            app.capture_deinterlace = toggle;
            Command::none()
        },
        Message::CaptureStabilizeToggled(toggle) => {
            app.capture_stabilize = toggle;
            Command::none()
        },
        Message::CaptureDenoiseChanged(denoise) => {
            app.capture_denoise = denoise;
            Command::none()
        },
        Message::CaptureVbitrateChanged(bitrate) => {
            app.capture_vbitrate = bitrate;
            Command::none()
        },
        Message::CaptureCrfChanged(crf) => {
            app.capture_crf = crf;
            Command::none()
        },
        Message::CaptureWidthChanged(width) => {
            app.capture_width = width;
            Command::none()
        },
        Message::CaptureHeightChanged(height) => {
            app.capture_height = height;
            Command::none()
        },
        Message::CaptureFpsChanged(fps) => {
            app.capture_fps = fps;
            Command::none()
        },
        Message::CaptureAbitrateChanged(bitrate) => {
            app.capture_abitrate = bitrate;
            Command::none()
        },
        Message::CaptureArchivalToggled(toggle) => {
            app.capture_archival = toggle;
            Command::none()
        },
        Message::CaptureButtonPressed => {
            println!("Capture button pressed!");
            let output_path = PathBuf::from(&app.capture_output);
            let video_device_clone = app.capture_video_device.clone();
            let audio_device_clone = app.capture_audio_device.clone();
            let format_clone = app.capture_format.clone();
            let deinterlace_clone = app.capture_deinterlace;
            let stabilize_clone = app.capture_stabilize;
            let denoise_clone = if app.capture_denoise.is_empty() {
                None
            } else {
                Some(app.capture_denoise.clone())
            };
            let vbitrate_clone = if app.capture_vbitrate.is_empty() {
                None
            } else {
                Some(app.capture_vbitrate.clone())
            };
            let crf_clone = app.capture_crf.parse::<u32>().ok();
            let width_clone = app.capture_width.parse::<u32>().ok();
            let height_clone = app.capture_height.parse::<u32>().ok();
            let fps_clone = app.capture_fps.parse::<u32>().ok();
            let abitrate_clone = app.capture_abitrate.clone();
            let archival_clone = app.capture_archival;

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
            let current_path = app.capture_output.clone();
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
                app.capture_output = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::CleanInputChanged(path) => {
            app.clean_input = path;
            Command::none()
        },
        Message::CleanMetadataToggled(toggle) => {
            app.clean_metadata = toggle;
            Command::none()
        },
        Message::CleanOptimizeToggled(toggle) => {
            app.clean_optimize = toggle;
            Command::none()
        },
        Message::CleanRecursiveToggled(toggle) => {
            app.clean_recursive = toggle;
            Command::none()
        },
        Message::CleanButtonPressed => {
            println!("Clean button pressed!");
            let input_path = PathBuf::from(&app.clean_input);
            let metadata_clone = app.clean_metadata;
            let optimize_clone = app.clean_optimize;
            let recursive_clone = app.clean_recursive;

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
            let current_path = app.clean_input.clone();
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
                app.clean_input = path_buf.to_string_lossy().into_owned();
            }
            Command::none()
        }
        Message::InfoInputChanged(path) => {
            app.info_input = path;
            Command::none()
        },
        Message::InfoButtonPressed => {
            println!("Info button pressed!");
            let input_path = PathBuf::from(&app.info_input);
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
            let current_path = app.info_input.clone();
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
                app.info_input = path_buf.to_string_lossy().into_owned();
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
            app.is_converting = true;
            app.conversion_progress = 0.0;
            app.status_message = "Conversion started...".to_string();
            Command::none()
        },
        Message::ConversionProgressed(progress) => {
            app.conversion_progress = progress;
            Command::none()
        },
        Message::ConversionCompleted(result) => {
            app.is_converting = false;
            app.conversion_progress = 0.0;
            match result {
                Ok(_) => app.status_message = "Conversion completed successfully!".to_string(),
                Err(e) => app.status_message = format!("Conversion failed: {}", e),
            }
            Command::none()
        },
        Message::UpdateStatus(message) => {
            app.status_message = message;
            Command::none()
        },
        Message::ThemeChanged(theme) => {
            app.current_theme = theme;
            Command::none()
        },
        Message::LanguageChanged(language) => {
            app.current_language = language;
            i18n::set_language(language); // Set the global language
            Command::none()
        },
    }
}
