use iced::{
    widget::{button, column, row, text, text_input},
    Alignment,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn vhs_rescue_page(
    input: &str,
    output: &str,
    notch: &str,
    deinterlace: bool,
    stabilize: bool,
    denoise_type: &str,
    sharpen: bool,
    color: bool,
    highpass: &str,
    lowpass: &str,
    compressor: bool,
    gate: bool,
    normalize: bool,
) -> iced::widget::Column<'static, Message> {
    use crate::i18n::translate; // Importa translate localmente

    column![
        text(translate("vhs_rescue_title")).size(30),
        row![
            text_input(&translate("input_file_placeholder"), input)
                .on_input(Message::VhsRescueInputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::VhsRescueBrowseInput),
        ]
        .spacing(10),
        row![
            text_input(&translate("output_file_placeholder"), output)
                .on_input(Message::VhsRescueOutputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::VhsRescueBrowseOutput),
        ]
        .spacing(10),
        text_input(&translate("notch_filter_placeholder"), notch)
            .on_input(Message::VhsRescueNotchChanged)
            .padding(10),
        // Controlli Video
        column![
            text(translate("video_filters_title")).size(20),
            iced::widget::checkbox(translate("deinterlace_checkbox"), deinterlace)
                .on_toggle(Message::VhsRescueDeinterlaceToggled),
            iced::widget::checkbox(translate("stabilize_checkbox"), stabilize)
                .on_toggle(Message::VhsRescueStabilizeToggled),
            row![
                text(translate("denoise_type_label")),
                text_input(&translate("denoise_type_placeholder"), denoise_type)
                    .on_input(Message::VhsRescueDenoiseTypeChanged)
                    .padding(10),
            ].spacing(10).align_items(Alignment::Center),
            iced::widget::checkbox(translate("sharpen_checkbox"), sharpen)
                .on_toggle(Message::VhsRescueSharpenToggled),
            iced::widget::checkbox(translate("color_adjust_checkbox"), color)
                .on_toggle(Message::VhsRescueColorToggled),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
        // Controlli Audio
        column![
            text(translate("audio_filters_title")).size(20),
            row![
                text(translate("highpass_filter_label")),
                text_input(&translate("highpass_filter_placeholder"), highpass)
                    .on_input(Message::VhsRescueHighpassChanged)
                    .padding(10),
            ].spacing(10).align_items(Alignment::Center),
            row![
                text(translate("lowpass_filter_label")),
                text_input(&translate("lowpass_filter_placeholder"), lowpass)
                    .on_input(Message::VhsRescueLowpassChanged)
                    .padding(10),
            ].spacing(10).align_items(Alignment::Center),
            iced::widget::checkbox(translate("compressor_checkbox"), compressor)
                .on_toggle(Message::VhsRescueCompressorToggled),
            iced::widget::checkbox(translate("gate_checkbox"), gate)
                .on_toggle(Message::VhsRescueGateToggled),
            iced::widget::checkbox(translate("normalize_checkbox"), normalize)
                .on_toggle(Message::VhsRescueNormalizeToggled),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
        button(text(translate("start_vhs_rescue_button"))).on_press(Message::VhsRescueButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
