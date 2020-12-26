use crate::game::movement::{Moving, Position};
use crate::game::units::attackers::Visitor;
use crate::gui::ui_state::Now;
use crate::gui::{render::Renderable, sprites::*, utils::*};
use crate::prelude::*;
use crate::{game::fight::Health, resolution::TOWN_TILE_S};
use paddle::Vector;
use paddlers_shared_lib::game_mechanics::town::*;
use specs::prelude::*;

pub fn change_duck_sprite_to_happy(r: &mut Renderable) {
    match r.kind {
        RenderVariant::ImgWithImgBackground(SpriteSet::Simple(ref mut img), _bkg) => match img {
            SingleSprite::Duck => {
                *img = SingleSprite::DuckHappy;
            }
            SingleSprite::CamoDuck => {
                *img = SingleSprite::CamoDuckHappy;
            }
            SingleSprite::WhiteDuck => {
                *img = SingleSprite::WhiteDuckHappy;
            }
            _ => {}
        },
        _ => {}
    }
}

/// Set visitor moving again (Without server communication)
pub fn release_and_move_visitor(visitor: &Visitor, now: Now) -> Moving {
    let ul = TOWN_TILE_S as f32;
    let now = now.0;
    let speed = visitor.speed;
    let momentum = Vector::new(-speed, 0.0);
    let x = TOWN_RESTING_X as f32 * ul;
    let y =
        TOWN_LANE_Y as f32 * ul + super::attacker_position_rank_offset(visitor.rank_offset, ul).y;
    let pos = Vector::new(x, y);
    Moving::new(now, pos, momentum, speed)
}

impl Game {
    /// Ensure there are not too many visitors resting in the town. (Without consulting the server)
    pub fn check_resting_queue(&mut self) -> PadlResult<()> {
        let town_world = self.town_world();
        let visitors = town_world.read_component::<Visitor>();
        let hps = town_world.read_component::<Health>();
        let positions = town_world.read_component::<Position>();
        let entities = town_world.entities();
        let now = self.world.fetch::<Now>().0;
        let ul = TOWN_TILE_S as f32;

        let mut resting_visitors = vec![];
        for (visitor, hp, e, pos) in (&visitors, &hps, &entities, &positions).join() {
            if !visitor.hurried
                && visitor.arrival <= now
                && hp.hp > 0
                && pos.area.pos.x >= TOWN_RESTING_X as f32 * ul
            {
                resting_visitors.push((visitor, e));
            }
        }

        let to_release = resting_visitors.len().saturating_sub(MAX_VISITOR_QUEUE);
        if to_release > 0 {
            let world = self.town_world();
            let now = *world.fetch::<Now>();
            let mut mov = world.write_component::<Moving>();
            resting_visitors.sort_by(|a, b| a.0.arrival.partial_cmp(&b.0.arrival).unwrap());
            for (visitor, e) in &resting_visitors[0..to_release] {
                mov.insert(*e, release_and_move_visitor(visitor, now))?;
            }
        }
        Ok(())
    }
}
