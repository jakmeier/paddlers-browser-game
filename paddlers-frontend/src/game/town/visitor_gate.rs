use std::collections::HashMap;

use crate::{
    game::{
        components::{NetObj, UiMenu},
        units::attackers::hobo_sprite_sad,
        Game,
    },
    gui::{
        gui_components::{ClickOutput, InteractiveTableArea, UiBox, UiElement},
        sprites::{SingleSprite, SpriteSet},
        ui_state::Now,
        utils::{ImageCollection, RenderVariant, SubImg},
        z::Z_UI_MENU,
    },
    net::game_master_api::{GameMasterMessage, HttpUpgradeBuilding},
    prelude::{GameEvent, PadlError, PadlErrorCode},
};
use chrono::NaiveDateTime;
use paddle::NutsCheck;
use paddlers_shared_lib::{game_mechanics::attributes::Attributes, prelude::*};
use specs::prelude::*;

pub type GraphqlVisitingHobo = crate::net::graphql::attacks_query::AttacksQueryVillageAttacksUnits;

/// Stores information about incoming attacks
pub struct VisitorGate {
    queue: HashMap<AttackKey, WaitingAttack>,
    town_entity: Option<Entity>,
    display_capacity: usize,
    level: usize,
    net_id: Option<NetObj>,
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
            town_entity: None,
            display_capacity: 1,
            level: 1,
            net_id: None,
        }
    }
    pub fn queue_attack(&mut self, ui: &mut UiBox, atk: WaitingAttack) {
        ui.add(atk.ui_element());
        self.queue.insert(atk.key, atk);
        self.complement_ui_table(ui);
    }
    pub fn complement_ui_table(&self, ui: &mut UiBox) {
        ui.remove(WaitingAttack::empty_slot_click_output());
        ui.remove(self.upgrade_click_output());
        let in_queue = self.queue.len();
        if in_queue < self.display_capacity {
            let required_empty_slots = self.display_capacity - in_queue;
            for _ in 0..required_empty_slots {
                ui.add(WaitingAttack::empty_slot());
            }
        }
        if let Some(button) = self.upgrade_button() {
            ui.add(button);
        }
    }
}
impl Game {
    /// Refresh internal state of watergate to make sure the capacity is displayed correctly + update town state that remembers how long the queue is (for checks on sending new invites)
    pub fn refresh_visitor_gate(&self) {
        if let Some((gate_entity, gate_building)) =
            super::Town::find_building(self.home_town_world(), BuildingType::Watergate)
        {
            let mut gate = self.home_town_world().write_resource::<VisitorGate>();
            gate.town_entity = Some(gate_entity);
            gate.level = gate_building.level as usize;
            gate.display_capacity =
                BuildingType::Watergate.visitor_queue_capacity(gate_building.level);
            gate.net_id = self
                .home_town_world()
                .read_component::<NetObj>()
                .get(gate_entity)
                .cloned();
            let mut uis = self.home_town_world().write_component::<UiMenu>();
            if let Some(ui) = uis.get_mut(gate_entity) {
                ui.ui.clear();
                for atk in gate.queue.values() {
                    ui.ui.add(atk.ui_element());
                }
                gate.complement_ui_table(&mut ui.ui);
            }
            self.town_mut()
                .set_entity_count_for_building_by_id(gate_entity, gate.queue.len())
                .nuts_check();
        }
    }
    pub fn queue_attack(&mut self, atk: WaitingAttack) {
        let mut gate = self.home_town_world().write_resource::<VisitorGate>();
        if let Some(gate_entity) = gate.town_entity {
            // Update the UiMenu for the watergate
            if let Some(ui) = self
                .home_town_world()
                .write_component::<UiMenu>()
                .get_mut(gate_entity)
            {
                gate.queue_attack(&mut ui.ui, atk);
            }
            // This is necessary to update the tile state and keep it consistent.
            // That way, the centrally defined logic (paddlers-shared-lib) can be used to check capacity.
            // (Does that make sense? Seems overcomplicated.)
            self.town_mut()
                .add_entity_to_building_by_id(gate_entity)
                .nuts_check();
        } else {
            NutsCheck::<()>::nuts_check(Err(PadlError::dev_err(PadlErrorCode::DevMsg(
                "Watergate not initialized or not present in town",
            ))));
        }
    }
    pub fn release_attack(&mut self, key: AttackKey) {
        let mut gate = self.home_town_world().write_resource::<VisitorGate>();
        let popped = gate.queue.remove(&key);
        if let Some(attack) = popped {
            let now = self.world.fetch::<Now>().0;
            if let Some(gate_entity) = gate.town_entity {
                if let Some(ui) = self
                    .home_town_world()
                    .write_component::<UiMenu>()
                    .get_mut(gate_entity)
                {
                    ui.ui.remove(attack.click_output());
                    gate.complement_ui_table(&mut ui.ui);
                }
                // Keep town/tile state consistent
                self.town_mut()
                    .remove_entity_from_building_by_id(gate_entity)
                    .nuts_check();
            }
            std::mem::drop(gate);
            self.insert_visitors_from_active_attack(attack.hobos, now)
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
            (1.5, 1.0),
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
            (1.5, 1.0),
            vec![
                SubImg::new(
                    SingleSprite::SingleDuckBackgroundShape,
                    (0.0, 0.0),
                    (1.5, 1.0),
                    Z_UI_MENU,
                ),
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
    fn empty_slot_click_output() -> ClickOutput {
        ClickOutput::DoNothing
    }
    fn empty_slot() -> UiElement {
        UiElement::new(Self::empty_slot_click_output()).with_render_variant(
            RenderVariant::ImgCollection(ImageCollection::new(
                (1.5, 1.0),
                vec![SubImg::new(
                    SingleSprite::SingleDuckBackgroundShape,
                    (0.0, 0.0),
                    (1.5, 1.0),
                    Z_UI_MENU,
                )],
            )),
        )
    }
}
impl VisitorGate {
    fn upgrade_click_output(&self) -> ClickOutput {
        if let Some(building) = self.net_id.as_ref().and_then(NetObj::as_building) {
            ClickOutput::Event(GameEvent::SendGameMasterMessage(
                GameMasterMessage::UpgradeBuilding(HttpUpgradeBuilding {
                    entity: self.town_entity.expect("Entity for watergate invalidated?"),
                    building,
                    current_level: self.level,
                }),
            ))
        } else {
            ClickOutput::DoNothing
        }
    }
    fn upgrade_button(&self) -> Option<UiElement> {
        if let Some(upgrade_price) = BuildingType::Watergate.upgrade_cost(self.level) {
            Some(
                UiElement::new(self.upgrade_click_output())
                    .with_render_variant(RenderVariant::Img(SpriteSet::Simple(SingleSprite::Plus)))
                    .with_cost(upgrade_price),
            )
        } else {
            None
        }
    }
}
