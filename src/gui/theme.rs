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
impl widget::text::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::text::Appearance {
        match self {
            AppTheme::Light => style.text(),
            AppTheme::Dark => style.text(),
            AppTheme::Gaming => widget::text::Appearance { color: Color::WHITE },
        }
    }
}

impl widget::button::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().active,
            AppTheme::Dark => style.button().active,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.8))), // Bluish
                text_color: Color::WHITE,
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 1.0),
                },
                shadow_offset: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().hovered,
            AppTheme::Dark => style.button().hovered,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.9))),
                text_color: Color::WHITE,
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.5, 0.5, 1.0),
                },
                shadow_offset: Default::default(),
            },
        }
    }

    fn pressed(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().pressed,
            AppTheme::Dark => style.button().pressed,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.7))),
                text_color: Color::WHITE,
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.9),
                },
                shadow_offset: Default::default(),
            },
        }
    }

    fn disabled(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().disabled,
            AppTheme::Dark => style.button().disabled,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                text_color: Color::from_rgb(0.5, 0.5, 0.5),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                },
                shadow_offset: Default::default(),
            },
        }
    }

}

impl widget::container::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::container::Appearance {
        match self {
            AppTheme::Light => style.container(),
            AppTheme::Dark => style.container(),
            AppTheme::Gaming => widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                border: Border {
                    radius: 10.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                text_color: None,
                shadow_offset: Default::default(),
            },
        }
    }
}

impl widget::progress_bar::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::progress_bar::Appearance {
        match self {
            AppTheme::Light => style.progress_bar(),
            AppTheme::Dark => style.progress_bar(),
            AppTheme::Gaming => widget::progress_bar::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
                bar: Background::Color(Color::from_rgb(0.0, 0.8, 0.0)),
                border_radius: 5.0.into(),
            },
        }
    }
}

impl widget::pick_list::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::pick_list::Appearance {
        match self {
            AppTheme::Light => style.pick_list().active,
            AppTheme::Dark => style.pick_list().active,
            AppTheme::Gaming => widget::pick_list::Appearance {
                text_color: Color::WHITE,
                placeholder_color: Color::from_rgb(0.6, 0.6, 0.6),
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                handle_color: Color::WHITE,
            },
        }
    }

    fn hovered(&self, style: &Theme) -> widget::pick_list::Appearance {
        match self {
            AppTheme::Light => style.pick_list().hovered,
            AppTheme::Dark => style.pick_list().hovered,
            AppTheme::Gaming => widget::pick_list::Appearance {
                text_color: Color::WHITE,
                placeholder_color: Color::from_rgb(0.7, 0.7, 0.7),
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 0.5),
                },
                handle_color: Color::WHITE,
            },
        }
    }

}

impl widget::text_input::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::text_input::Appearance {
        match self {
            AppTheme::Light => style.text_input().active,
            AppTheme::Dark => style.text_input().active,
            AppTheme::Gaming => widget::text_input::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.2))),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                icon_color: Color::WHITE,
            },
        }
    }

    fn focused(&self, style: &Theme) -> widget::text_input::Appearance {
        match self {
            AppTheme::Light => style.text_input().focused,
            AppTheme::Dark => style.text_input().focused,
            AppTheme::Gaming => widget::text_input::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.2))),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.8),
                },
                icon_color: Color::WHITE,
            },
        }
    }

    fn placeholder_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().placeholder_color,
            AppTheme::Dark => style.text_input().placeholder_color,
            AppTheme::Gaming => Color::from_rgb(0.6, 0.6, 0.6),
        }
    }

    fn value_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().value_color,
            AppTheme::Dark => style.text_input().value_color,
            AppTheme::Gaming => Color::WHITE,
        }
    }

    fn selection_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().selection_color,
            AppTheme::Dark => style.text_input().selection_color,
            AppTheme::Gaming => Color::from_rgb(0.2, 0.2, 0.8),
        }
    }

    fn disabled_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().disabled_color,
            AppTheme::Dark => style.text_input().disabled_color,
            AppTheme::Gaming => Color::from_rgb(0.5, 0.5, 0.5),
        }
    }

    fn disabled(&self, style: &Theme) -> widget::text_input::Appearance {
        match self {
            AppTheme::Light => style.text_input().disabled,
            AppTheme::Dark => style.text_input().disabled,
            AppTheme::Gaming => widget::text_input::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                },
                icon_color: Color::from_rgb(0.5, 0.5, 0.5),
            },
        }
    }
}

