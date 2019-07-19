use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::Timestamp;
use crate::gui::{
    render::Renderable,
    z::Z_UNITS,
    sprites::SpriteIndex,
    utils::*,
};
use crate::game::{
    input::Clickable,
    movement::{Position, Moving},
    fight::Health,
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Attacker;

pub fn insert_duck(world: &mut World, pos: impl Into<Vector>, birth_time: Timestamp, speed: impl Into<Vector>, hp: i64, ul: f32) -> Entity {
    let pos = pos.into();
    let speed = speed.into();
    world.create_entity()
        .with(Position::new(pos, (0.6*ul,0.4*ul), Z_UNITS))
        .with(Moving::new(birth_time, pos, speed, speed.len()))
        .with(
            Renderable {
                kind: RenderVariant::ImgWithImgBackground(SpriteIndex::Duck, SpriteIndex::Water),
            }
        )
        .with(Clickable)
        .with(Attacker)
        .with(Health::new_full_health(hp))
        .build()
}

use crate::net::graphql::attacks_query::{AttacksQueryAttacksUnits,AttacksQueryAttacks};
impl AttacksQueryAttacks {
    pub fn create_entities(&self, world: &mut World, ul: f32) -> Vec<Entity> {
        let birth_time = self.arrival * 1000.0;
        self.units
            .iter()
            .enumerate()
            .map(|(i, u)|{u.create_entity(world, birth_time, i, ul)})
            .collect()
    }
}
impl AttacksQueryAttacksUnits {
    // TODO: For entities already well into the map, compute the attacks so far.
    fn create_entity(&self, world: &mut World, birth: Timestamp, pos_rank: usize, ul: f32) -> Entity {
        let v = -self.speed as f32 / (super::super::CYCLE_SECS * 1000) as f32 * ul;
        let x = 1000.0 - 30.0;
        let y = 300.0;
        let pos = Vector::new(x,y) + attacker_position_rank_offset(pos_rank);
        let hp = self.hp;
        insert_duck(world, pos, birth, (v as f32,0.0), hp, ul)
    }
}

fn attacker_position_rank_offset(pr: usize) -> Vector {
    let y = if pr % 2 == 1 { -20 } else { 0 };
    let x = 15 * pr as i32;
    (x,y).into()
}