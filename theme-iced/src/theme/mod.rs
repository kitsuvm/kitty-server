//! Kitty Theme for Iced.

use std::time::Duration;

use iced_core::{
    Border, Color, Length, Padding, Settings, Shadow,
    border::Radius,
    color,
    theme::{Base, Mode, Palette, Style, palette::Extended},
};
use iced_widget::{container, scrollable, text, text_input};

use crate::{
    BaseExtended,
    font::{LATO_BOLD_FONT, fonts},
    theme::{
        dark::{DARK_EXTERNAL_PALETTE, DARK_PALETTE},
        light::{LIGHT_EXTENDED_PALETTE, LIGHT_PALETTE},
    },
    widget::{
        application, button, content, icon_button, sidebar, window, window_background, window_bar,
        window_button,
    },
};

mod dark;
mod light;

/// The default settings for the application.
pub fn default_settings() -> Settings {
    Settings {
        id: None,
        fonts: fonts(),
        default_font: LATO_BOLD_FONT,
        default_text_size: 14.into(),
        antialiasing: true,
        vsync: true,
    }
}

/// The default window settings for the application.
pub fn default_window_settings() -> iced_core::window::Settings {
    iced_core::window::Settings {
        min_size: Some((400, 300).into()),
        decorations: false,
        transparent: true,
        ..Default::default()
    }
}

/// The name of the theme.
pub const THEME_NAME: &str = "kitty";

/// The theme for the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
            Self::Light => LIGHT_PALETTE,
            Self::Dark => DARK_PALETTE,
        }
    }

    /// Returns the extended palette for the theme.
    pub fn extended(&self) -> Extended {
        match self {
            Self::Light => LIGHT_EXTENDED_PALETTE,
            Self::Dark => DARK_EXTERNAL_PALETTE,
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

    /// Returns the animation mode for the theme.
    pub fn animation() -> iced_anim::animated::Mode {
        iced_anim::transition::Easing::default()
            .with_duration(Duration::from_millis(150))
            .into()
    }
}

impl From<Mode> for Theme {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Light | Mode::None => Theme::Light,
            Mode::Dark => Theme::Dark,
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

impl BaseExtended for Theme {
    fn palette_extended(&self) -> Extended {
        self.extended()
    }
}

impl iced_widget::text::Catalog for Theme {
    type Class<'a> = iced_widget::text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| text::Style {
            color: Some(theme.palette().text),
        })
    }

    fn style(&self, item: &Self::Class<'_>) -> text::Style {
        item(self)
    }
}

impl iced_widget::container::Catalog for Theme {
    type Class<'a> = iced_widget::container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme| {
            let palette = theme.palette();
            iced_widget::container::Style {
                background: Some(palette.background.into()),
                ..Default::default()
            }
        })
    }

    fn style(&self, item: &Self::Class<'_>) -> iced_widget::container::Style {
        item(self)
    }
}

impl iced_widget::button::Catalog for Theme {
    type Class<'a> = iced_widget::button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, status: iced_widget::button::Status| {
            let palette = theme.extended();

            iced_widget::button::Style {
                background: match status {
                    iced_widget::button::Status::Pressed | iced_widget::button::Status::Hovered => {
                        Some(palette.background.weakest.color.into())
                    }
                    _ => Some(Color::TRANSPARENT.into()),
                },
                text_color: match status {
                    iced_widget::button::Status::Disabled => palette.background.weakest.text,
                    _ => palette.background.base.text,
                },
                border: Border {
                    radius: Radius::from(5),
                    color: Color::TRANSPARENT,
                    width: 0.0,
                },
                ..Default::default()
            }
        })
    }

    fn style(
        &self,
        item: &Self::Class<'_>,
        status: iced_widget::button::Status,
    ) -> iced_widget::button::Style {
        item(self, status)
    }
}

impl window_background::Catalog for Theme {
    type SuperClass<'a> = window_background::StyleFn<'a, Self>;

    fn default_parameters() -> window_background::Parameters {
        window_background::Parameters {
            width: Some(Length::Fill),
            height: Some(Length::Fill),
            max_width: None,
            max_height: None,
            padding: Some(1.into()),
        }
    }

    fn into_class<'a>(
        class: Self::SuperClass<'a>,
        status: window_background::Status,
    ) -> Self::Class<'a> {
        Box::new(move |theme: &Theme| Self::style(theme, &class, status))
    }

    fn default<'a>() -> Self::SuperClass<'a> {
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
        class: &Self::SuperClass<'_>,
        status: window_background::Status,
    ) -> container::Style {
        class(self, status)
    }
}

impl application::Catalog for Theme {
    fn style(&self) -> iced_core::theme::Style {
        let palette = self.palette();

        Style {
            background_color: Color::TRANSPARENT,
            text_color: palette.text,
        }
    }
}

impl window_button::Catalog for Theme {
    type SuperClass<'a> = window_button::StyleFn<'a, Self>;

    fn default_parameters() -> window_button::Parameters {
        window_button::Parameters {
            position: window_button::Position::Center,
            left_buttons: false,
            no_rounded_corner: false,
            size: 34.into(),
            animated: true,
            animation: Some(Theme::animation()),
        }
    }

