//! This module defines the [`Screen`] trait and the [`ScreenState`] enum, which represent the different screens in the application.

use iced::{Element, Renderer};
use kitty_theme_iced::theme::Theme;

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
