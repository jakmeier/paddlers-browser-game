use crate::game::town::Town;
use crate::game::units::attackers::insert_duck;
use crate::net::graphql::query_types::HoboEffect;
use crate::prelude::*;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;
use specs::world::EntitiesRes;

pub(crate) fn insert_buildings(town: &mut Town, entities: &EntitiesRes, lazy: &LazyUpdate) {
    town.insert_new_building(entities, lazy, (0, 0), BuildingType::BlueFlowers);
    town.insert_new_building(entities, lazy, (0, 1), BuildingType::BlueFlowers);
    town.insert_new_building(entities, lazy, (0, 2), BuildingType::BlueFlowers);
    town.insert_new_building(entities, lazy, (1, 0), BuildingType::BlueFlowers);
    town.insert_new_building(entities, lazy, (1, 1), BuildingType::BlueFlowers);
    town.insert_new_building(entities, lazy, (1, 2), BuildingType::BlueFlowers);

    town.insert_new_building(entities, lazy, (2, 1), BuildingType::PresentA);
    town.insert_new_building(entities, lazy, (4, 1), BuildingType::Temple);
    town.insert_new_building(entities, lazy, (6, 1), BuildingType::PresentB);

    town.insert_new_building(entities, lazy, (7, 0), BuildingType::RedFlowers);
    town.insert_new_building(entities, lazy, (7, 1), BuildingType::RedFlowers);
    town.insert_new_building(entities, lazy, (7, 2), BuildingType::RedFlowers);
    town.insert_new_building(entities, lazy, (8, 0), BuildingType::RedFlowers);
    town.insert_new_building(entities, lazy, (8, 1), BuildingType::RedFlowers);
    town.insert_new_building(entities, lazy, (8, 2), BuildingType::RedFlowers);

    town.insert_new_building(entities, lazy, (3, 5), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (3, 6), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (4, 5), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (4, 6), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (5, 5), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (5, 6), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (6, 5), BuildingType::Tree);
    town.insert_new_building(entities, lazy, (6, 6), BuildingType::Tree);

    town.insert_new_building(entities, lazy, (7, 5), BuildingType::SawMill);
    town.insert_new_building(entities, lazy, (7, 6), BuildingType::BundlingStation);
}
pub(crate) fn insert_hobos(world: &mut World) -> PadlResult<()> {
    let now = utc_now();
    let ul = world.fetch::<ScreenResolution>().unit_length();

    let w = TOWN_X as f32 * ul;
    let x = w - ul * 0.6;
    let y = TOWN_LANE_Y as f32 * ul;
    let pos0 = (x, y);
    let pos1 = (x - ul / 2.0, y + ul / 2.0);
    let color = UnitColor::Camo;
    let birth_time = now;
    let speed = (-10.0, 0.0);
    let hp = 3;
    let netid = 1;
    let effects = Vec::<HoboEffect>::new();

    insert_duck(
        world, pos0, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color, birth_time, speed, hp, ul, netid, &effects,
    )?;

    let dt = 5_000_000;

    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color, birth_time, speed, hp, ul, netid, &effects,
    )?;

    let birth_time = birth_time - 10 * dt;
    let color0 = UnitColor::Yellow;
    let color1 = UnitColor::White;
    insert_duck(
        world, pos0, color0, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color1, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color1, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color0, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color0, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color1, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color1, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color0, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - 2 * dt;
    insert_duck(
        world, pos0, color0, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color1, birth_time, speed, hp, ul, netid, &effects,
    )?;
    let birth_time = birth_time - dt;
    insert_duck(
        world, pos0, color1, birth_time, speed, hp, ul, netid, &effects,
    )?;
    insert_duck(
        world, pos1, color0, birth_time, speed, hp, ul, netid, &effects,
    )?;

    Ok(())
}
