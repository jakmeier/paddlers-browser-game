use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::game::{
    input::Clickable,
    movement::{Position, Velocity},
    render::{RenderType, Renderable},
    sprites::SpriteIndex,
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Attacker;

pub fn insert_duck(world: &mut World, pos: impl Into<Vector>, speed: impl Into<Vector>, ul: f32) -> Entity {
    let pos = pos.into();
    world.create_entity()
        .with(Position::new(pos, (0.6*ul,0.4*ul), 100))
        .with(Velocity::new(pos, speed))
        .with(
            Renderable {
                kind: RenderType::StaticImage(SpriteIndex::Duck, SpriteIndex::Water),
            }
        )
        .with(Clickable)
        .with(Attacker)
        .build()
}

pub fn delete_all_attackers(world: &mut World) {
    let attackers: Vec<Entity> = 
        (&world.entities(), &world.read_storage::<Attacker>())
        .join()
        .map( |tuple| tuple.0 )
        .collect();
    world.delete_entities(&attackers).expect("Deleting old attacker generation failed");
}


use crate::net::graphql::attacks_query::{AttacksQueryAttacksUnits,AttacksQueryAttacks};
impl AttacksQueryAttacks {
    pub fn create_entities(&self, world: &mut World, ul: f32) -> Vec<Entity> {
        let ms_timestamp_now = stdweb::web::Date::now();
        let now = chrono::NaiveDateTime::from_timestamp((ms_timestamp_now / 1000.0) as i64, (ms_timestamp_now % 1000.0) as u32 * 1000_000);
        let time_alive = now - self.departure();
        self.units
            .iter()
            .enumerate()
            .map(|(i, u)|{u.create_entity(world, &time_alive, i, ul)})
            .collect()
    }
}
impl AttacksQueryAttacksUnits {
    fn create_entity(&self, world: &mut World, time_alive: &chrono::Duration, pos_rank: usize, ul: f32) -> Entity {
        let v = -self.speed as f32 / (super::CYCLE_SECS * 1000) as f32 * ul;
        let start_x = 1000.0 - 30.0;
        let y = 300.0;
        let x = start_x + time_alive.num_milliseconds() as f32 * v;
        let pos = Vector::new(x,y) + attacker_position_rank_offset(pos_rank);
        insert_duck(world, pos, (v as f32,0.0), ul)
    }
}

fn attacker_position_rank_offset(pr: usize) -> Vector {
    let y = if pr % 2 == 1 { -20 } else { 0 };
    let x = 15 * pr as i32;
    (x,y).into()
}