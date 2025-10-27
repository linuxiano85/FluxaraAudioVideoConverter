use iced::{
    widget::{button, column, row, text, text_input},
    Alignment, Length, Element,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn vhs_rescue_page(
    input: &str,
    output: &str,
    notch: &str,
) -> iced::widget::Column<'static, Message> {
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
