//! Modulo per la definizione dei temi dell'applicazione.

use iced::{
    widget,
    Color, Theme, Border, Background,
};
use iced::widget::{button, container, text, text_input, pick_list, slider};
use std::fmt;

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

    fn active(&self, _style: &Self::Style, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.active(&iced::theme::Checkbox::default(), is_checked),
            AppTheme::Dark => iced::Theme::Dark.active(&iced::theme::Checkbox::default(), is_checked),
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

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.hovered(&iced::theme::Checkbox::default(), is_checked),
            AppTheme::Dark => iced::Theme::Dark.hovered(&iced::theme::Checkbox::default(), is_checked),
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

    fn appearance(&self, _style: &Self::Style) -> widget::progress_bar::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.appearance(&iced::theme::ProgressBar::default()),
            AppTheme::Dark => iced::Theme::Dark.appearance(&iced::theme::ProgressBar::default()),
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
            AppTheme::Gaming => Theme::Dark, // Use Dark theme as base, then override styles
        }
    }
}

impl From<AppTheme> for Color {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Color::BLACK,
            AppTheme::Dark => Color::WHITE,
            AppTheme::Gaming => Color::from_rgb(0.0, 0.8, 1.0), // Neon blue for text
        }
    }
}

impl From<&AppTheme> for iced::theme::Button {
    fn from(app_theme: &AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => iced::theme::Button::Primary,
            AppTheme::Dark => iced::theme::Button::Primary,
            AppTheme::Gaming => iced::theme::Button::Primary, // Use Primary as a base, custom styles are in StyleSheet
        }
    }
}

impl button::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.active(&iced::theme::Button::Primary),
            AppTheme::Dark => iced::Theme::Dark.active(&iced::theme::Button::Primary),
            AppTheme::Gaming => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.0, 0.4, 0.0))), // Darker green
                border: Border {
                    radius: 5.0.into(),
                    width: 2.0,
                    color: Color::from_rgb(0.0, 0.8, 0.0), // Bright green border
                },
                text_color: Color::WHITE,
                shadow_offset: iced::Vector::new(1.0, 1.0),
                shadow: iced::Shadow::default(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.hovered(&iced::theme::Button::Primary),
            AppTheme::Dark => iced::Theme::Dark.hovered(&iced::theme::Button::Primary),
            AppTheme::Gaming => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.0, 0.6, 0.0))), // Lighter green on hover
                border: Border {
                    radius: 5.0.into(),
                    width: 2.0,
                    color: Color::from_rgb(0.0, 1.0, 0.0), // Even brighter green border
                },
                text_color: Color::WHITE,
                shadow_offset: iced::Vector::new(2.0, 2.0),
                shadow: iced::Shadow::default(),
            },
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.pressed(&iced::theme::Button::Primary),
            AppTheme::Dark => iced::Theme::Dark.pressed(&iced::theme::Button::Primary),
            AppTheme::Gaming => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.0, 0.8, 0.0))), // Brightest green on press
                border: Border {
                    radius: 5.0.into(),
                    width: 2.0,
                    color: Color::from_rgb(0.0, 1.0, 0.0),
                },
                text_color: Color::WHITE,
                shadow_offset: iced::Vector::new(0.0, 0.0),
                shadow: iced::Shadow::default(),
            },
        }
    }
}

impl container::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.appearance(&iced::theme::Container::default()),
            AppTheme::Dark => iced::Theme::Dark.appearance(&iced::theme::Container::default()),
            AppTheme::Gaming => container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.05, 0.05, 0.1))), // Dark, almost black background
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                text_color: Some(Color::from_rgb(0.0, 1.0, 0.0)), // Bright green text
                shadow: iced::Shadow::default(),
            },
        }
    }
}

impl text::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match self {
            AppTheme::Light => style.appearance(iced::theme::Text::default()),
            AppTheme::Dark => style.appearance(iced::theme::Text::default()),
            AppTheme::Gaming => text::Appearance {
                color: Some(Color::from_rgb(0.0, 0.8, 1.0)), // Blu neon per il testo
            },
        }
    }
}

