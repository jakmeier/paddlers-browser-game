use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::gui::{
    render::{RenderType, Renderable},
    sprites::SpriteIndex,
    z::Z_UNITS
};
use crate::game::{
    Game,
    input::Clickable,
    movement::Position,
    fight::Range
};
use crate::net::game_master_api::http_place_building;
use duck_family_api_lib::shop::*;
use duck_family_api_lib::types::*;
use duck_family_api_lib::attributes::Attributes;

impl Game<'_,'_> {

    fn insert_flowers(&mut self, pos: (i32, i32), range: Option<f32>) -> Entity {
        let pos: Vector = pos.into();
        let ul = self.unit_len.unwrap();
        let builder = 
            self.world.create_entity()
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
                        println!("Buying buidling failed: {:?}", r);
                    }
                }
            )
        );
        // optimistically build
        self.resources.spend(&building_type.price());
        self.insert_flowers((pos.0 as i32, pos.1 as i32), building_type.range());
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
        game.insert_flowers(coordinates, maybe_range)
    }
}