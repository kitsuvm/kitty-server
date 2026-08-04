//! Kitty Theme for Iced.

use iced::{
    Border, Color, Padding,
    border::Radius,
    color,
    theme::{Base, Mode, Palette, Style},
    widget::{button, container, text},
};

use crate::widget::{application, window_background, window_button};

/// The name of the theme.
pub const THEME_NAME: &str = "kitty";

/// The palette for the dark theme.
pub const DARK_PALETTE: Palette = Palette {
    background: color!(0x000000),
    text: color!(0xffffff),
    primary: color!(0x8500ff),
    success: color!(0x00ff00),
    warning: color!(0xffff00),
    danger: color!(0xff0000),
};

/// The theme for the application.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    /// The light theme.
    Light,
    /// The dark theme.
    Dark,
}

impl Theme {
    /// Returns the palette for the theme.
    pub fn palette(&self) -> Palette {
        match self {
            Theme::Light => Palette::LIGHT,
            Theme::Dark => DARK_PALETTE,
        }
    }

    /// Returns the radius for the window.
    pub fn window_radius(&self) -> f32 {
        10.0
    }

    /// Returns the border width for the window.
    pub fn window_border_width(&self) -> f32 {
        1.2
    }
}

impl Base for Theme {
    fn name(&self) -> &str {
        THEME_NAME
    }

    fn default(preference: Mode) -> Self {
        match preference {
            Mode::Light | Mode::None => Theme::Light,
            Mode::Dark => Theme::Dark,
        }
    }

    fn mode(&self) -> Mode {
        match self {
            Theme::Light => Mode::Light,
            Theme::Dark => Mode::Dark,
        }
    }

    fn palette(&self) -> Option<Palette> {
        Some(self.palette())
    }

    fn base(&self) -> Style {
        let palette = self.palette();
        Style {
            background_color: palette.background,
            text_color: palette.text,
        }
    }
}

impl<State> iced::application::ThemeFn<State, Theme> for Theme {
    fn theme(&self, _: &State) -> Option<Theme> {
        Some(*self)
    }
}

impl iced::widget::text::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> iced::widget::text::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| text::Style {
            color: Some(theme.palette().text),
        })
    }

    fn style(&self, item: &Self::Class<'_>) -> text::Style {
        item(self)
    }
}

impl iced::widget::container::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> iced::widget::container::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| {
            let palette = theme.palette();
            iced::widget::container::Style {
                background: Some(palette.background.into()),
                ..Default::default()
            }
        })
    }

    fn style(&self, item: &Self::Class<'_>) -> iced::widget::container::Style {
        item(self)
    }
}

impl button::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, button::Status) -> button::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_: &Theme, _: button::Status| button::Style {
            ..Default::default()
        })
    }

    fn style(&self, item: &Self::Class<'_>, status: button::Status) -> button::Style {
        item(self, status)
    }
}

impl window_background::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, window_background::Status) -> container::Style>;

    fn padding() -> Option<Padding> {
        Some(Padding::from(1))
    }

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, status: window_background::Status| {
            let palette = theme.palette();

            container::Style {
                border: match status {
                    window_background::Status::Normal => Border {
                        radius: Radius::from(theme.window_radius()),
                        color: match *theme {
                            Theme::Light => color!(0xd1d1d1),
                            Theme::Dark => color!(0x2d2d2d),
                        },
                        width: theme.window_border_width(),
                    },
                    window_background::Status::Maximized => Border::default(),
                },
                background: Some(palette.background.into()),
                ..Default::default()
            }
        })
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: window_background::Status,
    ) -> container::Style {
        class(self, status)
    }
}

impl application::Catalog for Theme {
    fn style(&self) -> iced::theme::Style {
        let palette = self.palette();

        Style {
            background_color: Color::TRANSPARENT,
            text_color: palette.text,
        }
    }
}

impl window_button::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, window_button::Status) -> button::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, status: window_button::Status| {
            let window_corner = match status.no_rounded_corner {
                true => 0.0,
                false => theme.window_radius(),
            };

            button::Style {
                border: match (status.button_position, status.left_buttons) {
                    (window_button::Position::Left, true) => Border {
                        radius: Radius::from(0).top_left(window_corner),
                        color: Color::TRANSPARENT,
                        width: theme.window_border_width(),
                    },
                    (window_button::Position::Right, false) => Border {
                        radius: Radius::from(0).top_right(window_corner),
                        color: Color::TRANSPARENT,
                        width: theme.window_border_width(),
                    },
                    _ => Border::default(),
                },
                background: match status.button_status {
                    button::Status::Pressed => Some(theme.palette().primary.into()),
                    button::Status::Hovered => Some(theme.palette().primary.into()),
                    _ => None,
                },
                ..Default::default()
            }
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: window_button::Status) -> button::Style {
        class(self, status)
    }
}
