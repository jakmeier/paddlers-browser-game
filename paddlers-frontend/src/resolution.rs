use paddlers_shared_lib::game_mechanics::town::{TOWN_X, TOWN_Y};
use strum_macros::EnumIter;

// These are game coordinates, which are an abstraction over different resolution.
pub const SCREEN_H: u32 = 1080;
pub const SCREEN_W: u32 = 1920;

pub const MAIN_AREA_H: u32 = SCREEN_H;
pub const TOWN_TILE_S: u32 = MAIN_AREA_H / TOWN_Y as u32;
pub const MAIN_AREA_W: u32 = TOWN_TILE_S * TOWN_X as u32;

pub const MENU_AREA_X: u32 = MAIN_AREA_W;
pub const MENU_AREA_Y: u32 = 0;
pub const MENU_AREA_W: u32 = SCREEN_W - MAIN_AREA_W;
pub const MENU_AREA_H: u32 = SCREEN_H;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenResolution {
    Low,
    Mid,
    High,
}

impl ScreenResolution {
    /// Total window dimensions (ratio is always 16:9)
    pub fn pixels(&self) -> (f32, f32) {
        match self {
            ScreenResolution::Low => (640.0, 360.0),
            ScreenResolution::Mid => (1280.0, 720.0),
            ScreenResolution::High => (1920.0, 1080.0),
        }
    }
    /// The side-length of a square in the town view
    pub fn unit_length(&self) -> f32 {
        self.pixels().1 / TOWN_Y as f32
    }
    /// The dimensions of the main area
    pub fn main_area(&self) -> (f32, f32) {
        (self.unit_length() * TOWN_X as f32, self.pixels().1)
    }
    /// The dimensions of the menu area
    pub fn menu_area(&self) -> (f32, f32) {
        (self.menu_width(), self.pixels().1)
    }
    /// Menu on the right side must have the correct width to
    /// fill screen to 16:9 ratio in combination with the town view
    pub fn menu_width(&self) -> f32 {
        self.unit_length() * ((16.0 * TOWN_Y as f32) - (9.0 * TOWN_X as f32)) / 9.0
    }
}

impl Default for ScreenResolution {
    fn default() -> Self {
        ScreenResolution::Mid
    }
}
