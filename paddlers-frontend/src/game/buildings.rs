use crate::game::town::TownContext;
use crate::game::{
    components::*,
    fight::{Aura, Range},
    forestry::ForestComponent,
    input::Clickable,
    movement::Position,
    town::{TileIndex, Town},
};
use crate::gui::{render::Renderable, sprites::*, utils::*, z::Z_BUILDINGS};
use crate::prelude::*;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::{game_mechanics::attributes::Attributes, graphql_types::*};
use specs::prelude::*;
use specs::world::EntitiesRes;

#[derive(Debug, Component)]
#[storage(HashMapStorage)]
pub struct Building {
    pub built: Timestamp,
    pub bt: BuildingType,
}

impl Town {
    pub fn insert_new_building(
        &mut self,
        entities: &EntitiesRes,
        lazy: &LazyUpdate,
        pos: TileIndex,
        bt: BuildingType,
    ) -> Entity {
        self.insert_building(
            entities,
            lazy,
            pos,
            bt,
            bt.attack_power(),
            bt.attacks_per_cycle(),
            bt.range(),
            utc_now(),
        )
    }

    fn insert_building(
        &mut self,
        entities: &EntitiesRes,
        lazy: &LazyUpdate,
        tile_index: TileIndex,
        bt: BuildingType,
        ap: Option<i64>,
        attacks_per_cycle: Option<i64>,
        range: Option<f32>,
        created: crate::Timestamp,
    ) -> Entity {
        let area = self.resolution.tile_area(tile_index);
        let mut builder = lazy
            .create_entity(entities)
            .with(Position::new(area.pos, area.size, Z_BUILDINGS))
            .with(Renderable::new_transformed(
                RenderVariant::ImgWithImgBackground(bt.sprite(), SingleSprite::Grass),
                building_ingame_scaling(bt),
            ))
            .with(Building { built: created, bt })
            .with(Clickable);

        if let Some(r) = range {
            builder = builder.with(Range::new(r));
        }

        // No (None) attacks per cycle && Some ap => Aura effect
        if attacks_per_cycle.is_none() && ap.is_some() {
            if let Some(r) = range {
                builder = builder.with(Aura::new(r, ap.unwrap(), tile_index, self));
                if r > self.distance_to_lane(tile_index) {
                    self.total_ambience += ap.unwrap();
                }
            }
        }

        match bt {
            BuildingType::Temple => {
                builder = builder.with(UiMenu::new_shop_menu());
            }
            BuildingType::BundlingStation => {
                builder = builder
                    .with(EntityContainer::new(bt.capacity(), TaskType::GatherSticks))
                    .with(UiMenu::new_entity_container());
            }
            BuildingType::SawMill => {
                builder = builder
                    .with(EntityContainer::new(bt.capacity(), TaskType::ChopTree))
                    .with(UiMenu::new_entity_container());
            }
            BuildingType::Tree => {
                builder = builder.with(ForestComponent::new(created));
            }
            _ => {}
        }

        self.place_building(tile_index, bt, builder.entity);

        let entity = builder.build();
        entity
    }
}

fn building_ingame_scaling(b: BuildingType) -> f32 {
    match b {
        BuildingType::PresentA | BuildingType::PresentB => 0.5,
        BuildingType::BlueFlowers => 0.6,
        BuildingType::RedFlowers => 0.45,
        _ => std::f32::NAN,
    }
}

use crate::net::graphql::buildings_query;
impl buildings_query::ResponseData {
    pub(crate) fn village_id(&self) -> VillageKey {
        VillageKey(self.village.id)
    }
    pub(crate) fn create_entities(&self, town_context: &mut TownContext) -> Vec<Entity> {
        self.village
            .buildings
            .iter()
            .map(|u| u.create_entity(town_context))
            .collect()
    }
}

impl buildings_query::BuildingsQueryVillageBuildings {
    fn create_entity(&self, town_context: &mut TownContext) -> Entity {
        let coordinates = (self.x as usize, self.y as usize);
        let maybe_range = self.building_range.map(|f| f as f32);
        let maybe_ap = self.attack_power.map(|f| f as i64);
        let bt = match self.building_type {
            buildings_query::BuildingType::RED_FLOWERS => BuildingType::RedFlowers,
            buildings_query::BuildingType::BLUE_FLOWERS => BuildingType::BlueFlowers,
            buildings_query::BuildingType::TREE => BuildingType::Tree,
            buildings_query::BuildingType::BUNDLING_STATION => BuildingType::BundlingStation,
            buildings_query::BuildingType::SAW_MILL => BuildingType::SawMill,
            buildings_query::BuildingType::PRESENT_A => BuildingType::PresentA,
            buildings_query::BuildingType::PRESENT_B => BuildingType::PresentB,
            buildings_query::BuildingType::TEMPLE => BuildingType::Temple,
            _ => panic!("Unexpected BuildingType"),
        };
        let created = GqlTimestamp::from_string(&self.creation).unwrap().into();

        let entities = town_context.town_world.entities();
        let lazy = town_context.town_world.read_resource::<LazyUpdate>();
        let mut town = town_context.town_mut();
        town.insert_building(
            &entities,
            &lazy,
            coordinates,
            bt,
            maybe_ap,
            self.attacks_per_cycle,
            maybe_range,
            created,
        )
    }
}
