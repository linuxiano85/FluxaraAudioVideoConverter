use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use fluxara_avc::audio::{self, AudioEnhanceOptions};
use fluxara_avc::video::{self, VideoEnhanceOptions, DenoiseType};
use fluxara_avc::capture;
use fluxara_avc::ffmpeg;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Fluxara AVC - Audio/Video Converter",
        options,
        Box::new(|_cc| Box::new(AvcApp::default())),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    AudioEnhance,
    VideoEnhance,
    VhsRescue,
    Capture,
    Convert,
    Info,
}

pub struct AvcApp {
    current_tab: Tab,
    
    // Audio Enhancement
    audio_input: String,
    audio_output: String,
    audio_denoise: bool,
    audio_normalize: bool,
    audio_highpass: u32,
    audio_lowpass: Option<u32>,
    audio_notch: Option<u32>,
    audio_compressor: bool,
    audio_gate: bool,
    audio_gate_threshold: f32,
    
    // Video Enhancement
    video_input: String,
    video_output: String,
    video_deinterlace: bool,
    video_stabilize: bool,
    video_denoise: String,
    video_sharpen: bool,
    video_color: bool,
    video_width: Option<u32>,
    video_height: Option<u32>,
    video_aspect: String,
    
    // VHS Rescue
    vhs_input: String,
    vhs_output: String,
    vhs_notch: Option<u32>,
    
    // Capture
    capture_output: String,
    capture_video_device: String,
    capture_audio_device: String,
    capture_format: String,
    capture_deinterlace: bool,
    capture_stabilize: bool,
    capture_denoise: String,
    capture_width: Option<u32>,
    capture_height: Option<u32>,
    capture_fps: Option<u32>,
    capture_audio_bitrate: String,
    capture_archival: bool,
    
    // Convert
    convert_input: String,
    convert_output: String,
    convert_format: String,
    convert_quality: String,
    convert_codec: String,
    convert_recursive: bool,
    
    // Info
    info_input: String,
    info_output: String,
    
    // Status
    status_message: String,
    is_processing: Arc<Mutex<bool>>,
}

impl Default for AvcApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::AudioEnhance,
            
            audio_input: String::new(),
            audio_output: String::new(),
            audio_denoise: true,
            audio_normalize: true,
            audio_highpass: 80,
            audio_lowpass: None,
            audio_notch: None,
            audio_compressor: true,
            audio_gate: true,
            audio_gate_threshold: -50.0,
            
            video_input: String::new(),
            video_output: String::new(),
            video_deinterlace: true,
            video_stabilize: false,
            video_denoise: "hqdn3d".to_string(),
            video_sharpen: true,
            video_color: true,
            video_width: None,
            video_height: None,
            video_aspect: "16:9".to_string(),
            
            vhs_input: String::new(),
            vhs_output: String::new(),
            vhs_notch: None,
            
            capture_output: String::new(),
            capture_video_device: "/dev/video0".to_string(),
            capture_audio_device: "hw:0,0".to_string(),
            capture_format: "mp4".to_string(),
            capture_deinterlace: true,
            capture_stabilize: false,
            capture_denoise: "hqdn3d".to_string(),
            capture_width: Some(720),
            capture_height: Some(480),
            capture_fps: Some(30),
            capture_audio_bitrate: "192k".to_string(),
            capture_archival: false,
            
            convert_input: String::new(),
            convert_output: String::new(),
            convert_format: "mp4".to_string(),
            convert_quality: "192k".to_string(),
            convert_codec: "libx264".to_string(),
            convert_recursive: false,
            
            info_input: String::new(),
            info_output: String::new(),
            
            status_message: "Ready".to_string(),
            is_processing: Arc::new(Mutex::new(false)),
        }
    }
}

impl eframe::App for AvcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎬 Fluxara AVC - Audio/Video Converter");
            ui.separator();

            // Tab selector
            ui.horizontal(|ui| {
                if ui.selectable_label(self.current_tab == Tab::AudioEnhance, "🎵 Audio Enhance").clicked() {
                    self.current_tab = Tab::AudioEnhance;
                }
                if ui.selectable_label(self.current_tab == Tab::VideoEnhance, "🎥 Video Enhance").clicked() {
                    self.current_tab = Tab::VideoEnhance;
                }
                if ui.selectable_label(self.current_tab == Tab::VhsRescue, "📼 VHS Rescue").clicked() {
                    self.current_tab = Tab::VhsRescue;
                }
                if ui.selectable_label(self.current_tab == Tab::Capture, "📹 Capture").clicked() {
                    self.current_tab = Tab::Capture;
                }
                if ui.selectable_label(self.current_tab == Tab::Convert, "🔄 Convert").clicked() {
                    self.current_tab = Tab::Convert;
                }
                if ui.selectable_label(self.current_tab == Tab::Info, "ℹ️ Info").clicked() {
                    self.current_tab = Tab::Info;
                }
            });

            ui.separator();

            // Content based on tab
            match self.current_tab {
                Tab::AudioEnhance => self.show_audio_enhance(ui),
                Tab::VideoEnhance => self.show_video_enhance(ui),
                Tab::VhsRescue => self.show_vhs_rescue(ui),
                Tab::Capture => self.show_capture(ui),
                Tab::Convert => self.show_convert(ui),
                Tab::Info => self.show_info(ui),
            }

            ui.separator();
            ui.label(format!("Status: {}", self.status_message));
        });
    }
}