impl text_input::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.active(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.active(&iced::theme::TextInput::default()),
            AppTheme::Gaming => text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.15)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.0, 0.6, 0.8), // Blu più scuro
                },
                icon_color: Color::from_rgb(0.0, 0.8, 1.0),
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.focused(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.focused(&iced::theme::TextInput::default()),
            AppTheme::Gaming => text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.15)),
                border: Border {
                    radius: 5.0.into(),
                    width: 2.0,
                    color: Color::from_rgb(0.0, 0.8, 1.0), // Blu neon
                },
                icon_color: Color::from_rgb(0.0, 0.8, 1.0),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.hovered(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.hovered(&iced::theme::TextInput::default()),
            AppTheme::Gaming => text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.0, 0.7, 0.9), // Blu leggermente più chiaro
                },
                icon_color: Color::from_rgb(0.0, 0.8, 1.0),
            },
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        match self {
            AppTheme::Light => iced::Theme::Light.placeholder_color(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.placeholder_color(&iced::theme::TextInput::default()),
            AppTheme::Gaming => Color::from_rgb(0.3, 0.5, 0.6), // Grigio-blu per il placeholder
        }
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        match self {
            AppTheme::Light => iced::Theme::Light.value_color(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.value_color(&iced::theme::TextInput::default()),
            AppTheme::Gaming => Color::from_rgb(0.0, 0.8, 1.0), // Blu neon per il valore
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        match self {
            AppTheme::Light => iced::Theme::Light.disabled_color(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.disabled_color(&iced::theme::TextInput::default()),
            AppTheme::Gaming => Color::from_rgb(0.2, 0.2, 0.25),
        }
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        match self {
            AppTheme::Light => iced::Theme::Light.selection_color(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.selection_color(&iced::theme::TextInput::default()),
            AppTheme::Gaming => Color::from_rgb(0.0, 0.4, 0.6),
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.disabled(&iced::theme::TextInput::default()),
            AppTheme::Dark => iced::Theme::Dark.disabled(&iced::theme::TextInput::default()),
            AppTheme::Gaming => text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.05, 0.05, 0.1)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.1, 0.1, 0.15),
                },
                icon_color: Color::from_rgb(0.1, 0.1, 0.15),
            },
        }
    }
}

impl pick_list::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.active(&iced::theme::PickList::default()),
            AppTheme::Dark => iced::Theme::Dark.active(&iced::theme::PickList::default()),
            AppTheme::Gaming => pick_list::Appearance {
                text_color: Color::from_rgb(0.0, 0.8, 1.0),
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.15)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.0, 0.6, 0.8),
                },
                placeholder_color: Color::from_rgb(0.3, 0.5, 0.6),
                handle_color: Color::from_rgb(0.0, 0.8, 1.0),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> pick_list::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.hovered(&iced::theme::PickList::default()),
            AppTheme::Dark => iced::Theme::Dark.hovered(&iced::theme::PickList::default()),
            AppTheme::Gaming => pick_list::Appearance {
                text_color: Color::from_rgb(0.0, 0.9, 1.0),
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.0, 0.7, 0.9),
                },
                placeholder_color: Color::from_rgb(0.3, 0.5, 0.6),
                handle_color: Color::from_rgb(0.0, 0.9, 1.0),
            },
        }
    }
}

impl slider::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.active(&iced::theme::Slider::default()),
            AppTheme::Dark => iced::Theme::Dark.active(&iced::theme::Slider::default()),
            AppTheme::Gaming => slider::Appearance {
                rail: slider::Rail {
                    colors: (Color::from_rgb(0.0, 0.4, 0.6), Color::from_rgb(0.0, 0.8, 1.0)),
                    width: 4.0,
                    border_radius: 2.0.into(),
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 8.0 },
                    color: Color::from_rgb(0.0, 0.8, 1.0),
                    border_width: 1.0,
                    border_color: Color::from_rgb(0.0, 0.6, 0.8),
                },
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.hovered(&iced::theme::Slider::default()),
            AppTheme::Dark => iced::Theme::Dark.hovered(&iced::theme::Slider::default()),
            AppTheme::Gaming => slider::Appearance {
                rail: slider::Rail {
                    colors: (Color::from_rgb(0.0, 0.5, 0.7), Color::from_rgb(0.0, 0.9, 1.0)),
                    width: 4.0,
                    border_radius: 2.0.into(),
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 9.0 },
                    color: Color::from_rgb(0.0, 0.9, 1.0),
                    border_width: 1.0,
                    border_color: Color::from_rgb(0.0, 0.7, 0.9),
                },
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        match self {
            AppTheme::Light => iced::Theme::Light.dragging(&iced::theme::Slider::default()),
            AppTheme::Dark => iced::Theme::Dark.dragging(&iced::theme::Slider::default()),
            AppTheme::Gaming => slider::Appearance {
                rail: slider::Rail {
                    colors: (Color::from_rgb(0.0, 0.6, 0.8), Color::from_rgb(0.0, 1.0, 1.0)),
                    width: 4.0,
                    border_radius: 2.0.into(),
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 10.0 },
                    color: Color::from_rgb(0.0, 1.0, 1.0),
                    border_width: 1.0,
                    border_color: Color::from_rgb(0.0, 0.8, 1.0),
                },
            },
        }
    }
}