impl widget::checkbox::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => style.checkbox(is_checked).active,
            AppTheme::Dark => style.checkbox(is_checked).active,
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                icon_color: Color::WHITE,
                text_color: Some(Color::WHITE), // Aggiunto il campo mancante
            },
        }
    }

    fn hovered(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => style.checkbox(is_checked).hovered,
            AppTheme::Dark => style.checkbox(is_checked).hovered,
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 0.5),
                },
                icon_color: Color::WHITE,
                text_color: Some(Color::WHITE), // Aggiunto il campo mancante
            },
        }
    }

    fn disabled(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => style.checkbox(is_checked).disabled,
            AppTheme::Dark => style.checkbox(is_checked).disabled,
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                },
                icon_color: Color::from_rgb(0.5, 0.5, 0.5),
                text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            },
        }
    }
}

impl widget::radio::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme, is_selected: bool) -> widget::radio::Appearance {
        match self {
            AppTheme::Light => style.radio(is_selected).active,
            AppTheme::Dark => style.radio(is_selected).active,
            AppTheme::Gaming => widget::radio::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.3, 0.3, 0.4),
                dot_color: Color::from_rgb(0.0, 0.8, 0.0),
                text_color: Some(Color::WHITE),
            },
        }
    }

    fn hovered(&self, style: &Theme, is_selected: bool) -> widget::radio::Appearance {
        match self {
            AppTheme::Light => style.radio(is_selected).hovered,
            AppTheme::Dark => style.radio(is_selected).hovered,
            AppTheme::Gaming => widget::radio::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.4, 0.4, 0.5),
                dot_color: Color::from_rgb(0.0, 0.9, 0.0),
                text_color: Some(Color::WHITE),
            },
        }
    }

    fn disabled(&self, style: &Theme, is_selected: bool) -> widget::radio::Appearance {
        match self {
            AppTheme::Light => style.radio(is_selected).disabled,
            AppTheme::Dark => style.radio(is_selected).disabled,
            AppTheme::Gaming => widget::radio::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.2, 0.2, 0.2),
                dot_color: Color::from_rgb(0.5, 0.5, 0.5),
                text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            },
        }
    }
}

