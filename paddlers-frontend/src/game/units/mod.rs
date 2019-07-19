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
};

pub mod workers;
use workers::*; 
pub mod worker_system;



pub fn insert_hero(world: &mut World, pos: impl Into<Vector>, ul: f32, birth: Timestamp) -> Entity {
    let pos = pos.into();
    world.create_entity()
        .with(Position::new(pos, (0.6*ul,0.4*ul), Z_UNITS))
        .with(Moving::new(birth, pos, (0,0), 0.01))
        .with(
            Renderable {
                kind: RenderVariant::ImgWithImgBackground(SpriteIndex::Hero, SpriteIndex::Grass),
            }
        )
        .with(Clickable)
        .with(Worker::default())
        .build()
}
