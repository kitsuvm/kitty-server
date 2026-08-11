//! Kitty Theme for Iced.

use iced_core::theme::{Base, Palette, palette::Extended};

pub mod font;
pub mod renderer;
pub mod theme;
pub mod widget;
pub mod window_event;

//// A trait for themes that extend the base palette.
pub trait BaseExtended: Base {
    /// Returns the base palette for the theme.
    fn palette(&self) -> Palette {
        <Self as Base>::palette(self).unwrap_or(Palette::LIGHT)
    }

    /// Returns the extended palette for the theme.
    fn palette_extended(&self) -> Extended {
        Extended::generate(<Self as BaseExtended>::palette(self))
    }
}

impl BaseExtended for iced_core::Theme {}
