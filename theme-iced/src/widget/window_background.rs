//! Container styles for windows.

use iced_core::{Element, Length, Padding, Pixels, renderer};
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

/// Represents the parameters for a window container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Parameters {
    /// The width of the window container.
    pub width: Option<Length>,
    /// The height of the window container.
    pub height: Option<Length>,
    /// The maximum width of the window container.
    pub max_width: Option<Pixels>,
    /// The maximum height of the window container.
    pub max_height: Option<Pixels>,
    /// The padding of the window container.
    pub padding: Option<Padding>,
}

/// Represents a catalog of styles for window containers.
pub trait Catalog: container::Catalog {
    /// The class of the window container.
    type SuperClass<'a>;

    /// Returns the parameters for a window container.
    fn default_parameters() -> Parameters;

    /// Returns the default style class for a window container.
    fn default<'a>() -> Self::SuperClass<'a>;

    /// Returns the style for a container based on the given status.
    fn style(&self, class: &Self::SuperClass<'_>, status: Status) -> container::Style;

    /// Converts a style function into a class for the window container.
    fn into_class<'a>(class: Self::SuperClass<'a>, status: Status) -> Self::Class<'a>;
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
    /// The width of the window container.
    width: Option<Length>,
    /// The height of the window container.
    height: Option<Length>,
    /// The maximum width of the window container.
    max_width: Option<Pixels>,
    /// The maximum height of the window container.
    max_height: Option<Pixels>,
    /// The padding of the window container.
    padding: Option<Padding>,
}

impl<'a, Message, Theme, Renderer> WindowBackground<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`Window`] widget with the given content and status.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let parameters = Theme::default_parameters();

        Self {
            content: content.into(),
            status: Default::default(),
            class: <Theme as Catalog>::default(),
            width: parameters.width,
            height: parameters.height,
            max_width: parameters.max_width,
            max_height: parameters.max_height,
            padding: parameters.padding,
        }
    }

    /// Sets the status of the window.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Sets the width of the window container.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the window container.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the maximum width of the window container.
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    /// Sets the maximum height of the window container.
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    /// Sets the padding of the window container.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
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
        let mut widget =
            container(window.content).class(Theme::into_class(window.class, window.status));

        if let Some(width) = window.width {
            widget = widget.width(width);
        }

        if let Some(height) = window.height {
            widget = widget.height(height);
        }

        if let Some(max_width) = window.max_width {
            widget = widget.max_width(max_width);
        }

        if let Some(max_height) = window.max_height {
            widget = widget.max_height(max_height);
        }

        if let Some(padding) = window.padding {
            widget = widget.padding(padding);
        }

        widget.into()
    }
}
