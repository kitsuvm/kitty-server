//! This module defines the [`Screen`] trait and the [`ScreenState`] enum, which represent the different screens in the application.

use iced::{
    Border, Color, Element, Length, Renderer, color,
    widget::{container, mouse_area},
};
use kitty_theme_iced::{theme::Theme, widget::window_background};

use crate::Message;

pub mod server_add;

/// The trait that defines a modal in the application.
pub trait Modal {
    /// Returns the element to be displayed in the content area of the modal, or `None` if the modal is not currently active.
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer>;
}

macro_rules! impl_modal {
    (
        $state_enum:ident, $kind_enum:ident {
            $( $variant:ident($state_type:ty) ),* $(,)?
        }
    ) => {
        /// State containing a modal's state.
        #[derive(Debug, Clone, Default)]
        pub enum $state_enum {
            #[default]
            None,
            $( $variant($state_type), )*
        }

        /// The kind of modal.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub enum $kind_enum {
            #[default]
            None,
            $( $variant, )*
        }

        impl $state_enum {
            fn content<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
                match self {
                    Self::None => None,
                    $(
                        Self::$variant(state) => Some(state.content()),
                    )*
                }
            }
        }

        impl From<$kind_enum> for $state_enum {
            fn from(modal_kind: $kind_enum) -> Self {
                match modal_kind {
                    $kind_enum::None => Self::None,
                    $(
                        $kind_enum::$variant => Self::$variant(<$state_type>::default()),
                    )*
                }
            }
        }

        impl From<&$state_enum> for $kind_enum {
            fn from(state: &$state_enum) -> Self {
                match state {
                    $state_enum::None => Self::None,
                    $(
                        $state_enum::$variant(_) => $kind_enum::$variant,
                    )*
                }
            }
        }
    };
}

impl_modal! {
    ModalState, ModalKind {
        ServerAdd(server_add::State),
    }
}

pub fn modal(state: &ModalState) -> Option<Element<'_, Message, Theme, Renderer>> {
    state.content().map(|content| {
        let content_size = content.as_widget().size_hint();

        mouse_area(
            container(
                window_background(content)
                    .width(content_size.width.fluid())
                    .height(content_size.height.fluid()),
            )
            .center(Length::Fill)
            .style(|theme: &Theme| container::Style {
                background: Some(color!(0x000000, 0.5).into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    radius: theme.window_radius().into(),
                    width: theme.window_border_width(),
                },
                ..Default::default()
            }),
        )
        .on_press(Message::CloseModal)
        .into()
    })
}
