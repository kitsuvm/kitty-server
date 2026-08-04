//! Animated widgets using Kitty Theme for Iced.

use iced_core::Element;

use crate::widget::window_background;

pub mod window;
pub mod window_button;

/// Creates a new [`WindowButton`] widget with the given content.
pub fn window_button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> window_button::WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: window_button::Catalog + iced_widget::button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as window_button::Catalog>::Class<'a>: Into<window_button::StyleFn<'a, Theme>>,
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
    <Theme as window_background::Catalog>::Class<'a>: Into<window_background::StyleFn<'a, Theme>>,
    <Theme as window_button::Catalog>::Class<'a>:
        From<window_button::StyleFn<'a, Theme>> + Into<window_button::StyleFn<'a, Theme>>,
    <Theme as iced_widget::container::Catalog>::Class<'a>:
        From<iced_widget::container::StyleFn<'a, Theme>>,
    <Theme as iced_widget::button::Catalog>::Class<'a>: From<iced_widget::button::StyleFn<'a, Theme>>
        + Into<iced_widget::button::StyleFn<'a, Theme>>,
{
    window::Window::new(content)
}
