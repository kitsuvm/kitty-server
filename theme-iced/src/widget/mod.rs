//! This module contains all the widgets used in the application.

use iced_core::Element;
pub use icon::icon;
pub use text::text;

pub mod application;
pub mod icon;
pub mod sidebar;
pub mod text;
pub mod window;
pub mod window_background;
pub mod window_bar;
pub mod window_button;
pub mod window_resize;

/// Creates a new [`Window`] widget with the given content.
pub fn window_background<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_background::WindowBackground<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_background::Catalog + iced_widget::container::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as iced_widget::container::Catalog>::Class<'a>:
        From<iced_widget::container::StyleFn<'a, Theme>>,
{
    window_background::WindowBackground::new(content)
}

/// Creates a new [`WindowBar`] widget with the given content.
pub fn window_bar<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_bar::WindowBar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_widget::container::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as iced_widget::container::Catalog>::Class<'a>:
        From<iced_widget::container::StyleFn<'a, Theme>>,
{
    window_bar::WindowBar::new(content)
}

/// Creates a new [`WindowResize`] widget with the given content.
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
    Theme: window_button::Catalog + iced_widget::button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as iced_widget::button::Catalog>::Class<'a>:
        From<iced_widget::button::StyleFn<'a, Theme>>,
{
    window_button::WindowButton::new(content)
}

/// Creates a new [`Window`] widget with the given content.
pub fn window<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window::Window<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_core::theme::Base
        + window_background::Catalog
        + window_button::Catalog
        + iced_widget::button::Catalog
        + iced_widget::container::Catalog
        + iced_widget::text::Catalog
        + 'a,
    Renderer: iced_core::text::Renderer + 'a,
    <Renderer as iced_core::text::Renderer>::Font: From<iced_core::Font>,
    <Theme as window_button::Catalog>::Class<'a>: From<window_button::StyleFn<'a, Theme>>,
    <Theme as iced_widget::button::Catalog>::Class<'a>:
        From<iced_widget::button::StyleFn<'a, Theme>>,
    <Theme as iced_widget::container::Catalog>::Class<'a>:
        From<iced_widget::container::StyleFn<'a, Theme>>,
{
    window::Window::new(content)
}

/// Creates a new [`sidebar`] widget with the given content.
pub fn sidebar<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> sidebar::Sidebar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: sidebar::Catalog + iced_widget::container::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as iced_widget::container::Catalog>::Class<'a>:
        From<iced_widget::container::StyleFn<'a, Theme>>,
{
    sidebar::Sidebar::new(content)
}
