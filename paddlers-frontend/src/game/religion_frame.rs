use crate::gui::utils::*;
use crate::{
    gui::shapes::{PadlShape, PadlShapeIndex},
    prelude::UiView,
};
use paddle::quicksilver_compat::{Color, Shape};
use paddle::*;
use paddlers_shared_lib::civilization::*;

use super::{
    game_event_manager::{game_event, GameEvent},
    Game,
};

pub(crate) struct ReligionFrame {
    perks: CivilizationPerks,
}

const MARGIN: f32 = 50.0;

impl ReligionFrame {
    pub fn new() -> Self {
        Self {
            perks: CivilizationPerks::new(0),
        }
    }
    const fn main_area() -> Rectangle {
        Rectangle {
            pos: Vector {
                x: MARGIN,
                y: MARGIN,
            },
            size: Vector {
                x: Self::WIDTH as f32 - 2.0 * MARGIN,
                y: Self::HEIGHT as f32 * 0.61803398875,
            },
        }
    }
    const fn button_area() -> Rectangle {
        let main_area = Self::main_area();
        Rectangle {
            pos: main_area.pos.const_translate(Vector {
                x: Self::WIDTH as f32 / 3.0,
                y: main_area.size.y + MARGIN,
            }),
            size: Vector {
                x: Self::WIDTH as f32 / 3.0,
                y: Self::HEIGHT as f32 * (1.0 - 0.61803398875) - 3.0 * MARGIN,
            },
        }
    }
    const fn perk_position(perk: CivilizationPerk) -> Rectangle {
        let s = 100.0;
        let main_area = Self::main_area();

        let (dx, dy) = match perk {
            CivilizationPerk::NestBuilding => (0.0, -200.0),
            CivilizationPerk::TripleNestBuilding => (0.0, 0.0), // Not implemented
            CivilizationPerk::Invitation => (-178.89, 89.44),
            CivilizationPerk::Conversion => (178.89, 89.44),
        };
        let center_x = main_area.pos.x + (main_area.size.x - s) / 2.0;
        let center_y = main_area.pos.y + (main_area.size.y - s) / 2.0;
        Rectangle {
            pos: Vector {
                x: center_x + dx,
                y: center_y + dy,
            },
            size: Vector { x: s, y: s },
        }
    }
}

impl Frame for ReligionFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::SCREEN_W;
    const HEIGHT: u32 = crate::resolution::SCREEN_H;

    fn draw(&mut self, state: &mut Self::State, canvas: &mut paddle::DisplayArea, _timestamp: f64) {
        canvas.fill(Color::WHITE);

        // perks
        for perk in &[
            CivilizationPerk::NestBuilding,
            CivilizationPerk::Invitation,
            CivilizationPerk::Conversion,
        ] {
            canvas.draw(&Self::perk_position(*perk), Color::BLACK);
        }

        // back button
        draw_shape(
            &mut state.sprites,
            canvas,
            &Self::button_area(),
            PadlShapeIndex::LeftArrow,
            FitStrategy::Center,
            1,
        );
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        match event {
            PointerEvent(PointerEventType::PrimaryClick, pos) => {
                if Self::button_area().contains(pos) {
                    game_event(GameEvent::SwitchToView(UiView::Town));
                }
            }
            _ => {}
        }
    }
}
