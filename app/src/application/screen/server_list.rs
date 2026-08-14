//! The server list screen.

use iced::{
    Border, Element, Length, Padding, Renderer,
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{Column, column, container, scrollable, text_input},
};
use kitty_theme_iced::{
    theme::Theme,
    widget::{button, icon, icon_button, text},
};

use crate::{
    application::{
        message::Message,
        modal::ModalKind,
        screen::Screen,
        state::{GlobalState, Lazy},
    },
    resources::hosts::{Host, HostsManager},
    t,
};

/// The state of the server list screen.
#[derive(Debug, Clone, Default)]
pub struct State {
    /// The search query entered by the user.
    pub search_query: String,
    /// The connection state of the application.
    pub internal: Lazy<HostsManager>,
}

impl Screen for State {
    fn content<'a>(&'a self, global_state: &GlobalState) -> Element<'a, Message, Theme, Renderer> {
        match &self.internal {
            Lazy::Loading => container(text(t!(global_state, "loading")))
                .center(Length::Fill)
                .style(container::transparent)
                .into(),
            Lazy::Data(hosts_manager) if hosts_manager.is_empty() => container(
                text(t!(global_state, "no-client-found")).style(|theme: &Theme| text::Style {
                    color: Some(theme.extended().background.weaker.text),
                }),
            )
            .style(container::transparent)
            .center(Length::Fill)
            .into(),
            Lazy::Data(hosts_manager) => {
                let hosts = hosts_manager.get();

                let last_index = hosts.len() - 1;

                container(
                    container(scrollable(Column::with_children(
                        hosts.iter().enumerate().map(|(index, host)| {
                            button(
                                container(match host.subtitle() {
                                    Some(subtitle) => Element::from(column![
                                        text(host.title()),
                                        text(subtitle).style(|theme: &Theme| text::Style {
                                            color: Some(theme.extended().background.weaker.text),
                                        }),
                                    ]),
                                    None => text(host.title()).into(),
                                })
                                .style(container::transparent)
                                .center_y(Length::Fill),
                            )
                            .style(move |theme: &Theme, status: button::Status| {
                                let style = button::alt(theme, status);

                                button::Style {
                                    border: Border {
                                        radius: if index == 0 {
                                            Radius::from(0).top(8)
                                        } else if index == last_index {
                                            Radius::from(0).bottom(8)
                                        } else {
                                            Radius::from(0)
                                        },
                                        ..Default::default()
                                    },
                                    ..style
                                }
                            })
                            .height(60)
                            .width(Length::Fill)
                            .on_press(Message::Refresh)
                            .into()
                        }),
                    )))
                    .max_width(500)
                    .width(Length::Fill),
                )
                .padding(Padding::from(10).top(30))
                .center_x(Length::Fill)
                .into()
            }
            Lazy::Error(e) => container(
                column![
                    text(t!(global_state, "error-occurred")).style(text::danger),
                    text(format!("{:?}", e)).style(|theme: &Theme| text::Style {
                        color: Some(theme.extended().background.weaker.text),
                    }),
                    button(text(t!(global_state, "retry"))).on_press(Message::Refresh)
                ]
                .align_x(Horizontal::Center),
            )
            .center(Length::Fill)
            .into(),
        }
    }

    fn handle_text_input(&mut self, _id: usize, value: String) {
        self.search_query = value;
    }

    fn window_bar_side_width(&self) -> Option<Length> {
        Some(150.into())
    }

    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        Some(
            container(
                icon_button(icon::ADD_ICON).on_press(Message::OpenModal(ModalKind::ServerAdd)),
            )
            .style(container::transparent)
            .padding(Padding::from(0).left(8))
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .into(),
        )
    }

    fn window_bar_center<'a>(
        &'a self,
        global_state: &GlobalState,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        Some(
            container(
                text_input(&t!(global_state, "search"), &self.search_query)
                    .icon(icon::to_text_input_icon(icon::SEARCH_ICON, 0.0, None))
                    .on_input(|v| Message::ChangedTextInput(0, v)),
            )
            .max_width(360)
            .padding(Padding::from(0).top(8))
            .into(),
        )
    }
}
