//! A scaffold widget.

use iced_core::{Element, Length, Padding, Pixels, renderer};
use iced_widget::{column, container, row};

/// A catalog of styles for the [`Scaffold`] widget.
pub trait Catalog {
    /// Returns the padding of the scaffold.
    fn padding() -> Padding;
    /// Returns the spacing between the content and the sidebar.
    fn spacing() -> Pixels;
}

/// A scaffold widget that can be used to create a layout with a sidebar and content.
pub struct Scaffold<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the scaffold.
    content: Element<'a, Message, Theme, Renderer>,
    /// The optional sidebar of the scaffold.
    sidebar: Option<Element<'a, Message, Theme, Renderer>>,
    /// The optional bottom sidebar of the scaffold.
    bottom_sidebar: Option<Element<'a, Message, Theme, Renderer>>,
    /// The padding of the scaffold.
    padding: Padding,
    /// The spacing between the content and the sidebar.
    spacing: Pixels,
}

impl<'a, Message, Theme, Renderer> Scaffold<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`Scaffold`] widget with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Scaffold<'a, Message, Theme, Renderer> {
        Scaffold {
            content: content.into(),
            padding: Theme::padding(),
            sidebar: None,
            bottom_sidebar: None,
            spacing: Theme::spacing(),
        }
    }

    /// Sets the optional sidebar of the scaffold.
    pub fn sidebar(
        mut self,
        sidebar: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Scaffold<'a, Message, Theme, Renderer> {
        self.sidebar = Some(sidebar.into());
        self
    }

    /// Sets the optional bottom sidebar of the scaffold.
    pub fn bottom_sidebar(
        mut self,
        bottom_sidebar: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Scaffold<'a, Message, Theme, Renderer> {
        self.bottom_sidebar = Some(bottom_sidebar.into());
        self
    }

    /// Sets the padding of the scaffold.
    pub fn padding(
        mut self,
        padding: impl Into<Padding>,
    ) -> Scaffold<'a, Message, Theme, Renderer> {
        self.padding = padding.into();
        self
    }

    /// Sets the spacing between the content and the sidebar of the scaffold.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Scaffold<'a, Message, Theme, Renderer> {
        self.spacing = spacing.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Scaffold<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(scaffold: Scaffold<'a, Message, Theme, Renderer>) -> Self {
        row![
            match (scaffold.sidebar, scaffold.bottom_sidebar) {
                (Some(sidebar), Some(bottom_sidebar)) => Some(Element::from(column![
                    container(sidebar).align_top(Length::Fill),
                    bottom_sidebar
                ])),
                (Some(sidebar), None) => Some(container(sidebar).height(Length::Fill).into()),
                (None, Some(bottom_sidebar)) =>
                    Some(row![container(bottom_sidebar).align_bottom(Length::Fill)].into()),
                (None, None) => None,
            },
            container(scaffold.content)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .style(container::transparent)
        ]
        .padding(scaffold.padding)
        .spacing(scaffold.spacing)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
