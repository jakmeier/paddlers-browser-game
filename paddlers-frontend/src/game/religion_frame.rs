use crate::gui::{
    decoration::draw_leaf_border,
    menu::{LEAVES_BORDER_H, LEAVES_BORDER_W},
    sprites::WithSprite,
    utils::*,
};
use crate::{gui::shapes::PadlShapeIndex, prelude::UiView};
use paddle::quicksilver_compat::{Circle, Color, Shape};
use paddle::*;
use paddlers_shared_lib::civilization::*;

use super::{
    game_event_manager::{game_event, GameEvent},
    toplevel::Signal,
    Game,
};

pub(crate) struct ReligionFrame {
    perks: CivilizationPerks,
}

const MARGIN: f32 = 75.0;
const LOCKED_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.75,
};

impl ReligionFrame {
    pub fn new() -> Self {
        Self {
            perks: CivilizationPerks::new(0),
        }
    }
    pub fn signal(&mut self, state: &mut Game, msg: &Signal) {
        match msg {
            Signal::PlayerInfoUpdated => self.perks = state.player().civilization_perks(),
            _ => {}
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
                x: MARGIN,
                y: main_area.size.y + MARGIN,
            }),
            size: Vector {
                x: Self::WIDTH as f32 / 5.0,
                y: Self::HEIGHT as f32 * (1.0 - 0.61803398875) - 3.0 * MARGIN,
            },
        }
    }
    const fn perk_position(perk: CivilizationPerk) -> Rectangle {
        let s = 150.0;
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
        canvas.fill(Color::GREEN);

        // background
        let mut leaf_area = Self::area();
        let dx = LEAVES_BORDER_W / 2.0;
        leaf_area.pos.x += dx;
        leaf_area.size.x -= dx;
        draw_leaf_border(
            canvas,
            &mut state.sprites,
            &leaf_area,
            LEAVES_BORDER_W,
            LEAVES_BORDER_H,
        );

        // perks
        for perk in &[
            CivilizationPerk::NestBuilding,
            CivilizationPerk::Invitation,
            CivilizationPerk::Conversion,
        ] {
            let area = Self::perk_position(*perk);
            canvas.draw(&area, &state.sprites.index(perk.sprite().default()));
            if !self.perks.has(*perk) {
                canvas.draw(
                    &Circle::new(area.center(), area.width() / 2.0),
                    LOCKED_COLOR,
                );
            }
        }

        // back button
        draw_shape(
            &mut state.sprites,
            canvas,
            &Self::button_area(),
            PadlShapeIndex::LeftArrowV2,
            FitStrategy::Center,
            1,
        );
    }
    fn pointer(&mut self, _state: &mut Self::State, event: PointerEvent) {
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
