//! A button with an icon.

use iced_core::{
    Element, Length, Pixels,
    text::{Fragment, IntoFragment},
};
use iced_widget::text;

use crate::{
    renderer::TextRenderer,
    widget::{button, icon},
};

#[derive(Debug, Clone, Copy)]
/// A set of parameters for an [`IconButton`] widget.
pub struct Parameters {
    /// The size of the button.
    pub size: Length,
    /// The size of the icon.
    pub icon_size: Option<Pixels>,
    /// Whether the button is animated or not.
    pub animated: bool,
    /// The animation mode of the button.
    pub animation: Option<iced_anim::animated::Mode>,
}

/// A trait for defining the style for the [`IconButton`] widget.
pub trait Catalog: button::Catalog + text::Catalog {
    /// Returns the default parameters for the [`IconButton`] widget.
    fn default_parameters() -> Parameters;

    /// Returns the default class for the [`IconButton`] widget.
    fn default<'a>() -> <Self as iced_widget::button::Catalog>::Class<'a>;

    /// Returns the style for the [`IconButton`] widget.
    fn style<'a>(
        &self,
        class: <Self as iced_widget::button::Catalog>::Class<'a>,
        status: button::Status,
    ) -> button::Style;
}

/// A button with an icon.
pub struct IconButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: TextRenderer + 'a,
{
    content: Fragment<'a>,
    size: Length,
    icon_size: Option<Pixels>,
    on_press: Option<Message>,
    class: <Theme as iced_widget::button::Catalog>::Class<'a>,
    animated: bool,
    animation: Option<iced_anim::animated::Mode>,
    _marker: std::marker::PhantomData<Renderer>,
}

impl<'a, Message, Theme, Renderer> IconButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: TextRenderer + 'a,
{
    /// Creates a new [`IconButton`] widget.
    pub fn new(content: impl IntoFragment<'a>) -> Self {
        let parameters = <Theme as Catalog>::default_parameters();

        Self {
            content: content.into_fragment(),
            on_press: None,
            class: <Theme as Catalog>::default(),
            size: parameters.size,
            icon_size: parameters.icon_size,
            animated: parameters.animated,
            animation: parameters.animation,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn on_press_mut(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    pub fn size(mut self, size: impl Into<Length>) -> Self {
        self.size = size.into();
        self
    }

    pub fn icon_size(mut self, icon_size: impl Into<Pixels>) -> Self {
        self.icon_size = Some(icon_size.into());
        self
    }

    /// Sets the style of the button.
    pub fn style(mut self, style: impl Fn(&Theme, button::Status) -> button::Style + 'a) -> Self
    where
        <Theme as iced_widget::button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as button::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the class of the button.
    pub fn class(
        mut self,
        class: impl Into<<Theme as iced_widget::button::Catalog>::Class<'a>>,
    ) -> Self {
        self.class = class.into();
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn animation(mut self, animation: Option<iced_anim::animated::Mode>) -> Self {
        self.animation = animation;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<IconButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: TextRenderer + 'a,
{
    fn from(value: IconButton<'a, Message, Theme, Renderer>) -> Self {
        let mut content = icon(value.content).center();

        if let Some(icon_size) = value.icon_size {
            content = content.size(icon_size);
        }

        let mut button = button(content)
            .padding(0)
            .width(value.size)
            .height(value.size)
            .class(value.class)
            .animated(value.animated);

        if let Some(on_press) = value.on_press {
            button = button.on_press(on_press);
        }

        if let Some(animation) = value.animation {
            button = button.animation(animation);
        }

        button.into()
    }
}
