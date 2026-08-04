//! This module contains all the widgets used in the application.

pub use icon::icon;
pub use text::text;

pub mod application;
pub mod icon;
pub mod text;
pub mod window_background;
//pub mod window_bar;
pub mod window_button;
pub mod window_resize;

/// Creates a new [`Window`] widget with the given content.
pub fn window_background<'a, Message, Theme, Renderer>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
) -> window_background::WindowBackground<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_background::Catalog + iced::widget::container::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as window_background::Catalog>::Class<'a>: Into<window_background::StyleFn<'a, Theme>>,
{
    window_background::WindowBackground::new(content)
}

/// Creates a new [`WindowResize`] widget with the given content.
pub fn window_resize<'a, Message, Theme, Renderer>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
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
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
) -> window_button::WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_button::Catalog + iced::widget::button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as window_button::Catalog>::Class<'a>: Into<window_button::StyleFn<'a, Theme>>,
{
    window_button::WindowButton::new(content)
}
