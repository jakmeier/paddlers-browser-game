use quicksilver::geom::{Rectangle};
use specs::prelude::*;
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::game_mechanics::worker::*;
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
    components::*,
};

use super::workers::*; 

pub fn with_unit_base<B: Builder>(
    builder: B,
    speed: f32,
    tile_area: Rectangle,  
    birth: Timestamp,
    netid: i64,
) -> B
{
    let pos = tile_area.pos;
    let size = tile_area.size;
    builder
        .with(Position::new(pos, size, Z_UNITS))
        .with(Moving::new(birth, pos, (0,0), speed))
        .with(Clickable)
        .with(Worker::default())
        .with(NetObj{ id: netid })
}

pub fn with_hero<B: Builder>( builder: B ) -> B 
{
    builder.with(
        Renderable {
            kind: RenderVariant::ImgWithImgBackground(SpriteIndex::Hero, SpriteIndex::Grass),
        }
    )
}

pub fn with_basic_worker<B: Builder>( builder: B, color: UnitColor ) -> B 
{
    let sprite_index = match color {
        UnitColor::Yellow => SpriteIndex::Duck,
        UnitColor::White => SpriteIndex::WhiteDuck,
        UnitColor::Camo => SpriteIndex::CamoDuck,
    };
    builder.with(
        Renderable {
            kind: RenderVariant::ImgWithImgBackground(sprite_index, SpriteIndex::Grass),
        }
    )
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
        let speed = unit_speed_to_worker_tiles_per_second(self.speed as f32) / tile_area.width();
        let netid = self.id.parse().unwrap();
        let mut builder = with_unit_base(world.create_entity(), speed, tile_area, now, netid);
        match self.unit_type {
            village_units_query::UnitType::HERO => {
                builder = with_hero(builder);
            },
            village_units_query::UnitType::BASIC => { 
                let color = self.color.as_ref().unwrap().into();
                builder = with_basic_worker(builder, color);
            },
            _ => { panic!("Unexpected Unit Type") },
        }
        builder.build()
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