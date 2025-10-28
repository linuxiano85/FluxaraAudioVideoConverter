use iced::{
    widget::{column, row, text, PickList, radio, Column},
    Alignment, Length,
};
use crate::gui::{Message, AppTheme};
use crate::i18n::Language;

pub fn settings_page(current_theme: AppTheme, current_language: Language) -> Column<'static, Message> {
    column![
        text("Settings").size(30),
        row![
            text("Theme:").size(20),
            radio(
                "Light",
                AppTheme::Light,
                Some(current_theme),
                Message::ThemeChanged,
            )
            .spacing(5),
            radio(
                "Dark",
                AppTheme::Dark,
                Some(current_theme),
                Message::ThemeChanged,
            )
            .spacing(5),
            radio(
                "Gaming",
                AppTheme::Gaming,
                Some(current_theme),
                Message::ThemeChanged,
            )
            .spacing(5),
        ]
        .spacing(20)
        .align_items(Alignment::Center),
        row![
            text("Language:").size(20),
            PickList::new(
                &[Language::En, Language::It][..],
                Some(current_language),
                Message::LanguageChanged,
            )
            .padding(10)
            .width(Length::Fixed(150.0)),
        ]
        .spacing(20)
        .align_items(Alignment::Center),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
