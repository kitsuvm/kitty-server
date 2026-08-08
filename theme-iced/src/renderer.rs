//! Custom renderer traits.

use iced_core::{Font, text::Renderer};

/// A custom renderer trait that natively bundles the font requirement
pub trait TextRenderer: Renderer<Font: From<Font>> {}

// Blanket implementation so any renderer meeting the criteria automatically implements it
impl<T> TextRenderer for T where T: iced_core::text::Renderer<Font: From<Font>> {}
