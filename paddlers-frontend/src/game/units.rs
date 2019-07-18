use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::gui::{
    render::Renderable,
    z::Z_UNITS,
    sprites::SpriteIndex,
    utils::*,
};
use crate::game::{
    input::Clickable,
    movement::{Position, Velocity},
    fight::Health,
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Worker;

pub fn insert_hero(world: &mut World, pos: impl Into<Vector>, ul: f32) -> Entity {
    let pos = pos.into();
    world.create_entity()
        .with(Position::new(pos, (0.6*ul,0.4*ul), Z_UNITS))
        .with(Velocity::new(pos, (0,0)))
        .with(
            Renderable {
                kind: RenderVariant::ImgWithImgBackground(SpriteIndex::Hero, SpriteIndex::Grass),
            }
        )
        .with(Clickable)
        .with(Worker)
        .build()
}