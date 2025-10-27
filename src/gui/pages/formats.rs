use iced::{
    widget::{button, column, text},
    Alignment, Length, Element,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn formats_page() -> iced::widget::Column<'static, Message> {
    column![
        text("Supported Formats").size(30),
        button("List Formats").on_press(Message::FormatsButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
