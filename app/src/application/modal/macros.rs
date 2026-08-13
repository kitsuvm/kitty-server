//! This module contains helper macros for working modals.

#[macro_export]
/// This macro generates a modal state enum and a modal kind enum, along with the necessary implementations for the [`super::Modal`] trait and conversions between the two enums.
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
            /// Returns whether the modal is currently active (i.e., not `None`).
            pub fn is_active(&self) -> bool {
                !matches!(self, Self::None)
            }

            /// Returns the element to be displayed in the content area of the modal, or `None` if the modal is not currently active.
            pub fn content<'a>(&'a self) -> Option<iced::Element<'a, Message, Theme, Renderer>> {
                match self {
                    Self::None => None,
                    $(
                        Self::$variant(state) => Some(state.content()),
                    )*
                }
            }

            /// Handles a text input change event for the modal.
            pub fn handle_text_input(&mut self, id: usize, value: String) {
                match self {
                    Self::None => {},
                    $(
                        Self::$variant(state) => state.handle_text_input(id, value),
                    )*
                }
            }

            /// Handles the submission of the modal.
            pub fn handle_submit(&mut self, global_state: &mut crate::application::state::GlobalState, screen: &mut crate::application::screen::ScreenState) -> (bool, iced::Task<crate::application::Message>) {
                match self {
                    Self::None => (true, iced::Task::none()),
                    $(
                        Self::$variant(state) => state.handle_submit(global_state, screen),
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
