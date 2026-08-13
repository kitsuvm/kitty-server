//! This module defines the [`Screen`] trait and the [`ScreenState`] enum, which represent the different screens in the application.

use iced::{
    Border, Color, Element, Length, Renderer, Task, color,
    widget::{container, opaque},
};
use kitty_theme_iced::{theme::Theme, widget::window_background};

use crate::{
    application::{message::Message, screen::ScreenState, state::GlobalState},
    impl_modal,
};

pub mod macros;
pub mod server_add;

/// The trait that defines a modal in the application.
pub trait Modal {
    /// Returns the element to be displayed in the content area of the modal, or `None` if the modal is not currently active.
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer>;

    /// Handles a text input change event for the modal.
    fn handle_text_input(&mut self, _id: usize, _value: String) {
        // Default implementation does nothing
    }

    /// Handles the submission of the modal.
    fn handle_submit(
        &mut self,
        _global_state: &mut GlobalState,
        _screen: &mut ScreenState,
    ) -> (bool, Task<Message>) {
        (false, Task::none())
    }
}

impl_modal! {
    ModalState, ModalKind {
        ServerAdd(server_add::State),
    }
}

pub fn modal(state: &ModalState) -> Option<Element<'_, Message, Theme, Renderer>> {
    state.content().map(|content| {
        let content_size = content.as_widget().size_hint();

        opaque(
            container(
                window_background(content)
                    .width(content_size.width.fluid())
                    .height(content_size.height.fluid())
                    .max_width(310)
                    .padding(16),
            )
            .center(Length::Fill)
            .style(|theme: &Theme| container::Style {
                background: Some(color!(0x000000, 0.5).into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    radius: theme.window_radius().into(),
                    width: theme.window_border_width(),
                },
                ..Default::default()
            }),
        )
        .into()
    })
}