impl AvcApp {
    fn show_audio_enhance(&mut self, ui: &mut egui::Ui) {
        ui.label("Audio Enhancement");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Input File:");
            ui.text_edit_singleline(&mut self.audio_input);
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.audio_input = path.to_string_lossy().to_string();
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("Output File:");
            ui.text_edit_singleline(&mut self.audio_output);
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    self.audio_output = path.to_string_lossy().to_string();
                }
            }
        });

        ui.separator();
        ui.label("Enhancement Options:");

        ui.checkbox(&mut self.audio_denoise, "Denoise (afftdn)");
        ui.checkbox(&mut self.audio_normalize, "Normalize (EBU R128)");
        ui.checkbox(&mut self.audio_compressor, "Compressor");
        ui.checkbox(&mut self.audio_gate, "Noise Gate");

        ui.horizontal(|ui| {
            ui.label("High-pass Filter (Hz):");
            ui.add(egui::Slider::new(&mut self.audio_highpass, 20..=200));
        });

        ui.horizontal(|ui| {
            ui.label("Low-pass Filter (Hz):");
            let mut lowpass_enabled = self.audio_lowpass.is_some();
            if ui.checkbox(&mut lowpass_enabled, "Enable").clicked() {
                if lowpass_enabled && self.audio_lowpass.is_none() {
                    self.audio_lowpass = Some(12000);
                } else if !lowpass_enabled {
                    self.audio_lowpass = None;
                }
            }
            if let Some(ref mut freq) = self.audio_lowpass {
                ui.add(egui::Slider::new(freq, 5000..=20000));
            }
        });

        ui.horizontal(|ui| {
            ui.label("Notch Filter (Hz):");
            let mut notch_enabled = self.audio_notch.is_some();
            if ui.checkbox(&mut notch_enabled, "Enable").clicked() {
                if notch_enabled && self.audio_notch.is_none() {
                    self.audio_notch = Some(50);
                    } else if !notch_enabled {
                        self.audio_notch = None;
                    }
                }
                if let Some(ref mut freq) = self.audio_notch {
                    ui.add(egui::Slider::new(freq, 40..=60));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Gate Threshold (dB):");
                ui.add(egui::Slider::new(&mut self.audio_gate_threshold, -80.0..=-10.0));
            });

            ui.separator();

            if ui.button("▶ Enhance Audio").clicked() {
                self.enhance_audio();
            }
        }

        fn show_video_enhance(&mut self, ui: &mut egui::Ui) {
            ui.label("Video Enhancement");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Input File:");
                ui.text_edit_singleline(&mut self.video_input);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.video_input = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Output File:");
                ui.text_edit_singleline(&mut self.video_output);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.video_output = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.separator();
            ui.label("Enhancement Options:");

            ui.checkbox(&mut self.video_deinterlace, "Deinterlace (bwdif)");
            ui.checkbox(&mut self.video_stabilize, "Stabilize (deshake)");
            ui.checkbox(&mut self.video_sharpen, "Sharpen (unsharp)");
            ui.checkbox(&mut self.video_color, "Color Adjustment");

            ui.horizontal(|ui| {
                ui.label("Denoise Type:");
                ui.selectable_value(&mut self.video_denoise, "none".to_string(), "None");
                ui.selectable_value(&mut self.video_denoise, "hqdn3d".to_string(), "HQDN3D");
                ui.selectable_value(&mut self.video_denoise, "nlmeans".to_string(), "NLMeans");
            });

            ui.horizontal(|ui| {
                ui.label("Width:");
                let mut width_enabled = self.video_width.is_some();
                if ui.checkbox(&mut width_enabled, "Enable").clicked() {
                    if width_enabled && self.video_width.is_none() {
                        self.video_width = Some(1920);
                    } else if !width_enabled {
                        self.video_width = None;
                    }
                }
                if let Some(ref mut w) = self.video_width {
                    ui.add(egui::Slider::new(w, 320..=4096));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Height:");
                let mut height_enabled = self.video_height.is_some();
                if ui.checkbox(&mut height_enabled, "Enable").clicked() {
                    if height_enabled && self.video_height.is_none() {
                        self.video_height = Some(1080);
                    } else if !height_enabled {
                        self.video_height = None;
                    }
                }
                if let Some(ref mut h) = self.video_height {
                    ui.add(egui::Slider::new(h, 240..=2160));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Aspect Ratio:");
                ui.text_edit_singleline(&mut self.video_aspect);
            });

            ui.separator();

            if ui.button("▶ Enhance Video").clicked() {
                self.enhance_video();
            }
        }

        fn show_vhs_rescue(&mut self, ui: &mut egui::Ui) {
            ui.label("VHS Rescue - One-Click Restoration");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Input File:");
                ui.text_edit_singleline(&mut self.vhs_input);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.vhs_input = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Output File:");
                ui.text_edit_singleline(&mut self.vhs_output);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.vhs_output = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.separator();
            ui.label("Options:");

            ui.horizontal(|ui| {
                ui.label("Notch Filter (Hz):");
                let mut notch_enabled = self.vhs_notch.is_some();
                if ui.checkbox(&mut notch_enabled, "Enable").clicked() {
                    if notch_enabled && self.vhs_notch.is_none() {
                        self.vhs_notch = Some(50);
                    } else if !notch_enabled {
                        self.vhs_notch = None;
                    }
                }
                if let Some(ref mut freq) = self.vhs_notch {
                    ui.add(egui::Slider::new(freq, 40..=60));
                }
            });

            ui.separator();
            ui.label("VHS Rescue applies:");
            ui.label("• Video: Deinterlace, Stabilize, Denoise, Sharpen, Color Adjust");
            ui.label("• Audio: Denoise, Normalize, Compressor, Gate");

            ui.separator();

            if ui.button("▶ Start VHS Rescue").clicked() {
                self.vhs_rescue();
            }
        }

        fn capture(&mut self) {
            if self.capture_output.is_empty() {
                self.status_message = "Error: Output file required".to_string();
                return;
            }

            let output = PathBuf::from(self.capture_output.clone());
            let opts = capture::CaptureOptions {
                format: if self.capture_format == "mkv" {
                    capture::CaptureFormat::Mkv
                } else {
                    capture::CaptureFormat::Mp4
                },
                video_device: self.capture_video_device.clone(),
                audio_device: self.capture_audio_device.clone(),
                deinterlace: self.capture_deinterlace,
                stabilize: self.capture_stabilize,
                denoise: if self.capture_denoise.is_empty() {
                    None
                } else {
                    Some(self.capture_denoise.clone())
                },
                video_bitrate: None,
                crf: Some(23),
                width: self.capture_width,
                height: self.capture_height,
                fps: self.capture_fps,
                audio_bitrate: self.capture_audio_bitrate.clone(),
                archival_mode: self.capture_archival,
            };

            self.status_message = "Starting capture...".to_string();
            let is_processing = Arc::clone(&self.is_processing);

            thread::spawn(move || {
                *is_processing.lock().unwrap() = true;
                match capture::capture(&output, &opts) {
                    Ok(_) => println!("Capture completed!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
                *is_processing.lock().unwrap() = false;
            });

            self.status_message = "Capture started!".to_string();
        }

        fn convert(&mut self) {
            if self.convert_input.is_empty() || self.convert_output.is_empty() {
                self.status_message = "Error: Input and output paths required".to_string();
                return;
            }

            let input = PathBuf::from(self.convert_input.clone());
            let output = PathBuf::from(self.convert_output.clone());
            let format = self.convert_format.clone();
            let quality = self.convert_quality.clone();
            let codec = self.convert_codec.clone();
            let recursive = self.convert_recursive;

            self.status_message = "Starting conversion...".to_string();
            let is_processing = Arc::clone(&self.is_processing);

            thread::spawn(move || {
                *is_processing.lock().unwrap() = true;
                match ffmpeg::convert_media(
                    &input,
                    &output,
                    &format,
                    &quality,
                    &codec,
                    recursive,
                ) {
                    Ok(_) => println!("Conversion completed!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
                *is_processing.lock().unwrap() = false;
            });

            self.status_message = "Conversion started!".to_string();
        }

        fn get_info(&mut self) {
            if self.info_input.is_empty() {
                self.info_output = "Error: Input file required".to_string();
                return;
            }

            let input = PathBuf::from(self.info_input.clone());
            match ffmpeg::get_media_info(&input) {
                Ok(info) => {
                    self.info_output = serde_json::to_string_pretty(&info).unwrap_or_default();
                }
                Err(e) => {
                    self.info_output = format!("Error: {}", e);
                }
            }
        }
    }
