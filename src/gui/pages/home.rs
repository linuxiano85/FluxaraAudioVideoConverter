use iced::{
    widget::{column, text, Space},
    Alignment, Color, Length, Element,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn home_page() -> iced::widget::Column<'static, Message> {
    column![
        text("Welcome to Fluxara AVC!").size(40).style(Color::from_rgb8(0x00, 0xFF, 0x00)),
        text("Your Linux-first analog restoration & conversion tool.").size(25).style(Color::from_rgb8(0x00, 0xCC, 0xFF)),
        Space::with_height(Length::Fixed(50.0)),
        text("Select a function from the sidebar to get started.").size(20).style(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
