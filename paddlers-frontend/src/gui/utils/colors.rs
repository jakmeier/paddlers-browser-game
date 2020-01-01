#![allow(dead_code)]

// #[cfg(feature="dev_view")]
pub mod palette;
use quicksilver::prelude::*;

/* Color palette */
pub const LIGHT_GREEN: Color = Color {r: 0.600, g: 0.900, b: 0.250, a: 1.0};
pub const GREEN:       Color = Color {r: 0.059, g: 0.631, b: 0.000, a: 1.0};
pub const DARK_GREEN:  Color = Color {r: 0.047, g: 0.498, b: 0.000, a: 1.0};
pub const LIGHT_BLUE:  Color = Color {r: 0.250, g: 0.600, b: 0.900, a: 1.0};
pub const BLUE:        Color = Color {r: 0.000, g: 0.059, b: 0.631, a: 1.0};
pub const DARK_BLUE:   Color = Color {r: 0.000, g: 0.047, b: 0.498, a: 1.0}; 

/* Other colors */

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
pub const TRANSPARENT_BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.175,
};
pub const GREY: Color = Color {
    r: 0.75,
    g: 0.75,
    b: 0.75,
    a: 1.0,
};
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub const YELLOW: Color = Color {
    r: 1.0,
    g: 0.9,
    b: 0.25,
    a: 1.0
};
pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0
};
