mod welcome;
pub use welcome::*;
use paddlers_shared_lib::prelude::*;
use crate::gui::{
    gui_components::UiBox,
    sprites::WithSprite,
};

/// A unit can learn a limited number of Abilities. (including walking)
/// Although this simplifies things on the technical side, this is mainly
/// motivated from a game-design perspective. (simplicity)
pub const MAX_ABILITIES: usize = 4;

/// Represent the abilities a single unit instance has.
pub struct AbilitySet {
    abilities: [Option<AbilityType>; MAX_ABILITIES],
}

impl AbilitySet {
    pub fn new_test_set() -> AbilitySet {
        let abilities = [
            Some(AbilityType::Work),
            Some(AbilityType::Welcome),
            None,
            None,
        ];
        AbilitySet {
            abilities,
        }
    }
    pub fn construct_ui_box(&self) -> UiBox {
        let rows = 2;
        let mut ui = UiBox::new(MAX_ABILITIES / rows, rows, 0.0, 1.0);
        for a in &self.abilities {
            if let Some(ability) = a {
                ui.add(ability.sprite(), *ability);      
            }
        }
        ui
    }
}