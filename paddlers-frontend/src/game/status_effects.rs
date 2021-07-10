use crate::gui::{
    gui_components::*,
    sprites::{SingleSprite, SpriteSet},
};
use crate::net::graphql::query_types::HoboEffect;
use crate::prelude::*;
use paddlers_shared_lib::models::*;
use specs::prelude::*;
use specs::storage::BTreeStorage;

#[derive(Component, Debug, Clone)]
#[storage(BTreeStorage)]
pub struct StatusEffects {
    health: Option<StatusEffect>,
}

/// An effect to be displayed on a selected entity.
/// It can be, for example, a health reduction from a welcome ability
#[derive(Debug, Clone)]
struct StatusEffect {
    img: SpriteSet,
    // text: String, // TODO: add and display text on hover
    value: i32,
}

impl StatusEffects {
    pub fn new() -> Self {
        StatusEffects { health: None }
    }
    pub fn from_gql_query(effects: &[HoboEffect]) -> PadlResult<Self> {
        let mut status = Self::new();
        for ef in effects {
            match ef.attribute {
                HoboAttributeType::Health => {
                    let strength = ef.strength.ok_or(PadlError::dev_err(
                        PadlErrorCode::InvalidGraphQLData("Health effect without strength"),
                    ))?;
                    status.add_health_reduction(strength as i32);
                }
                _ => {
                    return PadlErrorCode::InvalidGraphQLData("Unknown Effect").dev();
                }
            }
        }
        Ok(status)
    }
    pub fn add_health_reduction(&mut self, v: i32) {
        if self.health.is_none() {
            self.health = Some(StatusEffect::new_health_reduction(v));
        } else {
            self.health.as_mut().unwrap().value += v;
        }
    }
    pub fn menu_table_infos<'a>(&self) -> Vec<TableRow<'a>> {
        let mut rows = vec![];
        if let Some(h) = &self.health {
            rows.push(h.details());
        }
        rows
    }
}

impl StatusEffect {
    fn new_health_reduction(init_val: i32) -> Self {
        StatusEffect {
            // For now, health reduction == welcome ability
            img: SpriteSet::Simple(SingleSprite::WelcomeAbility),
            value: init_val,
        }
    }
    fn details<'a>(&self) -> TableRow<'a> {
        let text = if self.value >= 0 {
            format!("{}{}", "+", self.value)
        } else {
            format!("{}", self.value)
        };
        TableRow::TextWithImage(text, self.img.default(), TextColor::Black)
    }
}
