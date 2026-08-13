//! This module contains all the widgets used in the application.

use iced_core::Element;
pub use icon::icon;

use crate::renderer;

pub mod application;
pub mod button;
pub mod content;
pub mod icon;
pub mod icon_button;
pub mod sidebar;
pub mod text;
pub mod window;
pub mod window_background;
pub mod window_bar;
pub mod window_button;
pub mod window_resize;

/// Creates a new [`window_background::WindowBackground`] widget with the given content.
pub fn window_background<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_background::WindowBackground<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_background::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    window_background::WindowBackground::new(content)
}

/// Creates a new [`window_bar::WindowBar`] widget with the given content.
pub fn window_bar<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_bar::WindowBar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_bar::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    window_bar::WindowBar::new(content)
}

/// Creates a new [`window_resize::WindowResize`] widget with the given content.
pub fn window_resize<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_resize::WindowResize<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    window_resize::WindowResize::new(content)
}

/// Creates a new [`WindowButton`] widget with the given content.
pub fn window_button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_button::WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    window_button::WindowButton::new(content)
}

/// Creates a new [`window::Window`] widget with the given content.
pub fn window<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window::Window<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window::Catalog + 'a,
    Renderer: renderer::TextRenderer + 'a,
{
    window::Window::new(content)
}

/// Creates a new [`sidebar::Sidebar`] widget with the given content.
pub fn sidebar<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> sidebar::Sidebar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: sidebar::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    sidebar::Sidebar::new(content)
}

/// Creates a new [`content::Content`] widget with the given content.
pub fn content<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> content::Content<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: content::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    content::Content::new(content)
}

/// Creates a new [`button::Button`] widget with the given content.
pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> button::Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    button::Button::new(content)
}

/// Creates a new [`icon_button::IconButton`] widget with the given content.
pub fn icon_button<'a, Message, Theme, Renderer>(
    content: impl iced_widget::text::IntoFragment<'a>,
) -> icon_button::IconButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: icon_button::Catalog + 'a,
    Renderer: renderer::TextRenderer + 'a,
{
    icon_button::IconButton::new(content)
}

/// Creates a new [`text::Text`] widget with the given content.
pub fn text<'a, Theme, Renderer>(
    text: impl text::IntoFragment<'a>,
) -> text::Text<'a, Theme, Renderer>
where
    Theme: text::Catalog + 'a,
    Renderer: iced_core::text::Renderer,
{
    text::Text::new(text)
}
