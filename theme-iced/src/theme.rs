//! Kitty Theme for Iced.

use iced_core::{
    Border, Color, Padding, Shadow,
    border::Radius,
    color,
    theme::{
        Base, Mode, Palette, Style,
        palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success, Warning},
    },
};
use iced_widget::{button, container, scrollable, text, text_input};

use crate::widget::{
    application, content, sidebar, window, window_background, window_bar, window_button,
};

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

pub const DARK_EXTERNAL_PALETTE: Extended = Extended {
    is_dark: true,
    background: Background {
        base: Pair {
            color: color!(0x000000),
            text: color!(0xffffff),
        },
        weakest: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        weaker: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        weak: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        neutral: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        strong: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        stronger: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        strongest: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
    },
    danger: Danger {
        base: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
    },
    primary: Primary {
        base: Pair {
            color: color!(0x8500ff),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x8500ff),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x8500ff),
            text: color!(0xffffff),
        },
    },
    secondary: Secondary {
        base: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
    },
    success: Success {
        base: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
    },
    warning: Warning {
        base: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
    },
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
            Self::Light => Palette::LIGHT,
            Self::Dark => DARK_PALETTE,
        }
    }

    /// Returns the extended palette for the theme.
    pub fn extended(&self) -> Extended {
        match self {
            Self::Light => Extended::generate(Self::Light.palette()),
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

impl button::Catalog for Theme {
    type Class<'a> = button::StyleFn<'a, Self>;

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
    type SuperClass<'a> = window_background::StyleFn<'a, Self>;

    fn padding() -> Option<Padding> {
        Some(Padding::from(1))
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

    fn default<'a>() -> Self::SuperClass<'a> {
        Box::new(
            |theme: &Self, status: window_button::Status, button_status: button::Status| {
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
                    background: match button_status {
                        button::Status::Pressed => Some(theme.palette().primary.into()),
                        button::Status::Hovered => Some(theme.palette().primary.into()),
                        _ => Some(Color::TRANSPARENT.into()),
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
}

impl content::Catalog for Theme {
    fn into_class<'a>(style: impl Fn(&Self) -> container::Style + 'a) -> Self::Class<'a> {
        Box::new(style) as Self::Class<'a>
    }
}
