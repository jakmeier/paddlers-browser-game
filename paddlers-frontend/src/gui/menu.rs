pub mod entity_details;
mod map_menu;
mod menu_background;
mod town_menu;

use crate::{
    gui::sprites::Sprites,
    resolution::{MENU_AREA_H, MENU_AREA_W},
};
pub(crate) use map_menu::MapMenuFrame;
pub(crate) use menu_background::MenuBackgroundFrame;
use paddle::*;
pub(crate) use town_menu::TownMenuFrame;

use crate::game::Game;
use crate::gui::{gui_components::*, ui_state::Now, z::*};
use paddle::quicksilver_compat::{Col, Rectangle, Transform, Vector};
use paddle::DisplayArea;

pub const MENU_PADDING: f32 = 5.0;
pub const AFTER_DUCK_STEPS_PADDING: f32 = 10.0;
pub const NAVIGATION_BUTTONS_H: f32 = 150.0;
pub const RESOURCES_H: f32 = 80.0;
pub const LEAVES_BORDER_H: f32 = 100.0;
pub const LEAVES_BORDER_W: f32 = 80.0;
pub const DUCK_STEPS_H: f32 = 40.0;

/// Returns the areas for the menu image and the table below
pub fn menu_selected_entity_spacing(area: &Rectangle) -> (Rectangle, Rectangle) {
    let mut img_bg_area = area.clone();
    img_bg_area.size.y = img_bg_area.height() / 3.0;
    let img_bg_area = img_bg_area
        .fit_square(FitStrategy::Center)
        .shrink_to_center(0.8);
    let text_y = img_bg_area.y() + img_bg_area.height();
    let text_area = Rectangle::new(
        (area.x(), text_y),
        (area.width(), area.y() + area.height() - text_y),
    )
    .padded(20.0);

    // self.draw_entity_details_img(window, e, &img_bg_area)?;
    // self.draw_entity_details_table(window, e, &text_area, text_provider, hover_res_comp)?;
    (img_bg_area, text_area)
}

/// Complete menu box area, without any padding
pub const fn menu_box_area() -> Rectangle {
    Rectangle {
        pos: Vector { x: 0.0, y: 0.0 },
        size: Vector {
            x: MENU_AREA_W as f32,
            y: MENU_AREA_H as f32,
        },
    }
}

/// Navigation buttons bar at the top (relative to frame, not the full screen)
pub const fn nav_area() -> Rectangle {
    Rectangle {
        pos: Vector {
            x: LEAVES_BORDER_W * 0.25 + MENU_PADDING,
            y: LEAVES_BORDER_H / 2.0,
        },
        size: Vector {
            x: MENU_AREA_W as f32 - (LEAVES_BORDER_W + 2.0 * MENU_PADDING),
            y: NAVIGATION_BUTTONS_H,
        },
    }
}
/// Division line between navigation area and inner menu
pub const fn duck_step_area() -> Rectangle {
    let nav_area = nav_area();
    Rectangle {
        pos: Vector {
            x: 0.0,
            y: nav_area.pos.y + nav_area.size.y,
        },
        size: Vector {
            x: MENU_AREA_W as f32,
            y: DUCK_STEPS_H,
        },
    }
}
/// Menu are with applied padding and without area for navigatino bar
pub const fn inner_menu_area() -> Rectangle {
    let duck_steps = duck_step_area();
    let y = duck_steps.pos.y + duck_steps.size.y + AFTER_DUCK_STEPS_PADDING;
    Rectangle {
        pos: Vector {
            x: LEAVES_BORDER_W * 0.25 + MENU_PADDING,
            y: y,
        },
        size: Vector {
            x: MENU_AREA_W as f32 - (LEAVES_BORDER_W + 2.0 * MENU_PADDING),
            y: MENU_AREA_H as f32 - y - LEAVES_BORDER_H / 2.0,
        },
    }
}
