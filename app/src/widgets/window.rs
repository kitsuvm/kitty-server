//! This module contains a widget that creates a window with the given content and resize messages.

use iced::{
    Element, Length, Renderer,
    mouse::Interaction,
    widget::{column, mouse_area, row, space},
};

use crate::{Message, theme::Theme};

/// Creates a window with the given content and resize messages.
pub fn window<'a>(
    on_resize: Option<Resize<Message>>,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::container::Catalog + 'a,
{
    let Some(resize) = on_resize else {
        return content.into();
    };

    column![
        row![
            create_button(resize.top_left, true, true, true),
            create_button(resize.top, false, true, false),
            create_button(resize.top_right, true, true, false)
        ],
        row![
            create_button(resize.left, true, false, false),
            content.into(),
            create_button(resize.right, true, false, false)
        ],
        row![
            create_button(resize.bottom_left, true, true, false),
            create_button(resize.bottom, false, true, false),
            create_button(resize.bottom_right, true, true, true)
        ],
    ]
    .into()
}

/// Creates a button with the given message and size.
fn create_button<'a>(
    message: Option<Message>,
    width: bool,
    height: bool,
    bottom_left: bool,
) -> Element<'a, Message, Theme, Renderer> {
    let size = 5;

    match (message, width, height) {
        (Some(msg), true, true) => mouse_area(space().width(size).height(size))
            .interaction(if bottom_left {
                Interaction::ResizingDiagonallyDown
            } else {
                Interaction::ResizingDiagonallyUp
            })
            .on_press(msg)
            .into(),
        (Some(msg), true, false) => mouse_area(space().width(size).height(Length::Fill))
            .interaction(Interaction::ResizingHorizontally)
            .on_press(msg)
            .into(),
        (Some(msg), false, true) => mouse_area(space().width(Length::Fill).height(size))
            .interaction(Interaction::ResizingVertically)
            .on_press(msg)
            .into(),
        (Some(msg), false, false) => mouse_area(space().width(Length::Fill).height(Length::Fill))
            .on_press(msg)
            .into(),
        (None, true, true) => space().width(size).height(size).into(),
        (None, true, false) => space().width(size).height(Length::Fill).into(),
        (None, false, true) => space().width(Length::Fill).height(size).into(),
        (None, false, false) => space().width(Length::Fill).height(Length::Fill).into(),
    }
}

/// A message that indicates the direction in which a window should be resized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resize<Message> {
    /// The message to send when the top edge of the window is dragged.
    top: Option<Message>,
    /// The message to send when the bottom edge of the window is dragged.
    bottom: Option<Message>,
    /// The message to send when the left edge of the window is dragged.
    left: Option<Message>,
    /// The message to send when the right edge of the window is dragged.
    right: Option<Message>,
    /// The message to send when the top left corner of the window is dragged.
    top_left: Option<Message>,
    /// The message to send when the top right corner of the window is dragged.
    top_right: Option<Message>,
    /// The message to send when the bottom left corner of the window is dragged.
    bottom_left: Option<Message>,
    /// The message to send when the bottom right corner of the window is dragged.
    bottom_right: Option<Message>,
}

impl<Message> Default for Resize<Message> {
    fn default() -> Self {
        Self {
            top: None,
            bottom: None,
            left: None,
            right: None,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

impl<Message> Resize<Message> {
    /// Creates a new [`Resize`] with no messages.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the message to send when the top edge of the window is dragged.
    pub fn top(mut self, message: Message) -> Self {
        self.top = Some(message);
        self
    }

    /// Sets the message to send when the bottom edge of the window is dragged.
    pub fn bottom(mut self, message: Message) -> Self {
        self.bottom = Some(message);
        self
    }

    /// Sets the message to send when the left edge of the window is dragged.
    pub fn left(mut self, message: Message) -> Self {
        self.left = Some(message);
        self
    }

    /// Sets the message to send when the right edge of the window is dragged.
    pub fn right(mut self, message: Message) -> Self {
        self.right = Some(message);
        self
    }

    /// Sets the message to send when the top left corner of the window is dragged.
    pub fn top_left(mut self, message: Message) -> Self {
        self.top_left = Some(message);
        self
    }

    /// Sets the message to send when the top right corner of the window is dragged.
    pub fn top_right(mut self, message: Message) -> Self {
        self.top_right = Some(message);
        self
    }

    /// Sets the message to send when the bottom left corner of the window is dragged.
    pub fn bottom_left(mut self, message: Message) -> Self {
        self.bottom_left = Some(message);
        self
    }

    /// Sets the message to send when the bottom right corner of the window is dragged.
    pub fn bottom_right(mut self, message: Message) -> Self {
        self.bottom_right = Some(message);
        self
    }
}
