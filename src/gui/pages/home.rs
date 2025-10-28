use iced::{
    widget::{column, text, Space},
    Alignment, Color, Length,
};
use crate::gui::Message; // Importa Message dal modulo genitore

use crate::i18n::translate;

pub fn home_page() -> iced::widget::Column<'static, Message> {
    column![
        text(translate("welcome_title")).size(40).style(Color::from_rgb8(0x00, 0xFF, 0x00)),
        text(translate("welcome_subtitle")).size(25).style(Color::from_rgb8(0x00, 0xCC, 0xFF)),
        Space::with_height(Length::Fixed(50.0)),
        text(translate("welcome_instruction")).size(20).style(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
