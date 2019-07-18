use quicksilver::geom::{Vector, Shape};
use specs::prelude::*;
use crate::gui::{
    render::Renderable,
    sprites::{SpriteIndex,WithSprite},
    z::Z_UNITS,
    utils::*,
};
use crate::game::{
    Game,
    input::Clickable,
    movement::Position,
    fight::{Range,Aura}
};
use crate::net::game_master_api::*;
use paddlers_shared_lib::api::shop::*;
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::api::attributes::Attributes;

impl Game<'_,'_> {
    fn insert_new_bulding(&mut self, pos: (i32, i32), bt: BuildingType) -> Entity {
        self.insert_bulding(pos, bt, bt.attack_power(), bt.attacks_per_cycle(), bt.range())
    }

    fn insert_bulding(&mut self, pos: (i32, i32), bt: BuildingType, ap: Option<i64>, attacks_per_cycle: Option<i64>,  range: Option<f32>) -> Entity {
        let posv: Vector = pos.into();
        let ul = self.unit_len.unwrap();
        let mut builder = 
            self.world.create_entity()
            .with(Position::new(posv * ul , (ul, ul), Z_UNITS))
            .with(
                Renderable {
                    kind: RenderVariant::ImgWithImgBackground(bt.sprite(), SpriteIndex::Grass),
                }
            )
            .with(Clickable);

        if let Some(r) = range {
            builder = builder.with(Range::new(r));
        }

        // No (None) attacks per cycle && Some ap => Aura effect
        if attacks_per_cycle.is_none() && ap.is_some() {
            if let Some(r) = range {
                builder = builder.with(Aura::new(r, ap.unwrap(), (pos.0 as usize, pos.1 as usize), &self.town))
            }
        }

        builder.build()
    }

    pub fn purchase_building(&mut self, building_type: BuildingType, pos: (usize, usize)) {
        let msg = BuildingPurchase {
            building_type: building_type, 
            x: pos.0,
            y: pos.1,
        };
        use futures_util::future::FutureExt;
        stdweb::spawn_local(
            http_place_building(msg).map( 
                |r| {
                    if r.is_err() {
                        println!("Buying building failed: {:?}", r);
                    }
                }
            )
        );
        // optimistically build
        self.resources.spend(&building_type.price());
        self.insert_new_bulding((pos.0 as i32, pos.1 as i32), building_type);
    }

    pub fn delete_building(&mut self, entity: Entity) {
        let pos_store = self.world.read_storage::<Position>();
        let pos = pos_store.get(entity).unwrap();
        let (x,y) = self.town.tile(pos.area.center());
        std::mem::drop(pos_store);

        let msg = BuildingDeletion { x: x, y: y };
        use futures_util::future::FutureExt;
        stdweb::spawn_local(
            http_delete_building(msg).map( 
                |r| {
                    if r.is_err() {
                        println!("Deleting building failed: {:?}", r);
                    }
                }
            )
        );
        // optimistically delete
        let result = self.world.delete_entity(entity);
        if let Err(e) = result {
            println!("Someting went wrong while deleting: {}", e);
        }
    }
}

use crate::net::graphql::buildings_query;
impl buildings_query::ResponseData {
    pub (crate) fn create_entities(&self, game: &mut Game) -> Vec<Entity> {
        self.buildings
            .iter()
            .map(|u|{u.create_entity(game)})
            .collect()
    }
}

impl buildings_query::BuildingsQueryBuildings {
    fn create_entity(&self, game: &mut Game) -> Entity {
        let coordinates = (self.x as i32,self.y as i32);
        let maybe_range = self.building_range.map(|f| f as f32);
        let maybe_ap = self.attack_power.map(|f| f as i64);
        let bt = match self.building_type {
            buildings_query::BuildingType::RED_FLOWERS => BuildingType::RedFlowers,
            buildings_query::BuildingType::BLUE_FLOWERS => BuildingType::BlueFlowers,
            buildings_query::BuildingType::TREE => BuildingType::Tree,
            _ => panic!("Unexpected BuildingType"),
        };
        game.insert_bulding(coordinates, bt, maybe_ap, self.attacks_per_cycle, maybe_range)
    }
}