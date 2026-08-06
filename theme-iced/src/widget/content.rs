//! A widget that wraps content in a container with optional max width and padding.

use iced_core::{Element, Length, Padding, Pixels};
use iced_widget::container;

/// A trait that defines a catalog for the content container.
pub trait Catalog: container::Catalog {
    /// Converts a style function into a class for the content container.
    fn into_class<'a>(style: impl Fn(&Self) -> container::Style + 'a) -> Self::Class<'a>;
}

/// A widget that wraps content in a container with optional max width and padding.
pub struct Content<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    /// The content of the widget.
    content: Element<'a, Message, Theme, Renderer>,
    /// The maximum width of the content.
    content_max_width: Option<Pixels>,
    /// The padding of the content.
    content_padding: Option<Padding>,
}

impl<'a, Message, Theme, Renderer> Content<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    /// Creates a new [`Content`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            content_max_width: Some(200.into()),
            content_padding: Some(Padding::from(0).top(10)),
        }
    }

    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.content_max_width = Some(max_width.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.content_padding = Some(padding.into());
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Content<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    fn from(content: Content<'a, Message, Theme, Renderer>) -> Self {
        let mut child = container(content.content);

        if let Some(max_width) = content.content_max_width {
            child = child.max_width(max_width);
        }

        if let Some(padding) = content.content_padding {
            child = child.padding(padding);
        }

        container(child)
            .center_x(Length::Fill)
            .height(Length::Fill)
            .class(Theme::into_class(container::transparent))
            .into()
    }
}
