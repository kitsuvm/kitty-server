//! Kitty Theme for Iced.

use iced::{
    Color, application,
    theme::{Base, Mode, Palette, Style},
    widget::{button, container, text},
};

/// The name of the theme.
pub const THEME_NAME: &str = "kitty";

pub mod dark;

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
            Theme::Dark => dark::PALETTE,
        }
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

impl<State> application::ThemeFn<State, Theme> for Theme {
    fn theme(&self, _: &State) -> Option<Theme> {
        Some(*self)
    }
}

impl text::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> text::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| {
            let palette = theme.palette();
            text::Style {
                color: Some(palette.text),
            }
        })
    }

    fn style(&self, item: &Self::Class<'_>) -> text::Style {
        item(self)
    }
}

impl container::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> container::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| container::Style {
            background: Some(
                match theme {
                    Theme::Light => Color::WHITE,
                    Theme::Dark => Color::BLACK,
                }
                .into(),
            ),
            ..Default::default()
        })
    }

    fn style(&self, item: &Self::Class<'_>) -> container::Style {
        item(self)
    }
}

impl button::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, button::Status) -> button::Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, _: button::Status| {
            let palette = theme.palette();
            button::Style {
                background: Some(palette.background.into()),
                text_color: palette.text,
                ..Default::default()
            }
        })
    }

    fn style(&self, item: &Self::Class<'_>, status: button::Status) -> button::Style {
        item(self, status)
    }
}
