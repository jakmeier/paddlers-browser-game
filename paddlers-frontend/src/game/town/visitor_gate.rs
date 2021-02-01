use std::collections::HashMap;

use crate::{
    game::{components::UiMenu, units::attackers::hobo_sprite_sad, Game},
    gui::{
        gui_components::{ClickOutput, InteractiveTableArea, UiBox, UiElement},
        sprites::SingleSprite,
        ui_state::Now,
        utils::{ImageCollection, RenderVariant, SubImg},
        z::Z_UI_MENU,
    },
    prelude::GameEvent,
};
use chrono::NaiveDateTime;
use paddle::NutsCheck;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

pub type GraphqlVisitingHobo = crate::net::graphql::attacks_query::AttacksQueryVillageAttacksUnits;

/// Stores information about incoming attacks
pub struct VisitorGate {
    queue: HashMap<AttackKey, WaitingAttack>,
    /// Number of attacks not yet arrived at the watergate
    inflight_visitor_groups: usize,
    town_entity: Option<Entity>,
}

/// A group of visitors waiting to be let in
pub struct WaitingAttack {
    pub key: AttackKey,
    pub arrival: NaiveDateTime,
    pub shown_as_arrived: bool,
    pub hobos: Vec<GraphqlVisitingHobo>,
}

impl VisitorGate {
    pub fn new() -> Self {
        Self {
            queue: Default::default(),
            inflight_visitor_groups: 0,
            town_entity: None,
        }
    }
    pub fn inflight_visitor_groups(&self) -> usize {
        self.inflight_visitor_groups
    }
    pub fn set_inflight_visitor_groups(&mut self, n: usize) {
        self.inflight_visitor_groups = n;
    }
    pub fn update_entity(&mut self, town_entity: Entity) {
        self.town_entity = Some(town_entity);
    }
    pub fn queue_attack(&mut self, ui: &mut UiBox, atk: WaitingAttack) {
        ui.add(atk.ui_element());
        self.queue.insert(atk.key, atk);
    }
}
impl Game {
    pub fn release_attack(&mut self, key: AttackKey) {
        let popped = self
            .home_town_world()
            .write_resource::<VisitorGate>()
            .queue
            .remove(&key);
        if let Some(attack) = popped {
            let now = self.world.fetch::<Now>().0;
            if let Some(gate_component) =
                super::Town::find_building(self.town_world(), BuildingType::Watergate)
            {
                if let Some(ui) = self
                    .town_world()
                    .write_component::<UiMenu>()
                    .get_mut(gate_component)
                {
                    ui.ui.remove(attack.click_output());
                }
                self.town_mut()
                    .add_entity_to_building_by_id(gate_component)
                    .nuts_check();
            }
            self.insert_visitors_from_active_attack(attack.hobos, now)
                .nuts_check();
        }
    }
    pub fn queue_attack(&mut self, atk: WaitingAttack) {
        if let Some(gate_component) =
            super::Town::find_building(self.town_world(), BuildingType::Watergate)
        {
            if let Some(ui) = self
                .town_world()
                .write_component::<UiMenu>()
                .get_mut(gate_component)
            {
                let mut gate = self.home_town_world().write_resource::<VisitorGate>();
                gate.update_entity(gate_component);
                gate.queue_attack(&mut ui.ui, atk);
            }
            self.town_mut()
                .add_entity_to_building_by_id(gate_component)
                .nuts_check();
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct WatergateQueueSystem;

impl<'a> System<'a> for WatergateQueueSystem {
    type SystemData = (
        WriteStorage<'a, UiMenu>,
        WriteExpect<'a, VisitorGate>,
        ReadExpect<'a, Now>,
    );

    fn run(&mut self, (mut uis, mut gate, now): Self::SystemData) {
        let now = now.0;
        if let Some(entity) = gate.town_entity {
            if let Some(ui) = uis.get_mut(entity) {
                for atk in gate.queue.values_mut() {
                    if !atk.shown_as_arrived && atk.arrival <= now {
                        ui.ui.remove(atk.click_output());
                        atk.shown_as_arrived = true;
                        ui.ui.add(atk.ui_element());
                    }
                }
            }
        }
    }
}

impl WaitingAttack {
    pub fn new(
        arrival: NaiveDateTime,
        hobos: Vec<GraphqlVisitingHobo>,
        shown_as_arrived: bool,
        key: AttackKey,
    ) -> Self {
        Self {
            arrival,
            hobos,
            shown_as_arrived,
            key,
        }
    }
    fn ui_element(&self) -> UiElement {
        UiElement::new(self.click_output()).with_render_variant(self.render_variant())
    }
    fn render_variant(&self) -> RenderVariant {
        let main_img = hobo_sprite_sad(self.hobos[0].hobo.color.as_ref().unwrap().into());
        if self.shown_as_arrived {
            WaitingAttack::arrived_render_variant(main_img)
        } else {
            WaitingAttack::travelling_render_variant(main_img)
        }
    }
    fn click_output(&self) -> ClickOutput {
        if self.shown_as_arrived {
            let event = GameEvent::LetVisitorsIn(self.key);
            ClickOutput::Event(event)
        } else {
            let event = GameEvent::DisplayConfirmation("visitor-not-here".into());
            ClickOutput::Event(event)
        }
    }
    fn arrived_render_variant(main_img: SingleSprite) -> RenderVariant {
        let inner_size = (1.2, 0.8);
        let inner_offset = (0.15, 0.1);
        RenderVariant::ImgCollection(ImageCollection::new(
            (1.0, 1.0),
            vec![
                SubImg::new(
                    SingleSprite::SingleDuckBackgroundShape,
                    (0.0, 0.0),
                    (1.5, 1.0),
                    Z_UI_MENU,
                ),
                SubImg::new(main_img, inner_offset, inner_size, Z_UI_MENU + 1),
            ],
        ))
    }
    fn travelling_render_variant(main_img: SingleSprite) -> RenderVariant {
        let inner_size = (1.2, 0.8);
        let inner_offset = (0.15, 0.1);
        RenderVariant::ImgCollection(ImageCollection::new(
            (1.0, 1.0),
            vec![
                SubImg::new(main_img, inner_offset, inner_size, Z_UI_MENU + 1),
                SubImg::new(
                    SingleSprite::SingleDuckShape,
                    inner_offset,
                    inner_size,
                    Z_UI_MENU + 2,
                ),
            ],
        ))
    }
    fn empty_render_variant() -> RenderVariant {
        Self::arrived_render_variant(SingleSprite::SingleDuckShape)
    }
}
