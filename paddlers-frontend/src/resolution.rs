use paddlers_shared_lib::game_mechanics::town::{TOWN_X, TOWN_Y};

// These are all game coordinates, which are an abstraction over different resolution.

// The full area used
pub const SCREEN_H: u32 = 1080;
pub const SCREEN_W: u32 = 1920;

// The area on the left showing the actual game (as opposed to UI)
pub const MAIN_AREA_H: u32 = SCREEN_H;
pub const TOWN_TILE_S: u32 = MAIN_AREA_H / TOWN_Y as u32;
pub const MAIN_AREA_W: u32 = TOWN_TILE_S * TOWN_X as u32;

// Right-hand side area with all UI elements
pub const OUTER_MENU_AREA_X: u32 = MAIN_AREA_W;
pub const OUTER_MENU_AREA_Y: u32 = 0;
pub const OUTER_MENU_AREA_W: u32 = SCREEN_W - MAIN_AREA_W;
pub const OUTER_MENU_AREA_H: u32 = SCREEN_H;

// TODO: Make it selectable by user (maybe even move some code to paddle)
// #[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ScreenResolution {
//     Low,
//     Mid,
//     High,
// }

// impl ScreenResolution {
//     /// Total window dimensions (ratio is always 16:9)
//     pub fn pixels(&self) -> (f32, f32) {
//         match self {
//             ScreenResolution::Low => (640.0, 360.0),
//             ScreenResolution::Mid => (1280.0, 720.0),
//             ScreenResolution::High => (1920.0, 1080.0),
//         }
//     }
// }

// impl Default for ScreenResolution {
//     fn default() -> Self {
//         ScreenResolution::Mid
//     }
// }
