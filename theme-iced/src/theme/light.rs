//! The light palette for the Kitty Theme.

use iced_core::{
    color,
    theme::{
        Palette,
        palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success, Warning},
    },
};

/// The palette for the light theme.
pub const LIGHT_PALETTE: Palette = Palette {
    background: color!(0xffffff),
    text: color!(0x000000),
    primary: color!(0x8500ff),
    success: color!(0x00ff00),
    warning: color!(0xffff00),
    danger: color!(0xff0000),
};

/// The extended palette for the light theme.
pub const LIGHT_EXTENDED_PALETTE: Extended = Extended {
    is_dark: false,
    background: Background {
        base: Pair {
            color: color!(0xffffff),
            text: color!(0x000000),
        },
        weakest: Pair {
            color: color!(0xeaeaea),
            text: color!(0x7e7e7f),
        },
        weaker: Pair {
            color: color!(0xeaeaea),
            text: color!(0x7e7e7f),
        },
        weak: Pair {
            color: color!(0xeaeaea),
            text: color!(0x7e7e7f),
        },
        neutral: Pair {
            color: color!(0xcecece),
            text: color!(0x7e7e7f),
        },
        strong: Pair {
            color: color!(0xcecece),
            text: color!(0x7e7e7f),
        },
        stronger: Pair {
            color: color!(0xcecece),
            text: color!(0x7e7e7f),
        },
        strongest: Pair {
            color: color!(0xcecece),
            text: color!(0x7e7e7f),
        },
    },
    danger: Danger {
        base: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
    },
    primary: Primary {
        base: Pair {
            color: color!(0x8500ff),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x6a00ce),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x5301a0),
            text: color!(0xffffff),
        },
    },
    secondary: Secondary {
        base: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
    },
    success: Success {
        base: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
    },
    warning: Warning {
        base: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
    },
};
