use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::game::{
    input::Clickable,
    movement::Position,
    render::{RenderType, Renderable},
    sprites::SpriteIndex,
    fight::Range,
    render::Z_UNITS
};


pub fn insert_flowers(world: &mut World, pos: (i32, i32), range: Option<f32>, ul: f32) -> Entity {
    let pos: Vector = pos.into();
    let builder = 
        world.create_entity()
        .with(Position::new(pos * ul , (ul, ul), Z_UNITS))
        .with(
            Renderable {
                kind: RenderType::StaticImage(SpriteIndex::Flowers, SpriteIndex::Grass),
            }
        )
        .with(Clickable);

    let builder = 
        if let Some(r) = range {
            builder.with(Range::new(r))
        }
        else {
            builder
        };

    builder.build()
}

use crate::net::graphql::buildings_query;
impl buildings_query::ResponseData {
    pub fn create_entities(&self, world: &mut World, ul: f32) -> Vec<Entity> {
        self.buildings
            .iter()
            .map(|u|{u.create_entity(world, ul)})
            .collect()
    }
}

impl buildings_query::BuildingsQueryBuildings {
    fn create_entity(&self, world: &mut World, ul: f32) -> Entity {
        let coordinates = (self.x as i32,self.y as i32);
        let maybe_range = self.building_range.map(|f| f as f32);
        insert_flowers(world, coordinates, maybe_range, ul)
    }
}