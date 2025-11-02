use iced::{
    widget::{button, column, row, text, text_input},
    Alignment,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn info_page(input: &str) -> iced::widget::Column<'static, Message> {
    column![
        text("Media File Information").size(30),
        row![
            text_input("Input File", input)
                .on_input(Message::InfoInputChanged)
                .padding(10),
            button("Browse").on_press(Message::InfoBrowseInput),
        ]
        .spacing(10),
        button("Get Info").on_press(Message::InfoButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
