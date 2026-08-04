//! This module contains a widget that creates a window resize handles with the given content and resize messages.

use iced::{
    Element, Length,
    mouse::Interaction,
    widget::{column, mouse_area, row, space},
    window::Direction,
};
use iced_core::renderer;

/// Changes how the window resize handles are displayed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Handles {
    /// The window resize handles are disabled and not displayed.
    Disabled,
    /// The window resize handles are displayed but not interactive.
    Empty,
    /// The window resize handles are displayed and interactive.
    #[default]
    Clickable,
}

//// A widget that creates a window with the given content and resize messages.
pub struct WindowResize<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the window.
    content: Element<'a, Message, Theme, Renderer>,
    /// The message to send when the window is resized.
    on_resize: Option<Box<dyn Fn(Direction) -> Message + 'a>>,
    /// The size of the window resize handles.
    size: Length,
    /// The window resize handles.
    handles: Handles,
}

impl<'a, Message, Theme, Renderer> WindowResize<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`WindowResize`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            on_resize: None,
            handles: Handles::default(),
            size: 5.into(),
        }
    }

    /// Sets the message to send when the window is resized.
    pub fn on_resize(mut self, f: impl Fn(Direction) -> Message + 'a) -> Self {
        self.on_resize = Some(Box::new(f));
        self
    }

    /// Sets the window resize handles.
    pub fn handles(mut self, handles: Handles) -> Self {
        self.handles = handles;
        self
    }

    /// Sets the size of the window resize handles.
    pub fn size(mut self, size: impl Into<Length>) -> Self {
        self.size = size.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> From<WindowResize<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(window_resize: WindowResize<'a, Message, Theme, Renderer>) -> Self {
        if window_resize.handles == Handles::Disabled {
            return window_resize.content;
        }

        let on_resize = match window_resize.handles {
            Handles::Clickable => window_resize.on_resize,
            _ => None,
        };

        column![
            row![
                create_handle(
                    window_resize.size,
                    window_resize.size,
                    Interaction::ResizingDiagonallyUp,
                    on_resize.as_ref().map(|f| f(Direction::NorthWest)),
                ),
                create_handle(
                    Length::Fill,
                    window_resize.size,
                    Interaction::ResizingVertically,
                    on_resize.as_ref().map(|f| f(Direction::North)),
                ),
                create_handle(
                    window_resize.size,
                    window_resize.size,
                    Interaction::ResizingDiagonallyDown,
                    on_resize.as_ref().map(|f| f(Direction::NorthEast)),
                ),
            ],
            row![
                create_handle(
                    window_resize.size,
                    Length::Fill,
                    Interaction::ResizingHorizontally,
                    on_resize.as_ref().map(|f| f(Direction::West)),
                ),
                window_resize.content,
                create_handle(
                    window_resize.size,
                    Length::Fill,
                    Interaction::ResizingHorizontally,
                    on_resize.as_ref().map(|f| f(Direction::East)),
                ),
            ],
            row![
                create_handle(
                    window_resize.size,
                    window_resize.size,
                    Interaction::ResizingDiagonallyDown,
                    on_resize.as_ref().map(|f| f(Direction::SouthWest)),
                ),
                create_handle(
                    Length::Fill,
                    window_resize.size,
                    Interaction::ResizingVertically,
                    on_resize.as_ref().map(|f| f(Direction::South)),
                ),
                create_handle(
                    window_resize.size,
                    window_resize.size,
                    Interaction::ResizingDiagonallyUp,
                    on_resize.as_ref().map(|f| f(Direction::SouthEast)),
                ),
            ],
        ]
        .into()
    }
}

/// Creates a handle with the given size and interaction.
fn create_handle<'a, Message, Theme, Renderer>(
    width: impl Into<Length>,
    height: impl Into<Length>,
    interaction: Interaction,
    message: Option<Message>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    match message {
        Some(msg) => mouse_area(space().width(width).height(height))
            .interaction(interaction)
            .on_press(msg)
            .into(),
        None => space().width(width).height(height).into(),
    }
}
