//! This module contains the font definitions for the application.

use iced::{
    Font,
    font::{Family, Stretch, Style, Weight},
};

/// Lato Regular font
pub const LATO_REGULAR_FONT: Font = Font {
    weight: Weight::Normal,
    family: Family::Name("Lato"),
    stretch: Stretch::Normal,
    style: Style::Normal,
};
/// Material Symbols Rounded Filled font
pub static MATERIAL_SYMBOLS_FILLED_ROUNDED_FONT: Font = Font {
    weight: Weight::Normal,
    family: Family::Name("Material Symbols Rounded Filled"),
    stretch: Stretch::Normal,
    style: Style::Normal,
};

/// Lato Regular font bytes
pub static LATO_REGULAR_BYTES: &[u8] = include_bytes!("./assets/Lato-Regular.ttf");

/// Material Symbols Rounded Filled font bytes
pub static MATERIAL_SYMBOLS_FILLED_ROUNDED_BYTES: &[u8] =
    include_bytes!("./assets/MaterialSymbolsRounded_Filled-Regular.ttf");
