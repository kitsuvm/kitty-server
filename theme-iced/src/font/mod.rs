//! This module contains the fonts for the Kitty Theme.

use iced_core::{
    Font,
    font::{Family, Stretch, Style, Weight},
};
use iced_runtime::{Task, font};

/// Lato Regular font bytes
pub static LATO_REGULAR_BYTES: &[u8] = include_bytes!("./Lato-Regular.ttf");

/// Lato Bold font bytes
pub static LATO_BOLD_BYTES: &[u8] = include_bytes!("./Lato-Bold.ttf");

/// Constructs a Lato font with the given weight.
pub const fn lato_font(weight: Weight) -> Font {
    Font {
        weight,
        family: Family::Name("Lato"),
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

/// Lato Regular font
pub const LATO_REGULAR_FONT: Font = lato_font(Weight::Normal);

/// Lato Bold font
pub const LATO_BOLD_FONT: Font = lato_font(Weight::Bold);

/// Material Symbols Rounded Filled font bytes
pub static MATERIAL_SYMBOLS_ROUNDED_FILLED_REGULAR_BYTES: &[u8] =
    include_bytes!("./MaterialSymbolsRounded_Filled-Regular.ttf");

/// Material Symbols Rounded Filled font
pub const MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT: Font = Font {
    weight: Weight::Normal,
    family: Family::Name("Material Symbols Rounded Filled"),
    stretch: Stretch::Normal,
    style: Style::Normal,
};

/// Loads the fonts for the Kitty Theme.
pub fn load_all<Message: Send + 'static>() -> Task<Message> {
    Task::batch([
        load_lato_regular(),
        load_lato_bold(),
        load_material_symbols_rounded_filled(),
    ])
}

/// Load the Lato Regular font for the Kitty Theme.
pub fn load_lato_regular<Message: Send + 'static>() -> Task<Message> {
    font::load(LATO_REGULAR_BYTES).discard()
}

/// Load the Lato Bold font for the Kitty Theme.
pub fn load_lato_bold<Message: Send + 'static>() -> Task<Message> {
    font::load(LATO_BOLD_BYTES).discard()
}

/// Load the Material Symbols Rounded Filled font for the Kitty Theme.
pub fn load_material_symbols_rounded_filled<Message: Send + 'static>() -> Task<Message> {
    font::load(MATERIAL_SYMBOLS_ROUNDED_FILLED_REGULAR_BYTES).discard()
}