impl widget::slider::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::slider::Appearance {
        match self {
            AppTheme::Light => style.slider().active,
            AppTheme::Dark => style.slider().active,
            AppTheme::Gaming => widget::slider::Appearance {
                rail: widget::slider::Rail {
                    colors: (Color::from_rgb(0.3, 0.3, 0.4), Color::from_rgb(0.15, 0.15, 0.2)),
                    width: 2.0,
                    border_radius: 2.0.into(),
                },
                handle: widget::slider::Handle {
                    shape: widget::slider::HandleShape::Circle { radius: 6.0 },
                    color: Color::from_rgb(0.0, 0.8, 0.0),
                    border_color: Color::from_rgb(0.0, 0.8, 0.0),
                    border_width: 1.0,
                },
            },
        }
    }

    fn hovered(&self, style: &Theme) -> widget::slider::Appearance {
        match self {
            AppTheme::Light => style.slider().hovered,
            AppTheme::Dark => style.slider().hovered,
            AppTheme::Gaming => widget::slider::Appearance {
                rail: widget::slider::Rail {
                    colors: (Color::from_rgb(0.4, 0.4, 0.5), Color::from_rgb(0.2, 0.2, 0.25)),
                    width: 2.0,
                    border_radius: 2.0.into(),
                },
                handle: widget::slider::Handle {
                    shape: widget::slider::HandleShape::Circle { radius: 6.0 },
                    color: Color::from_rgb(0.0, 0.9, 0.0),
                    border_color: Color::from_rgb(0.0, 0.9, 0.0),
                    border_width: 1.0,
                },
            },
        }
    }

    fn dragging(&self, style: &Theme) -> widget::slider::Appearance {
        match self {
            AppTheme::Light => style.slider().dragging,
            AppTheme::Dark => style.slider().dragging,
            AppTheme::Gaming => widget::slider::Appearance {
                rail: widget::slider::Rail {
                    colors: (Color::from_rgb(0.4, 0.4, 0.5), Color::from_rgb(0.2, 0.2, 0.25)),
                    width: 2.0,
                    border_radius: 2.0.into(),
                },
                handle: widget::slider::Handle {
                    shape: widget::slider::HandleShape::Circle { radius: 6.0 },
                    color: Color::from_rgb(0.0, 1.0, 0.0),
                    border_color: Color::from_rgb(0.0, 1.0, 0.0),
                    border_width: 1.0,
                },
            },
        }
    }

}

impl widget::scrollable::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::scrollable::Appearance {
        match self {
            AppTheme::Light => style.scrollable().active,
            AppTheme::Dark => style.scrollable().active,
            AppTheme::Gaming => widget::scrollable::Appearance {
                container: widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: None,
                    shadow_offset: Default::default(),
                },
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgb(0.3, 0.3, 0.4),
                        border: Border {
                            radius: 5.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                },
                gap: 0.0,
            },
        }
    }

    fn hovered(&self, style: &Theme, is_mouse_over_scrollbar: bool) -> widget::scrollable::Appearance {
        match self {
            AppTheme::Light => style.scrollable().hovered(is_mouse_over_scrollbar),
            AppTheme::Dark => style.scrollable().hovered(is_mouse_over_scrollbar),
            AppTheme::Gaming => widget::scrollable::Appearance {
                container: widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: None,
                    shadow_offset: Default::default(),
                },
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgb(0.4, 0.4, 0.5),
                        border: Border {
                            radius: 5.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                },
                gap: 0.0,
            },
        }
    }

    fn dragging(&self, style: &Theme) -> widget::scrollable::Appearance {
        match self {
            AppTheme::Light => style.scrollable().dragging,
            AppTheme::Dark => style.scrollable().dragging,
            AppTheme::Gaming => widget::scrollable::Appearance {
                container: widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: None,
                    shadow_offset: Default::default(),
                },
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgb(0.5, 0.5, 0.6),
                        border: Border {
                            radius: 5.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                },
                gap: 0.0,
            },
        }
    }
}

impl widget::rule::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::rule::Appearance {
        match self {
            AppTheme::Light => style.rule(),
            AppTheme::Dark => style.rule(),
            AppTheme::Gaming => widget::rule::Appearance {
                color: Color::from_rgb(0.3, 0.3, 0.4),
                width: 1,
                radius: 0.0.into(),
                fill_mode: widget::rule::FillMode::Full,
            },
        }
    }
}
impl widget::text::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::text::Appearance {
        match self {
            AppTheme::Light => style.text(),
            AppTheme::Dark => style.text(),
            AppTheme::Gaming => widget::text::Appearance { color: Some(Color::WHITE) },
        }
    }
}

