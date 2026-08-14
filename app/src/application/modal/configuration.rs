//! A modal for changing the application configuration, such as theme and language.

use iced::{
    Border, Element, Length, Padding, Renderer,
    border::Radius,
    widget::{column, row, space},
};
use kitty_theme_iced::{
    theme::Theme,
    widget::{button, icon, text},
};

use crate::{
    application::{message::Message, modal::Modal, state::GlobalState},
    resources::app_config::{AppLanguage, AppTheme},
    t,
};

/// A modal for changing the application configuration, such as theme and language.
#[derive(Debug, Clone, Default)]
pub struct State;

impl Modal for State {
    fn content<'a>(&'a self, global_state: &GlobalState) -> Element<'a, Message, Theme, Renderer> {
        let current_theme = global_state.app_config.theme;
        let current_language = global_state.app_config.language;

        column![
            row![
                button(icon(icon::COMPUTER_ICON).center())
                    .width(Length::Fill)
                    .on_press_maybe(match current_theme {
                        AppTheme::System => None,
                        _ => Some(Message::ChangeAppTheme(AppTheme::System)),
                    })
                    .style(move |theme: &Theme, status| {
                        let base = match current_theme {
                            AppTheme::System => button::primary(theme, button::Status::Active),
                            _ => button::alt(theme, status),
                        };

                        button::Style {
                            border: Border {
                                radius: Radius::from(0).left(6),
                                width: 0.0,
                                ..Default::default()
                            },
                            ..base
                        }
                    }),
                button(icon(icon::LIGHT_MODE_ICON).center())
                    .width(Length::Fill)
                    .on_press_maybe(match current_theme {
                        AppTheme::Light => None,
                        _ => Some(Message::ChangeAppTheme(AppTheme::Light)),
                    })
                    .style(move |theme: &Theme, status| {
                        let base = match current_theme {
                            AppTheme::Light => button::primary(theme, button::Status::Active),
                            _ => button::alt(theme, status),
                        };

                        button::Style {
                            border: Border {
                                radius: Radius::from(0),
                                width: 0.0,
                                ..Default::default()
                            },
                            ..base
                        }
                    }),
                button(icon(icon::DARK_MODE_ICON).center())
                    .width(Length::Fill)
                    .on_press_maybe(match current_theme {
                        AppTheme::Dark => None,
                        _ => Some(Message::ChangeAppTheme(AppTheme::Dark)),
                    })
                    .style(move |theme: &Theme, status| {
                        let base = match current_theme {
                            AppTheme::Dark => button::primary(theme, button::Status::Active),
                            _ => button::alt(theme, status),
                        };

                        button::Style {
                            border: Border {
                                radius: Radius::from(0).right(6),
                                width: 0.0,
                                ..Default::default()
                            },
                            ..base
                        }
                    }),
            ],
            space().height(Length::Fixed(20.0)),
            button(
                text(t!(global_state, "system"))
                    .width(Length::Fill)
                    .center()
            )
            .on_press_maybe(match current_language {
                AppLanguage::System => None,
                _ => Some(Message::ChangeAppLanguage(AppLanguage::System)),
            })
            .width(Length::Fill)
            .padding(Padding::from(0).vertical(10))
            .style(move |theme: &Theme, status| {
                let base = match current_language {
                    AppLanguage::System => button::primary(theme, button::Status::Active),
                    _ => button::alt(theme, status),
                };

                button::Style {
                    border: Border {
                        radius: Radius::from(0).top(6),
                        width: 0.0,
                        ..Default::default()
                    },
                    ..base
                }
            }),
            button(text("English").width(Length::Fill).center())
                .on_press_maybe(match current_language {
                    AppLanguage::English => None,
                    _ => Some(Message::ChangeAppLanguage(AppLanguage::English)),
                })
                .width(Length::Fill)
                .padding(Padding::from(0).vertical(10))
                .style(move |theme: &Theme, status| {
                    let base = match current_language {
                        AppLanguage::English => button::primary(theme, button::Status::Active),
                        _ => button::alt(theme, status),
                    };

                    button::Style {
                        border: Border {
                            radius: Radius::from(0),
                            width: 0.0,
                            ..Default::default()
                        },
                        ..base
                    }
                }),
            button(text("Português").width(Length::Fill).center())
                .on_press_maybe(match current_language {
                    AppLanguage::Portuguese => None,
                    _ => Some(Message::ChangeAppLanguage(AppLanguage::Portuguese)),
                })
                .width(Length::Fill)
                .padding(Padding::from(0).vertical(10))
                .style(move |theme: &Theme, status| {
                    let base = match current_language {
                        AppLanguage::Portuguese => button::primary(theme, button::Status::Active),
                        _ => button::alt(theme, status),
                    };

                    button::Style {
                        border: Border {
                            radius: Radius::from(0).bottom(6),
                            width: 0.0,
                            ..Default::default()
                        },
                        ..base
                    }
                }),
            space().height(Length::Fixed(20.0)),
            button(text(t!(global_state, "close")).width(Length::Fill).center())
                .on_press(Message::CloseModal)
                .width(Length::Fill)
        ]
        .into()
    }
}
