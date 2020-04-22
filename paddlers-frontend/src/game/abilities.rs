mod welcome;
use crate::gui::{
    gui_components::{UiBox, UiElement},
    sprites::{SingleSprite, WithSprite},
    utils::RenderVariant,
};
use crate::net::graphql::query_types::parse_timestamp;
use crate::prelude::*;
use paddlers_shared_lib::prelude::*;
pub use welcome::*;

/// A unit can learn a limited number of Abilities. (including walking)
/// Although this simplifies things on the technical side, this is mainly
/// motivated from a game-design perspective. (simplicity)
pub const MAX_ABILITIES: usize = 4;

/// Represent the abilities a single unit instance has.
pub struct AbilitySet {
    abilities: [Option<AbilityType>; MAX_ABILITIES],
    last_used: [Option<Timestamp>; MAX_ABILITIES],
}

use crate::net::graphql::village_units_query::VillageUnitsQueryVillageWorkersAbilities;
impl AbilitySet {
    pub fn from_gql(
        gql_abilities: &[VillageUnitsQueryVillageWorkersAbilities],
    ) -> PadlResult<AbilitySet> {
        if gql_abilities.len() > MAX_ABILITIES {
            return PadlErrorCode::InvalidGraphQLData("Too many abilities").dev();
        }
        let mut abilities: [Option<AbilityType>; MAX_ABILITIES] = [None; MAX_ABILITIES];
        let mut last_used: [Option<Timestamp>; MAX_ABILITIES] = [None; MAX_ABILITIES];
        let mut i = 0;
        for gqla in gql_abilities {
            abilities[i] = Some((&gqla.ability_type).into());
            last_used[i] = gqla.last_used.as_ref().map(parse_timestamp);
            i += 1;
        }
        Ok(AbilitySet {
            abilities,
            last_used,
        })
    }
    pub fn construct_ui_box(&self) -> UiBox {
        let rows = 1;
        let mut ui = UiBox::new(MAX_ABILITIES / rows, rows, 15.0, 5.0);
        for i in 0..MAX_ABILITIES {
            let a = self.abilities[i];
            let lu = self.last_used[i];
            if let Some(ability) = a {
                let mut el = UiElement::new(ability).with_render_variant(
                    RenderVariant::ImgWithImgBackground(
                        ability.sprite(),
                        SingleSprite::FrameGreen3,
                    ),
                );
                if let Some(t) = lu {
                    el = el.with_cooldown(t, t + ability.cooldown().num_microseconds().unwrap());
                }
                ui.add(el);
            }
        }
        ui
    }
}
