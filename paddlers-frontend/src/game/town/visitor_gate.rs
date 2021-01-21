use std::collections::HashMap;

use crate::{
    game::{components::UiMenu, units::attackers::hobo_sprite_sad, Game},
    gui::{
        gui_components::{ClickOutput, UiElement},
        sprites::SpriteSet,
        ui_state::Now,
    },
    prelude::GameEvent,
};
use chrono::NaiveDateTime;
use paddle::{quicksilver_compat::Color, NutsCheck};
use paddlers_shared_lib::prelude::*;
use specs::WorldExt;
pub type GraphqlVisitingHobo = crate::net::graphql::attacks_query::AttacksQueryVillageAttacksUnits;

/// Stores information about incoming attacks
pub struct VisitorGate {
    // TODO: max_outstanding, currently_fighting
    queue: HashMap<AttackKey, WaitingAttack>,
}

/// A group of visitors waiting to be let in
pub struct WaitingAttack {
    pub arrival: NaiveDateTime,
    pub hobos: Vec<GraphqlVisitingHobo>,
}

impl VisitorGate {
    pub fn new() -> Self {
        Self {
            queue: Default::default(),
        }
    }

    pub fn queue_attack(&mut self, key: AttackKey, atk: WaitingAttack) {
        self.queue.insert(key, atk);
    }
}
impl Game {
    pub fn release_attack(&mut self, key: AttackKey) {
        let popped = self
            .world
            .write_resource::<VisitorGate>()
            .queue
            .remove(&key);
        if let Some(attack) = popped {
            let now = self.world.fetch::<Now>().0;
            self.insert_visitors_from_active_attack(attack.hobos, now)
                .nuts_check();
        }
    }
    pub fn queue_attack(&mut self, atk: WaitingAttack, key: AttackKey) {
        if let Some(gate_component) =
            super::Town::find_building(self.town_world(), BuildingType::Watergate)
        {
            if let Some(ui) = self
                .town_world()
                .write_component::<UiMenu>()
                .get_mut(gate_component)
            {
                let event = GameEvent::LetVisitorsIn(key);
                let img = hobo_sprite_sad(atk.hobos[0].hobo.color.as_ref().unwrap().into());
                ui.ui.add(
                    UiElement::new(ClickOutput::Event(event))
                        .with_image(SpriteSet::Simple(img))
                        .with_background_color(Color::BLACK),
                )
            }
        }
        self.world
            .write_resource::<VisitorGate>()
            .queue_attack(key, atk);
    }
}

impl WaitingAttack {
    pub fn new(arrival: NaiveDateTime, hobos: Vec<GraphqlVisitingHobo>) -> Self {
        Self { arrival, hobos }
    }
}
