use quicksilver::geom::{Rectangle};
use specs::prelude::*;
use crate::prelude::*;
use paddlers_shared_lib::game_mechanics::worker::*;
use crate::gui::{
    render::Renderable,
    z::Z_UNITS,
    sprites::*,
    utils::*,
    animation::*,
};
use crate::game::{
    input::Clickable,
    movement::{Position, Moving},
    town::Town,
    components::*,
    units::workers::*,
    abilities::AbilitySet,
};

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
        .with(NetObj::worker(netid))
        .with(AnimationState{ direction: Direction::Undirected })
}

pub fn with_hero<B: Builder>( builder: B ) -> B 
{
    builder.with(
        Renderable {
            kind: RenderVariant::ImgWithImgBackground(SpriteSet::Animated(AnimatedSprite::Roger), SingleSprite::Grass),
        }
    )
}

pub fn with_abilities<B: Builder>( builder: B, abilities: AbilitySet) -> B 
{
    builder.with(UiMenu { ui: abilities.construct_ui_box() } )
}

pub fn with_basic_worker<B: Builder>( builder: B, color: UnitColor ) -> B 
{
    let sprite_index = match color {
        UnitColor::Yellow => SingleSprite::Duck,
        UnitColor::White => SingleSprite::WhiteDuck,
        UnitColor::Camo => SingleSprite::CamoDuck,
    };
    builder.with(
        Renderable {
            kind: RenderVariant::ImgWithImgBackground(SpriteSet::Simple(sprite_index), SingleSprite::Grass),
        }
    )
}
pub fn with_worker<B: Builder, T: IntoIterator<Item = WorkerTask>>(builder: B, tasks: T, netid: i64) -> B {
    let worker_tasks = tasks.into_iter()
        .map(|t| t.into())
        .collect::<std::collections::VecDeque<_>>();
    builder.with(Worker {
        tasks: worker_tasks,
        netid: netid,
    })
}

use crate::net::graphql::WorkerResponse;
pub fn create_worker_entities(response: &WorkerResponse, world: &mut World, now: Timestamp) -> Vec<PadlResult<Entity>> {
    response.iter()
        .map(|w|{
            let town = world.read_resource::<Town>();
            let area = town.tile_area((w.x as usize, w.y as usize));
            std::mem::drop(town);
            w.create_entity(world, now, area)
        })
        .collect()
}


use crate::net::graphql::village_units_query::{self, VillageUnitsQueryVillageWorkers};
impl VillageUnitsQueryVillageWorkers {
    fn create_entity(&self, world: &mut World, now: Timestamp, tile_area: Rectangle,) -> PadlResult<Entity> {
        let speed = unit_speed_to_worker_tiles_per_second(self.speed as f32) * tile_area.width();
        let netid = self.id.parse().unwrap();
        
        let net = world.read_storage::<NetObj>();
        let ent = world.entities();
        let tasks = self.tasks.iter()
            .map(|t| t.create(&net, &ent))
            .filter(|t| t.is_ok()) // Ignoring task that cannot be assigned to a unit 
            .map(|t| t.unwrap());
        
        let lazy = world.read_resource::<LazyUpdate>();
        let mut builder = with_unit_base(lazy.create_entity(&world.entities()), speed, tile_area, now, netid);
        builder = with_worker(builder, tasks, netid);
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
        let abilities = AbilitySet::from_gql(&self.abilities)?;
        builder = with_abilities(builder, abilities);
        Ok(builder.build())
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