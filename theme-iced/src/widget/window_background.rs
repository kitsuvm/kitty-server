//! Container styles for windows.

use iced_core::{Element, Length, Padding, renderer};
use iced_widget::container;

/// Represents the status of a window.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Status {
    /// The window is in its normal state.
    #[default]
    Normal,
    /// The window is maximized.
    Maximized,
}

/// A type alias for a style function that takes a theme and a status and returns a container style.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> container::Style + 'a>;

/// Represents a catalog of styles for window containers.
pub trait Catalog: container::Catalog {
    /// The class of the window container.
    type SuperClass<'a>;

    /// Returns the default style class for a window container.
    fn default<'a>() -> Self::SuperClass<'a>;

    /// Returns the style for a container based on the given status.
    fn style(&self, class: &Self::SuperClass<'_>, status: Status) -> container::Style;

    /// Converts a style function into a class for the window container.
    fn into_class<'a>(class: Self::SuperClass<'a>, status: Status) -> Self::Class<'a>;

    /// Returns the padding for a window container.
    fn padding() -> Option<Padding> {
        None
    }
}

/// A window widget that contains content and applies styles based on the theme and status.
pub struct WindowBackground<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the window.
    content: Element<'a, Message, Theme, Renderer>,
    /// The status of the window.
    status: Status,
    /// The class of the window container.
    class: <Theme as Catalog>::SuperClass<'a>,
}

impl<'a, Message, Theme, Renderer> WindowBackground<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`Window`] widget with the given content and status.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            status: Default::default(),
            class: <Theme as Catalog>::default(),
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Sets the style function for the window.
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> container::Style + 'a) -> Self
    where
        Theme::SuperClass<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the class of the window.
    pub fn class(mut self, class: impl Into<Theme::SuperClass<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> From<WindowBackground<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(window: WindowBackground<'a, Message, Theme, Renderer>) -> Self {
        let widget = container(window.content)
            .width(Length::Fill)
            .height(Length::Fill)
            .class(Theme::into_class(window.class, window.status));

        if let Some(padding) = Theme::padding() {
            widget.padding(padding)
        } else {
            widget
        }
        .into()
    }
}