    fn default<'a>() -> Self::SuperClass<'a> {
        Box::new(
            |theme: &Self, status: window_button::Status, button_status: button::Status| {
                let window_corner = match status.no_rounded_corner {
                    true => 0.0,
                    false => theme.window_radius(),
                };

                let palette = theme.extended();

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
                    background: match button_status {
                        button::Status::Pressed | button::Status::Hovered => {
                            Some(palette.primary.base.color.into())
                        }
                        _ => Some(Color::TRANSPARENT.into()),
                    },
                    text_color: match button_status {
                        button::Status::Disabled => palette.background.weakest.text,
                        button::Status::Pressed | button::Status::Hovered => {
                            palette.primary.base.text
                        }
                        _ => palette.background.base.text,
                    },
                    ..Default::default()
                }
            },
        )
    }

    fn style(
        &self,
        class: &Self::SuperClass<'_>,
        status: window_button::Status,
        button_status: button::Status,
    ) -> button::Style {
        class(self, status, button_status)
    }

    fn into_class<'a>(
        class: Self::SuperClass<'a>,
        status: window_button::Status,
    ) -> Self::Class<'a> {
        Box::new(move |theme: &Self, button_status: button::Status| {
            Self::style(theme, &class, status, button_status)
        }) as Self::Class<'a>
    }
}

impl window_bar::Catalog for Theme {
    fn into_class<'a>(style: impl Fn(&Self) -> container::Style + 'a) -> Self::Class<'a> {
        Box::new(style) as Self::Class<'a>
    }
}

impl sidebar::Catalog for Theme {
    fn padding() -> Padding {
        15.into()
    }

    fn spacing() -> iced_core::Pixels {
        25.into()
    }

    fn into_class<'a>(style: impl Fn(&Self) -> container::Style + 'a) -> Self::Class<'a> {
        (Box::new(style) as container::StyleFn<'a, Self>).into()
    }
}

impl text_input::Catalog for Theme {
    type Class<'a> = text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme: &Theme, _: text_input::Status| {
            let extended = theme.extended();

            text_input::Style {
                background: extended.background.weakest.color.into(),
                border: Border {
                    radius: Radius::from(6),
                    ..Default::default()
                },
                icon: extended.background.base.text.into(),
                placeholder: extended.background.weakest.text.into(),
                selection: extended.primary.base.color.into(),
                value: extended.background.base.text.into(),
            }
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        class(self, status)
    }
}

impl scrollable::Catalog for Theme {
    type Class<'a> = scrollable::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_: &Self, _: scrollable::Status| scrollable::Style {
            auto_scroll: scrollable::AutoScroll {
                background: Color::BLACK.into(),
                border: Border::default(),
                shadow: Shadow::default(),
                icon: Color::WHITE.into(),
            },
            container: container::Style::default(),
            gap: None,
            horizontal_rail: scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    background: Color::BLACK.into(),
                    border: Border::default(),
                },
            },
            vertical_rail: scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    background: Color::BLACK.into(),
                    border: Border::default(),
                },
            },
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        class(self, status)
    }
}

impl window::Catalog for Theme {
    fn default_parameters() -> window::Parameters {
        window::Parameters {
            animated: true,
            animation: Some(Theme::animation()),
            window_bar_centered: true,
            icon_size: Some(16.into()),
            ..Default::default()
        }
    }

    fn into_button_class<'a>(
        style: impl Fn(&Self, window_button::Status, button::Status) -> button::Style + 'a,
    ) -> <Self as window_button::Catalog>::SuperClass<'a> {
        Box::new(style) as <Self as window_button::Catalog>::SuperClass<'a>
    }

    fn into_container_class<'a>(
        style: impl Fn(&Self) -> container::Style + 'a,
    ) -> <Self as container::Catalog>::Class<'a> {
        Box::new(style) as <Self as container::Catalog>::Class<'a>
    }

    fn into_text_class<'a>(
        style: impl Fn(&Self) -> iced_widget::text::Style + 'a,
    ) -> <Self as iced_widget::text::Catalog>::Class<'a> {
        Box::new(style) as <Self as iced_widget::text::Catalog>::Class<'a>
    }
}

impl content::Catalog for Theme {
    fn into_class<'a>(style: impl Fn(&Self) -> container::Style + 'a) -> Self::Class<'a> {
        Box::new(style) as Self::Class<'a>
    }
}

impl icon_button::Catalog for Theme {
    fn default_parameters() -> icon_button::Parameters {
        icon_button::Parameters {
            size: 26.into(),
            icon_size: Some(16.into()),
            animated: true,
            animation: Some(Theme::animation()),
        }
    }

    fn default<'a>() -> <Self as iced_widget::button::Catalog>::Class<'a> {
        Box::new(|theme: &Theme, status: button::Status| button::Style {
            background: match status {
                button::Status::Pressed | button::Status::Hovered => {
                    Some(theme.extended().primary.base.color.into())
                }
                _ => Some(Color::TRANSPARENT.into()),
            },
            border: Border {
                radius: Radius::from(5),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    fn style<'a>(
        &self,
        class: <Self as iced_widget::button::Catalog>::Class<'a>,
        status: button::Status,
    ) -> iced_anim::widget::button::Style {
        class(self, status)
    }
}

impl button::Catalog for Theme {
    fn default_parameters() -> button::Parameters {
        button::Parameters {
            animated: true,
            animation: Some(Theme::animation()),
            ..Default::default()
        }
    }
}