impl widget::button::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().active,
            AppTheme::Dark => style.button().active,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.8))), // Bluish
                text_color: Color::WHITE,
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 1.0),
                },
                shadow_offset: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().hovered,
            AppTheme::Dark => style.button().hovered,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.9))),
                text_color: Color::WHITE,
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.5, 0.5, 1.0),
                },
                shadow_offset: Default::default(),
            },
        }
    }

    fn pressed(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().pressed,
            AppTheme::Dark => style.button().pressed,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.7))),
                text_color: Color::WHITE,
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.9),
                },
                shadow_offset: Default::default(),
            },
        }
    }

    fn disabled(&self, style: &Theme) -> widget::button::Appearance {
        match self {
            AppTheme::Light => style.button().disabled,
            AppTheme::Dark => style.button().disabled,
            AppTheme::Gaming => widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                text_color: Color::from_rgb(0.5, 0.5, 0.5),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                },
                shadow_offset: Default::default(),
            },
        }
    }

}

impl widget::container::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::container::Appearance {
        match self {
            AppTheme::Light => style.container(),
            AppTheme::Dark => style.container(),
            AppTheme::Gaming => widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                border: Border {
                    radius: 10.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                text_color: None,
                shadow_offset: Default::default(),
            },
        }
    }
}

impl widget::progress_bar::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::progress_bar::Appearance {
        match self {
            AppTheme::Light => style.progress_bar(),
            AppTheme::Dark => style.progress_bar(),
            AppTheme::Gaming => widget::progress_bar::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
                bar: Background::Color(Color::from_rgb(0.0, 0.8, 0.0)),
                border_radius: 5.0.into(),
            },
        }
    }
}

impl widget::pick_list::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::pick_list::Appearance {
        match self {
            AppTheme::Light => style.pick_list().active,
            AppTheme::Dark => style.pick_list().active,
            AppTheme::Gaming => widget::pick_list::Appearance {
                text_color: Color::WHITE,
                placeholder_color: Color::from_rgb(0.6, 0.6, 0.6),
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                handle_color: Color::WHITE,
            },
        }
    }

    fn hovered(&self, style: &Theme) -> widget::pick_list::Appearance {
        match self {
            AppTheme::Light => style.pick_list().hovered,
            AppTheme::Dark => style.pick_list().hovered,
            AppTheme::Gaming => widget::pick_list::Appearance {
                text_color: Color::WHITE,
                placeholder_color: Color::from_rgb(0.7, 0.7, 0.7),
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 0.5),
                },
                handle_color: Color::WHITE,
            },
        }
    }

}

impl widget::text_input::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::text_input::Appearance {
        match self {
            AppTheme::Light => style.text_input().active,
            AppTheme::Dark => style.text_input().active,
            AppTheme::Gaming => widget::text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                icon_color: Color::WHITE,
            },
        }
    }

    fn focused(&self, style: &Theme) -> widget::text_input::Appearance {
        match self {
            AppTheme::Light => style.text_input().focused,
            AppTheme::Dark => style.text_input().focused,
            AppTheme::Gaming => widget::text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.8),
                },
                icon_color: Color::WHITE,
            },
        }
    }

    fn placeholder_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().placeholder_color,
            AppTheme::Dark => style.text_input().placeholder_color,
            AppTheme::Gaming => Color::from_rgb(0.6, 0.6, 0.6),
        }
    }

    fn value_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().value_color,
            AppTheme::Dark => style.text_input().value_color,
            AppTheme::Gaming => Color::WHITE,
        }
    }

    fn selection_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().selection_color,
            AppTheme::Dark => style.text_input().selection_color,
            AppTheme::Gaming => Color::from_rgb(0.2, 0.2, 0.8),
        }
    }

    fn disabled_color(&self, style: &Theme) -> Color {
        match self {
            AppTheme::Light => style.text_input().disabled_color,
            AppTheme::Dark => style.text_input().disabled_color,
            AppTheme::Gaming => Color::from_rgb(0.5, 0.5, 0.5),
        }
    }

    fn disabled(&self, style: &Theme) -> widget::text_input::Appearance {
        match self {
            AppTheme::Light => style.text_input().disabled,
            AppTheme::Dark => style.text_input().disabled,
            AppTheme::Gaming => widget::text_input::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                },
                icon_color: Color::from_rgb(0.5, 0.5, 0.5),
            },
        }
    }
}

