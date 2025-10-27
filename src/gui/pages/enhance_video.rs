use iced::{
    widget::{button, column, row, text, text_input, checkbox},
    Alignment, Length, Element,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn enhance_video_page(
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
) -> iced::widget::Column<'static, Message> {
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
