use quicksilver::geom::{Vector, Rectangle};
use specs::prelude::*;
use paddlers_shared_lib::models::*;
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
    town::Town,
};

use super::workers::*; 

pub fn insert_hero(
    world: &mut World, 
    pos: impl Into<Vector>, 
    speed: f32,
    tile_area: Rectangle,  
    birth: Timestamp
) -> Entity 
{
    let pos = tile_area.pos;
    let size = tile_area.size;
    world.create_entity()
        .with(Position::new(pos, size, Z_UNITS))
        .with(Moving::new(birth, pos, (0,0), speed))
        .with(
            Renderable {
                kind: RenderVariant::ImgWithImgBackground(SpriteIndex::Hero, SpriteIndex::Grass),
            }
        )
        .with(Clickable)
        .with(Worker::default())
        .build()
}

pub fn insert_basic_worker(
    world: &mut World, 
    pos: impl Into<Vector>, 
    speed: f32,
    color: UnitColor,
    tile_area: Rectangle, 
    birth: Timestamp
) -> Entity 
{
    let pos = tile_area.pos;
    let size = tile_area.size;
    let sprite_index = match color {
        UnitColor::Yellow => SpriteIndex::Duck,
        UnitColor::White => SpriteIndex::Duck, // TODO
        UnitColor::Camo => SpriteIndex::Duck, // TODO
    };
    world.create_entity()
        .with(Position::new(pos, size, Z_UNITS))
        .with(Moving::new(birth, pos, (0,0), speed))
        .with(
            Renderable {
                kind: RenderVariant::ImgWithImgBackground(sprite_index, SpriteIndex::Grass),
            }
        )
        .with(Clickable)
        .with(Worker::default())
        .build()
}

use crate::net::graphql::WorkerResponse;
pub fn create_worker_entities(response: &WorkerResponse, world: &mut World, now: Timestamp) -> Vec<Entity> {
    response.iter()
        .map(|w|{
            let town = world.read_resource::<Town>();
            let area = town.tile_area((w.x as usize, w.y as usize));
            std::mem::drop(town);
            w.create_entity(world, now, area)
        })
        .collect()
}


use crate::net::graphql::village_units_query::{self, VillageUnitsQueryVillageUnits};
impl VillageUnitsQueryVillageUnits {
    fn create_entity(&self, world: &mut World, now: Timestamp, tile_area: Rectangle,) -> Entity {
        let pos = (self.x as f32, self.y as f32);
        let speed = self.speed as f32;
        match self.unit_type {
            village_units_query::UnitType::HERO => {
                insert_hero(world, pos, speed, tile_area, now)
            },
            village_units_query::UnitType::BASIC => { 
                let color = self.color.as_ref().unwrap().into();
                insert_basic_worker(world, pos, speed, color, tile_area, now)
            },
            _ => { panic!("Unexpected Unit Type") },
        }
    }
}

impl Into<UnitColor> for &village_units_query::UnitColor {
    fn into(self) -> UnitColor {
        match self {
            village_units_query::UnitColor::YELLOW => UnitColor::Yellow,
            village_units_query::UnitColor::WHITE => UnitColor::White,
            village_units_query::UnitColor::CAMO => UnitColor::Camo,
            _ => panic!("Unexpected color")
        }
    }
}