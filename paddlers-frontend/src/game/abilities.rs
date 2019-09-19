mod welcome;
pub use welcome::*;
use crate::gui::{
    gui_components::UiBox,
    sprites::WithSprite,
};

/// Abilities are attributes of worker and hero units.
/// They are closely related to Tasks but there is no one-to-one correspondence.
/// TODO: Probably move to shared lib as soon as implementation on backend starts 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ability {
    Walk,
    Welcome,
}

/// A unit can learn a limited number of Abilities. (including walking)
/// Although this simplifies things on the technical side, this is mainly
/// motivated from a game-design perspective. (simplicity)
pub const MAX_ABILITIES: usize = 4;

/// Represent the abilities a single unit instance has.
pub struct AbilitySet {
    abilities: [Option<Ability>; MAX_ABILITIES],
}

impl AbilitySet {
    pub fn new_test_set() -> AbilitySet {
        let abilities = [
            Some(Ability::Walk),
            Some(Ability::Welcome),
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