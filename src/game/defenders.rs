use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::game::{
    input::Clickable,
    movement::Position,
    render::{RenderType, Renderable},
    sprites::SpriteIndex,
};


pub fn insert_flowers(world: &mut World, pos: (i32, i32), ul: f32) -> Entity {
    let pos: Vector = pos.into();
    world.create_entity()
        .with(Position::new(pos * ul , (ul, ul), 100))
        .with(
            Renderable {
                kind: RenderType::StaticImage(SpriteIndex::Flowers, SpriteIndex::Grass),
            }
        )
        .with(Clickable)
        .build()
}

use crate::net::graphql::buildings_query;
impl buildings_query::ResponseData {
    pub fn create_entities(&self, world: &mut World, ul: f32) -> Vec<Entity> {
        self.buildings
            .iter()
            .enumerate()
            .map(|(i, u)|{u.create_entity(world, ul)})
            .collect()
    }
}

impl buildings_query::BuildingsQueryBuildings {
    fn create_entity(&self, world: &mut World, ul: f32) -> Entity {
        let coordinates = (self.x as i32,self.y as i32);
        insert_flowers(world, coordinates, ul)
    }
}