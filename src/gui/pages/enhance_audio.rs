use iced::{
    widget::{button, column, row, text, text_input, checkbox},
    Alignment,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn enhance_audio_page(
    input: &str,
    output: &str,
    denoise: bool,
    normalize: bool,
    highpass: &str,
    lowpass: &str,
    notch: &str,
    compressor: bool,
    gate: bool,
    gate_threshold: &str, // Aggiunto il nuovo argomento
    audio_only: bool,
) -> iced::widget::Column<'static, Message> {
    use crate::i18n::translate; // Importa translate localmente

    column![
        text(translate("enhance_audio_title")).size(30),
        row![
            text_input(&translate("input_file_placeholder"), input)
                .on_input(Message::EnhanceAudioInputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::EnhanceAudioBrowseInput),
        ]
        .spacing(10),
        row![
            text_input(&translate("output_file_placeholder"), output)
                .on_input(Message::EnhanceAudioOutputChanged)
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::EnhanceAudioBrowseOutput),
        ]
        .spacing(10),
        checkbox(translate("enable_denoising_checkbox"), denoise)
            .on_toggle(Message::EnhanceAudioDenoiseToggled),
        checkbox(translate("enable_loudness_normalization_checkbox"), normalize)
            .on_toggle(Message::EnhanceAudioNormalizeToggled),
        text_input(&translate("highpass_filter_frequency_placeholder"), highpass)
            .on_input(Message::EnhanceAudioHighpassChanged)
            .padding(10),
        text_input(&translate("lowpass_filter_frequency_placeholder"), lowpass)
            .on_input(Message::EnhanceAudioLowpassChanged)
            .padding(10),
        text_input(&translate("notch_filter_placeholder"), notch)
            .on_input(Message::EnhanceAudioNotchChanged)
            .padding(10),
        checkbox(translate("enable_compressor_checkbox"), compressor)
            .on_toggle(Message::EnhanceAudioCompressorToggled),
        checkbox(translate("enable_noise_gate_checkbox"), gate)
            .on_toggle(Message::EnhanceAudioGateToggled),
        text_input(&translate("noise_gate_threshold_placeholder"), gate_threshold) // Nuovo input per la soglia del gate
            .on_input(Message::EnhanceAudioGateThresholdChanged)
            .padding(10),
        checkbox(translate("process_audio_only_checkbox"), audio_only)
            .on_toggle(Message::EnhanceAudioOnlyToggled),
        button(text(translate("start_audio_enhancement_button"))).on_press(Message::EnhanceAudioButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
