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
    use crate::i18n::translate; // Importa translate localmente

    column![
        text(translate("enhance_video_title")).size(30),
        row![
            text_input(&translate("input_file_placeholder"), input)
                .on_input(Message::EnhanceVideoInputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::EnhanceVideoBrowseInput),
        ]
        .spacing(10),
        row![
            text_input(&translate("output_file_placeholder"), output)
                .on_input(Message::EnhanceVideoOutputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::EnhanceVideoBrowseOutput),
        ]
        .spacing(10),
        checkbox(translate("enable_deinterlacing_checkbox"), deinterlace)
            .on_toggle(Message::EnhanceVideoDeinterlaceToggled),
        checkbox(translate("enable_stabilization_checkbox"), stabilize)
            .on_toggle(Message::EnhanceVideoStabilizeToggled),
        text_input(&translate("denoise_type_placeholder"), denoise_type)
            .on_input(Message::EnhanceVideoDenoiseTypeChanged)
            .padding(10),
        checkbox(translate("enable_sharpening_checkbox"), sharpen)
            .on_toggle(Message::EnhanceVideoSharpenToggled),
        checkbox(translate("enable_color_adjustment_checkbox"), color)
            .on_toggle(Message::EnhanceVideoColorToggled),
        text_input(&translate("scale_width_placeholder"), width)
            .on_input(Message::EnhanceVideoWidthChanged)
            .padding(10),
        text_input(&translate("scale_height_placeholder"), height)
            .on_input(Message::EnhanceVideoHeightChanged)
            .padding(10),
        text_input(&translate("display_aspect_ratio_placeholder"), aspect)
            .on_input(Message::EnhanceVideoAspectChanged)
            .padding(10),
        button(text(translate("start_video_enhancement_button"))).on_press(Message::EnhanceVideoButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
