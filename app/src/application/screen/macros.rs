//! This module contains helper macros for working with screens.

#[macro_export]
/// This macro generates a screen state enum and a screen kind enum, along with the necessary implementations for the [`super::Screen`] trait and conversions between the two enums.
macro_rules! impl_screen {
    (
        $state_enum:ident, $kind_enum:ident {
            #[default]
            $default_variant:ident($default_state_type:ty),
            $( $variant:ident($state_type:ty) ),* $(,)?
        }
    ) => {
        /// State containing a screen's state.
        #[derive(Debug, Clone)]
        pub enum $state_enum {
            $default_variant($default_state_type),
            $( $variant($state_type), )*
        }

        /// The kind of screen.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub enum $kind_enum {
            #[default]
            $default_variant,
            $( $variant, )*
        }

        impl Default for $state_enum {
            fn default() -> Self {
                Self::from($kind_enum::default())
            }
        }

        impl crate::application::screen::Screen for $state_enum {
            fn content<'a>(&'a self, global_state: &GlobalState) -> iced::Element<'a, Message, Theme, Renderer> {
                match self {
                    Self::$default_variant(state) => state.content(global_state),
                    $(
                        Self::$variant(state) => state.content(global_state),
                    )*
                }
            }

            fn handle_text_input(&mut self, id: usize, value: String) {
                match self {
                    Self::$default_variant(state) => state.handle_text_input(id, value),
                    $(
                        Self::$variant(state) => state.handle_text_input(id, value),
                    )*
                }
            }

            fn window_bar_opposite<'a>(&'a self) -> Option<iced::Element<'a, Message, Theme, Renderer>> {
                match self {
                    Self::$default_variant(state) => state.window_bar_opposite(),
                    $(
                        Self::$variant(state) => state.window_bar_opposite(),
                    )*
                }
            }

            fn window_bar_center<'a>(&'a self) -> Option<iced::Element<'a, Message, Theme, Renderer>> {
                match self {
                    Self::$default_variant(state) => state.window_bar_center(),
                    $(
                        Self::$variant(state) => state.window_bar_center(),
                    )*
                }
            }

            fn window_bar_side_width(&self) -> Option<iced::Length> {
                match self {
                    Self::$default_variant(state) => state.window_bar_side_width(),
                    $(
                        Self::$variant(state) => state.window_bar_side_width(),
                    )*
                }
            }
        }

        impl From<$kind_enum> for $state_enum {
            fn from(screen_kind: $kind_enum) -> Self {
                match screen_kind {
                    $kind_enum::$default_variant => Self::$default_variant(<$default_state_type>::default()),
                    $(
                        $kind_enum::$variant => Self::$variant(<$state_type>::default()),
                    )*
                }
            }
        }

        impl From<&$state_enum> for $kind_enum {
            fn from(state: &$state_enum) -> Self {
                match state {
                    $state_enum::$default_variant(_) => $kind_enum::$default_variant,
                    $(
                        $state_enum::$variant(_) => $kind_enum::$variant,
                    )*
                }
            }
        }
    };
}