impl widget::checkbox::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => style.checkbox(is_checked).active,
            AppTheme::Dark => style.checkbox(is_checked).active,
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.3, 0.3, 0.4),
                },
                icon_color: Color::WHITE,
                text_color: Some(Color::WHITE), // Aggiunto il campo mancante
            },
        }
    }

    fn hovered(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => style.checkbox(is_checked).hovered,
            AppTheme::Dark => style.checkbox(is_checked).hovered,
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.4, 0.4, 0.5),
                },
                icon_color: Color::WHITE,
                text_color: Some(Color::WHITE), // Aggiunto il campo mancante
            },
        }
    }

    fn disabled(&self, style: &Theme, is_checked: bool) -> widget::checkbox::Appearance {
        match self {
            AppTheme::Light => style.checkbox(is_checked).disabled,
            AppTheme::Dark => style.checkbox(is_checked).disabled,
            AppTheme::Gaming => widget::checkbox::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                },
                icon_color: Color::from_rgb(0.5, 0.5, 0.5),
                text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            },
        }
    }
}

impl widget::radio::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme, is_selected: bool) -> widget::radio::Appearance {
        match self {
            AppTheme::Light => style.radio(is_selected).active,
            AppTheme::Dark => style.radio(is_selected).active,
            AppTheme::Gaming => widget::radio::Appearance {
                background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.3, 0.3, 0.4),
                dot_color: Color::from_rgb(0.0, 0.8, 0.0),
                text_color: Some(Color::WHITE),
            },
        }
    }

    fn hovered(&self, style: &Theme, is_selected: bool) -> widget::radio::Appearance {
        match self {
            AppTheme::Light => style.radio(is_selected).hovered,
            AppTheme::Dark => style.radio(is_selected).hovered,
            AppTheme::Gaming => widget::radio::Appearance {
                background: Background::Color(Color::from_rgb(0.2, 0.2, 0.25)),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.4, 0.4, 0.5),
                dot_color: Color::from_rgb(0.0, 0.9, 0.0),
                text_color: Some(Color::WHITE),
            },
        }
    }

    fn disabled(&self, style: &Theme, is_selected: bool) -> widget::radio::Appearance {
        match self {
            AppTheme::Light => style.radio(is_selected).disabled,
            AppTheme::Dark => style.radio(is_selected).disabled,
            AppTheme::Gaming => widget::radio::Appearance {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                border_radius: 10.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.2, 0.2, 0.2),
                dot_color: Color::from_rgb(0.5, 0.5, 0.5),
                text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            },
        }
    }
}

impl widget::slider::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::slider::Appearance {
        match self {
            AppTheme::Light => style.slider().active,
            AppTheme::Dark => style.slider().active,
            AppTheme::Gaming => widget::slider::Appearance {
                rail: widget::slider::Rail {
                    colors: (Color::from_rgb(0.3, 0.3, 0.4), Color::from_rgb(0.15, 0.15, 0.2)),
                    width: 2.0,
                    border_radius: 2.0.into(),
                },
                handle: widget::slider::Handle {
                    shape: widget::slider::HandleShape::Circle { radius: 6.0 },
                    color: Color::from_rgb(0.0, 0.8, 0.0),
                    border_color: Color::from_rgb(0.0, 0.8, 0.0),
                    border_width: 1.0,
                },
            },
        }
    }

    fn hovered(&self, style: &Theme) -> widget::slider::Appearance {
        match self {
            AppTheme::Light => style.slider().hovered,
            AppTheme::Dark => style.slider().hovered,
            AppTheme::Gaming => widget::slider::Appearance {
                rail: widget::slider::Rail {
                    colors: (Color::from_rgb(0.4, 0.4, 0.5), Color::from_rgb(0.2, 0.2, 0.25)),
                    width: 2.0,
                    border_radius: 2.0.into(),
                },
                handle: widget::slider::Handle {
                    shape: widget::slider::HandleShape::Circle { radius: 6.0 },
                    color: Color::from_rgb(0.0, 0.9, 0.0),
                    border_color: Color::from_rgb(0.0, 0.9, 0.0),
                    border_width: 1.0,
                },
            },
        }
    }

    fn dragging(&self, style: &Theme) -> widget::slider::Appearance {
        match self {
            AppTheme::Light => style.slider().dragging,
            AppTheme::Dark => style.slider().dragging,
            AppTheme::Gaming => widget::slider::Appearance {
                rail: widget::slider::Rail {
                    colors: (Color::from_rgb(0.4, 0.4, 0.5), Color::from_rgb(0.2, 0.2, 0.25)),
                    width: 2.0,
                    border_radius: 2.0.into(),
                },
                handle: widget::slider::Handle {
                    shape: widget::slider::HandleShape::Circle { radius: 6.0 },
                    color: Color::from_rgb(0.0, 1.0, 0.0),
                    border_color: Color::from_rgb(0.0, 1.0, 0.0),
                    border_width: 1.0,
                },
            },
        }
    }

}

