//! Container styles for windows.

use iced::{Element, Length, Padding, widget::container};
use iced_core::renderer;

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
pub trait Catalog {
    /// The class of the window container.
    type Class<'a>;

    /// Returns the default style class for a window container.
    fn default<'a>() -> Self::Class<'a>;

    /// Returns the style for a container based on the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> container::Style;

    /// Returns the padding for a window container.
    fn padding() -> Option<Padding> {
        None
    }
}

/// A window widget that contains content and applies styles based on the theme and status.
pub struct Window<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the window.
    content: Element<'a, Message, Theme, Renderer>,
    /// The status of the window.
    status: Status,
    /// The class of the window container.
    class: <Theme as Catalog>::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Window<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + 'a,
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
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the class of the window.
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Window<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
    <Theme as iced::widget::container::Catalog>::Class<'a>:
        From<iced::widget::container::StyleFn<'a, Theme>>,
{
    fn from(window: Window<'a, Message, Theme, Renderer>) -> Self {
        let widget = container(window.content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme| {
                <Theme as Catalog>::style(theme, &window.class, window.status)
            });

        if let Some(padding) = Theme::padding() {
            widget.padding(padding)
        } else {
            widget
        }
        .into()
    }
}
