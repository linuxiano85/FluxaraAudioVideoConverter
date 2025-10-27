//! Modulo per la definizione dei temi dell'applicazione.

use iced::{
    widget::{self, button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text_input},
    Color, Theme, Border, Background,
};
use std::fmt;
use std::sync::Arc;

/// Enum per la selezione del tema dell'applicazione.
/// `Default` è implementato per impostare `Dark` come tema predefinito.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppTheme {
    Light,
    #[default]
    Dark,
    Gaming, // Nuovo tema gaming
}

impl fmt::Display for AppTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Implementazioni dei trait StyleSheet per AppTheme

impl widget::checkbox::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => iced::theme::Checkbox::default().active(style, is_checked),
            AppTheme::Dark => iced::theme::Checkbox::default().active(style, is_checked),
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                icon_color: Color::WHITE,
                text_color: Some(Color::WHITE),
            },
        }
    }

    fn hovered(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => iced::theme::Checkbox::default().hovered(style, is_checked),
            AppTheme::Dark => iced::theme::Checkbox::default().hovered(style, is_checked),
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 0.5),
                },
                icon_color: Color::WHITE,
                text_color: Some(Color::WHITE),
            },
        }
    }
}

impl widget::progress_bar::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> widget::progress_bar::Appearance {
        match self {
            AppTheme::Light => iced::theme::ProgressBar::default().appearance(style),
            AppTheme::Dark => iced::theme::ProgressBar::default().appearance(style),
            AppTheme::Gaming => widget::progress_bar::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                bar: Background::Color(Color::from_rgb(0.0, 1.0, 0.0)),
                border_radius: 5.0.into(),
            },
        }
    }
}

impl From<AppTheme> for Theme {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
            AppTheme::Gaming => Theme::Dark, // You might want a custom theme for Gaming
        }
    }
}

impl From<AppTheme> for Color {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Color::BLACK,
            AppTheme::Dark => Color::WHITE,
            AppTheme::Gaming => Color::from_rgb(0.0, 1.0, 0.0), // Green for gaming
        }
    }
}
