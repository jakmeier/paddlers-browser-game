use specs::storage::BTreeStorage;
use specs::prelude::*;
pub use crate::gui::{
    utils::*,
    gui_components::*,
    sprites::{SpriteSet, SingleSprite},
    animation::AnimationState,
    render::Renderable,
    input::Clickable,
};
pub use super::movement::{Moving, Position};
pub use super::fight::{Health, Range};
pub use super::forestry::ForestComponent;
pub use super::map::{VillageMetaInfo, MapPosition};

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
        StatusEffects {
            health: None,
        }
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
        let text = 
        if self.value >= 0 {
            format!("{}{}", "+", self.value)
        } else {
            format!("{}", self.value)
        };
        TableRow::TextWithImage(
            text,
            self.img.default(),
        )
    }
}