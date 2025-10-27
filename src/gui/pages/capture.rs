use iced::{
    widget::{button, column, row, text, text_input, checkbox, pick_list}, // Aggiunto pick_list
    Alignment, Length, Element,
};
use crate::gui::Message; // Importa Message dal modulo genitore
use std::sync::Arc; // Importa Arc

pub fn capture_page(
    output: &str,
    video_device: &str,
    video_devices: &Arc<Vec<String>>, // Nuovi argomenti
    audio_device: &str,
    audio_devices: &Arc<Vec<String>>, // Nuovi argomenti
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
) -> iced::widget::Column<'static, Message> {
    use crate::i18n::translate; // Importa translate localmente

    column![
        text(translate("capture_devices_title")).size(30),
        row![
            text_input(&translate("output_file_placeholder"), output)
                .on_input(Message::CaptureOutputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::CaptureBrowseOutput),
        ]
        .spacing(10),
        pick_list(video_devices.as_ref().to_vec(), Some(video_device.to_string()), Message::CaptureVideoDeviceChanged)
            .padding(10)
            .placeholder(translate("video_device_placeholder")),
        pick_list(audio_devices.as_ref().to_vec(), Some(audio_device.to_string()), Message::CaptureAudioDeviceChanged)
            .padding(10)
            .placeholder(translate("audio_device_placeholder")),
        text_input(&translate("output_format_placeholder"), format)
            .on_input(Message::CaptureFormatChanged)
            .padding(10),
        checkbox(translate("enable_deinterlacing_checkbox"), deinterlace)
            .on_toggle(Message::CaptureDeinterlaceToggled),
        checkbox(translate("enable_stabilization_checkbox"), stabilize)
            .on_toggle(Message::CaptureStabilizeToggled),
        text_input(&translate("denoise_type_placeholder"), denoise)
            .on_input(Message::CaptureDenoiseChanged)
            .padding(10),
        text_input(&translate("video_bitrate_placeholder"), vbitrate)
            .on_input(Message::CaptureVbitrateChanged)
            .padding(10),
        text_input(&translate("crf_value_placeholder"), crf)
            .on_input(Message::CaptureCrfChanged)
            .padding(10),
        text_input(&translate("video_width_placeholder"), width)
            .on_input(Message::CaptureWidthChanged)
            .padding(10),
        text_input(&translate("video_height_placeholder"), height)
            .on_input(Message::CaptureHeightChanged)
            .padding(10),
        text_input(&translate("frame_rate_placeholder"), fps)
            .on_input(Message::CaptureFpsChanged)
            .padding(10),
        text_input(&translate("audio_bitrate_placeholder"), abitrate)
            .on_input(Message::CaptureAbitrateChanged)
            .padding(10),
        checkbox(translate("archival_mode_checkbox"), archival)
            .on_toggle(Message::CaptureArchivalToggled),
        button(text(translate("start_capture_button"))).on_press(Message::CaptureButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