impl widget::scrollable::StyleSheet for AppTheme {
    type Style = Theme;

    fn active(&self, style: &Theme) -> widget::scrollable::Appearance {
        match self {
            AppTheme::Light => style.scrollable().active,
            AppTheme::Dark => style.scrollable().active,
            AppTheme::Gaming => widget::scrollable::Appearance {
                container: widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: None,
                    shadow_offset: Default::default(),
                },
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgb(0.3, 0.3, 0.4),
                        border: Border {
                            radius: 5.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                },
                gap: 0.0,
            },
        }
    }

    fn hovered(&self, style: &Theme, is_mouse_over_scrollbar: bool) -> widget::scrollable::Appearance {
        match self {
            AppTheme::Light => style.scrollable().hovered(is_mouse_over_scrollbar),
            AppTheme::Dark => style.scrollable().hovered(is_mouse_over_scrollbar),
            AppTheme::Gaming => widget::scrollable::Appearance {
                container: widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: None,
                    shadow_offset: Default::default(),
                },
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgb(0.4, 0.4, 0.5),
                        border: Border {
                            radius: 5.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                },
                gap: 0.0,
            },
        }
    }

    fn dragging(&self, style: &Theme) -> widget::scrollable::Appearance {
        match self {
            AppTheme::Light => style.scrollable().dragging,
            AppTheme::Dark => style.scrollable().dragging,
            AppTheme::Gaming => widget::scrollable::Appearance {
                container: widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: None,
                    shadow_offset: Default::default(),
                },
                scrollbar: widget::scrollable::Scrollbar {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    scroller: widget::scrollable::Scroller {
                        color: Color::from_rgb(0.5, 0.5, 0.6),
                        border: Border {
                            radius: 5.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                },
                gap: 0.0,
            },
        }
    }
}

impl widget::rule::StyleSheet for AppTheme {
    type Style = Theme;

    fn appearance(&self, style: &Theme) -> widget::rule::Appearance {
        match self {
            AppTheme::Light => style.rule(),
            AppTheme::Dark => style.rule(),
            AppTheme::Gaming => widget::rule::Appearance {
                color: Color::from_rgb(0.3, 0.3, 0.4),
                width: 1,
                radius: 0.0.into(),
                fill_mode: widget::rule::FillMode::Full,
            },
        }
    }
}

/// Conversione da `AppTheme` a `iced::Theme`.
/// I temi `Light` e `Dark` usano i temi predefiniti di Iced.
/// Il tema `Gaming` usa un tema `Custom` con l'implementazione di `AppTheme` stessa.
impl From<AppTheme> for Theme {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
            AppTheme::Gaming => Theme::Custom(Arc::new(app_theme)),
        }
    }
}
