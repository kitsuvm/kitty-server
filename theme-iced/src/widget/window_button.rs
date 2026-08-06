//! Custom window button widget.

use iced_core::{Element, color, renderer, theme::Base};
use iced_widget::button;

/// The position of the window button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// The button is aligned to the left.
    Left,
    /// The button is aligned to the center.
    Center,
    /// The button is aligned to the right.
    Right,
}

/// The status of the window button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// The position of the button.
    pub button_position: Position,
    /// If the window buttons are aligned to the left or right of the window.
    pub left_buttons: bool,
    /// If the button should have no rounded corners.
    pub no_rounded_corner: bool,
}

/// A catalog of styles for window buttons.
pub trait Catalog: button::Catalog {
    /// The class of the window button.
    type SuperClass<'a>;

    /// Returns the default style of the window button.
    fn default<'a>() -> Self::SuperClass<'a>;

    /// Returns the style of the window button based on its class and status.
    fn style(
        &self,
        class: &Self::SuperClass<'_>,
        status: Status,
        button_status: button::Status,
    ) -> button::Style;

    /// Converts a style function into a class for the window button.
    fn into_class<'a>(class: Self::SuperClass<'a>, status: Status) -> Self::Class<'a>;
}

/// A type alias for a style function that takes a theme and a status and returns a button style.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status, button::Status) -> button::Style + 'a>;

/// A window button widget to create new buttons.
pub struct WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the button.
    content: Element<'a, Message, Theme, Renderer>,
    /// The position of the button.
    position: Position,
    /// If the window buttons are aligned to the left or right of the window.
    left_buttons: bool,
    /// The message to send when the button is pressed.
    on_press: Option<Message>,
    /// The style function for the button.
    class: Theme::SuperClass<'a>,
    /// If the button should have no rounded corners.
    no_rounded_corner: bool,
    /// If the button should be animated.
    animated: bool,
}

impl<'a, Message, Theme, Renderer> WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`WindowButton`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            position: Position::Center,
            left_buttons: false,
            class: <Theme as Catalog>::default(),
            no_rounded_corner: false,
            animated: false,
            on_press: None,
        }
    }

    /// Sets the message to send when the button is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the position of the button.
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets whether the window buttons are aligned to the left or right of the window.
    pub fn left_buttons(mut self, left_buttons: bool) -> Self {
        self.left_buttons = left_buttons;
        self
    }

    /// Sets the style function for the button.
    pub fn style(
        mut self,
        style: impl Fn(&Theme, Status, button::Status) -> button::Style + 'a,
    ) -> Self
    where
        Theme::SuperClass<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the class of the button.
    pub fn class(mut self, class: impl Into<Theme::SuperClass<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets whether the button should have no rounded corners.
    pub fn no_rounded_corner(mut self, no_rounded_corner: bool) -> Self {
        self.no_rounded_corner = no_rounded_corner;
        self
    }

    /// Sets whether the button should be animated.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<WindowButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(window_button: WindowButton<'a, Message, Theme, Renderer>) -> Self {
        let class = Theme::into_class(
            window_button.class,
            Status {
                button_position: window_button.position,
                left_buttons: window_button.left_buttons,
                no_rounded_corner: window_button.no_rounded_corner,
            },
        );

        match window_button.animated {
            true => {
                let mut button = iced_anim::widget::button(window_button.content).class(class);

                if let Some(message) = window_button.on_press {
                    button = button.on_press(message);
                }

                button.into()
            }
            false => {
                let mut button = button(window_button.content).class(class);

                if let Some(message) = window_button.on_press {
                    button = button.on_press(message);
                }

                button.into()
            }
        }
    }
}

/// Returns a style for a danger button based on the given theme and status.
pub fn danger<'a, Theme>(
    theme: &Theme,
    status: Status,
    button_status: button::Status,
) -> button::Style
where
    Theme: Catalog + Base + 'a,
{
    let base =
        <Theme as Catalog>::style(theme, &<Theme as Catalog>::default(), status, button_status);

    if matches!(
        button_status,
        button::Status::Pressed | button::Status::Hovered
    ) {
        base.with_background(theme.palette().map_or(color!(0xff000), |p| p.danger))
    } else {
        base
    }
}
