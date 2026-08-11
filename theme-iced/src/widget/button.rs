use iced_core::{Element, Length, Padding, renderer};
use iced_widget::button;
pub use iced_widget::button::{Status, Style, StyleFn};

use crate::BaseExtended;

/// A set of parameters for a button widget.
#[derive(Debug, Clone, Copy, Default)]
pub struct Parameters {
    /// The width of the button.
    pub width: Option<Length>,
    /// The height of the button.
    pub height: Option<Length>,
    /// The padding of the button.
    pub padding: Option<Padding>,
    /// Whether the button is animated or not.
    pub animated: bool,
    /// The animation mode of the button.
    pub animation: Option<iced_anim::animated::Mode>,
}

/// A catalog of button styles.
pub trait Catalog: button::Catalog {
    /// Returns the parameters for a button widget.
    fn default_parameters() -> Parameters;
}

/// A button widget.
pub struct Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the button.
    content: Element<'a, Message, Theme, Renderer>,
    /// The width of the button.
    width: Option<Length>,
    /// The height of the button.
    height: Option<Length>,
    /// The padding of the button.
    padding: Option<Padding>,
    /// The message to send when the button is pressed.
    on_press: Option<Message>,
    /// Whether the button is animated or not.
    animated: bool,
    /// The animation mode of the button.
    animation: Option<iced_anim::animated::Mode>,
    /// The class of the button.
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`Button`] widget.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let parameters = Theme::default_parameters();

        Self {
            content: content.into(),
            width: parameters.width,
            height: parameters.height,
            padding: parameters.padding,
            animated: parameters.animated,
            animation: parameters.animation,
            on_press: None,
            class: Theme::default(),
        }
    }

    /// Sets the width of the button.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the button.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the padding of the button.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the on_press message for the button.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }

    /// Sets the on_press message for the button, allowing it to be optional.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press;
        self
    }

    /// Sets whether the button is animated or not.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the animation mode of the button.
    pub fn animation(mut self, animation: impl Into<iced_anim::animated::Mode>) -> Self {
        self.animation = Some(animation.into());
        self
    }

    /// Sets the style of the button.
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the class of the button.
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(value: Button<'a, Message, Theme, Renderer>) -> Self {
        if value.animated {
            let mut content = iced_anim::widget::button(value.content).class(value.class);

            if let Some(on_press) = value.on_press {
                content = content.on_press(on_press);
            }

            if let Some(width) = value.width {
                content = content.width(width);
            }

            if let Some(height) = value.height {
                content = content.height(height);
            }

            if let Some(padding) = value.padding {
                content = content.padding(padding);
            }

            if let Some(animation) = value.animation {
                content = content.animation(animation);
            }

            content.into()
        } else {
            let mut content = button(value.content).class(value.class);

            if let Some(on_press) = value.on_press {
                content = content.on_press(on_press);
            }

            if let Some(width) = value.width {
                content = content.width(width);
            }

            if let Some(height) = value.height {
                content = content.height(height);
            }

            if let Some(padding) = value.padding {
                content = content.padding(padding);
            }

            content.into()
        }
    }
}

/// Returns a primary button style for the given theme and status.
pub fn primary<'a, Theme>(theme: &Theme, status: Status) -> Style
where
    Theme: BaseExtended + Catalog + 'a,
{
    let pallete = theme.palette_extended();
    let default_class = <Theme as button::Catalog>::default();
    let default_style = <Theme as button::Catalog>::style(theme, &default_class, status);

    match status {
        Status::Active => Style {
            background: Some(pallete.primary.base.color.into()),
            text_color: pallete.primary.base.text,
            ..default_style
        },
        Status::Hovered | Status::Pressed => Style {
            background: Some(pallete.primary.weak.color.into()),
            text_color: pallete.primary.weak.text,
            ..default_style
        },
        Status::Disabled => Style {
            background: Some(pallete.primary.strong.color.into()),
            text_color: pallete.primary.strong.text,
            ..default_style
        },
    }
}
