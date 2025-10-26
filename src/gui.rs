use iced::{
    widget::{button, column, row, text, Space, text_input, checkbox, Column}, // Import Column directly
    theme, Alignment, Element, Length, Sandbox, Settings, Theme, Color,
};
use std::fmt;
use std::path::PathBuf; // Import PathBuf
use crate::convert_files; // Import convert_files from main.rs

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
    // Convert page state
    convert_input_path: String,
    convert_output_format: String,
    convert_output_dir: String,
    convert_recursive: bool,
    convert_quality: String,
    convert_codec: String,
    convert_jobs: String,
}

#[derive(Debug, Clone)] // Changed to Clone to allow String cloning
enum Message {
    PageChanged(Page),
    // Convert page messages
    ConvertInputPathChanged(String),
    ConvertOutputFormatChanged(String),
    ConvertOutputDirChanged(String),
    ConvertRecursiveToggled(bool),
    ConvertQualityChanged(String),
    ConvertCodecChanged(String),
    ConvertJobsChanged(String),
    ConvertButtonPressed,
    EnhanceAudioButtonPressed,
    EnhanceVideoButtonPressed,
    VhsRescueButtonPressed,
    CaptureButtonPressed,
    CleanButtonPressed,
    InfoButtonPressed,
    FormatsButtonPressed,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            current_page: Page::Home,
            convert_input_path: String::new(),
            convert_output_format: "mp4".to_string(),
            convert_output_dir: String::new(),
            convert_recursive: false,
            convert_quality: "192k".to_string(),
            convert_codec: String::new(),
            convert_jobs: "4".to_string(),
        }
    }

    fn title(&self) -> String {
        format!("Fluxara AVC - {}", self.current_page)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PageChanged(page) => self.current_page = page,
            // Convert page updates
            Message::ConvertInputPathChanged(path) => self.convert_input_path = path,
            Message::ConvertOutputFormatChanged(format) => self.convert_output_format = format,
            Message::ConvertOutputDirChanged(dir) => self.convert_output_dir = dir,
            Message::ConvertRecursiveToggled(toggle) => self.convert_recursive = toggle,
            Message::ConvertQualityChanged(quality) => self.convert_quality = quality,
            Message::ConvertCodecChanged(codec) => self.convert_codec = codec,
            Message::ConvertJobsChanged(jobs) => self.convert_jobs = jobs,
            Message::ConvertButtonPressed => {
                println!("Convert button pressed!");
                // Example of calling the CLI function (needs to be async and handle Result)
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

                // This needs to be spawned as a tokio task
                let format_clone = self.convert_output_format.clone();
                let quality_clone = self.convert_quality.clone();
                let recursive_clone = self.convert_recursive; // Clone recursive
                tokio::spawn(async move {
                    match convert_files(
                        &input_path,
                        &format_clone,
                        output_dir.as_ref(),
                        recursive_clone, // Use the cloned value
                        &quality_clone,
                        codec_option.as_ref(),
                        jobs_parsed,
                    )
                    .await
                    {
                        Ok(_) => println!("Conversion successful!"),
                        Err(e) => eprintln!("Conversion failed: {:?}", e),
                    }
                });
            }
            Message::EnhanceAudioButtonPressed => println!("Enhance Audio button pressed!"),
            Message::EnhanceVideoButtonPressed => println!("Enhance Video button pressed!"),
            Message::VhsRescueButtonPressed => println!("VHS Rescue button pressed!"),
            Message::CaptureButtonPressed => println!("Capture button pressed!"),
            Message::CleanButtonPressed => println!("Clean button pressed!"),
            Message::InfoButtonPressed => println!("Info button pressed!"),
            Message::FormatsButtonPressed => println!("Formats button pressed!"),
        }
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
            Page::EnhanceAudio => enhance_audio_page(),
            Page::EnhanceVideo => enhance_video_page(),
            Page::VhsRescue => vhs_rescue_page(),
            Page::Capture => capture_page(),
            Page::Clean => clean_page(),
            Page::Info => info_page(),
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

    fn theme(&self) -> Theme {
        Theme::Dark
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
        text("Welcome to Fluxara AVC!").size(40).style(Color::from_rgb8(0x00, 0xFF, 0x00)), // Green RGB
        text("Your Linux-first analog restoration & conversion tool.").size(25).style(Color::from_rgb8(0x00, 0xCC, 0xFF)), // Cyan RGB
        Space::with_height(Length::Fixed(50.0)),
        text("Select a function from the sidebar to get started.").size(20).style(Color::from_rgb8(0xFF, 0xFF, 0xFF)), // White
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

// Placeholder pages for each functionality
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
        text("Convert Files").size(30),
        text_input("Input File or Directory", input_path)
            .on_input(Message::ConvertInputPathChanged)
            .padding(10),
        text_input("Output Format (mp4, mp3, etc.)", output_format)
            .on_input(Message::ConvertOutputFormatChanged)
            .padding(10),
        text_input("Output Directory (optional)", output_dir)
            .on_input(Message::ConvertOutputDirChanged)
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

fn enhance_audio_page() -> Column<'static, Message> {
    column![
        text("Enhance Audio").size(30),
        button("Start Audio Enhancement").on_press(Message::EnhanceAudioButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn enhance_video_page() -> Column<'static, Message> {
    column![
        text("Enhance Video").size(30),
        button("Start Video Enhancement").on_press(Message::EnhanceVideoButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn vhs_rescue_page() -> Column<'static, Message> {
    column![
        text("VHS Rescue").size(30),
        button("Start VHS Rescue").on_press(Message::VhsRescueButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn capture_page() -> Column<'static, Message> {
    column![
        text("Capture Devices").size(30),
        button("List Devices").on_press(Message::CaptureButtonPressed),
        button("Start Capture").on_press(Message::CaptureButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn clean_page() -> Column<'static, Message> {
    column![
        text("Clean Media Files").size(30),
        button("Start Cleaning").on_press(Message::CleanButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}

fn info_page() -> Column<'static, Message> {
    column![
        text("Media File Information").size(30),
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
