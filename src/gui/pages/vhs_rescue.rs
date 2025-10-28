use iced::{
    widget::{button, column, row, text, text_input},
    Alignment,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn vhs_rescue_page(
    input: &str,
    output: &str,
    notch: &str,
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
        button(text(translate("start_vhs_rescue_button"))).on_press(Message::VhsRescueButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
