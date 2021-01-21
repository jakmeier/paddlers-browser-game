use super::*;
use crate::game::buildings::Building;
use crate::game::fight::Aura;
use crate::net::graphql::attacks_query::{AttacksQueryVillageAttacksUnits, HoboAttributeType};
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::graphql_types::*;
use specs::prelude::*;

pub(crate) struct AttackingHobo {
    pub unit: AttacksQueryVillageAttacksUnits,
    pub start_of_fight: Timestamp,
}

impl IAttackingHobo for AttackingHobo {
    fn max_hp(&self) -> u32 {
        self.unit.hobo.hp as u32
    }
    fn speed(&self) -> f32 {
        self.unit.hobo.speed as f32
    }
    fn hurried(&self) -> bool {
        self.unit.hobo.hurried
    }
    fn start_of_fight(&self) -> Option<Timestamp> {
        Some(self.start_of_fight)
    }
    fn released(&self) -> Option<Timestamp> {
        self.unit
            .info
            .released
            .as_ref()
            .map(|t| GqlTimestamp::from_string(&t).unwrap().to_chrono().into())
    }
    fn effects_strength(&self) -> i32 {
        self.unit
            .hobo
            .effects
            .iter()
            .filter(|e| e.attribute == HoboAttributeType::HEALTH)
            .filter(|e| e.strength.is_some())
            .fold(0, |acc, e| acc + e.strength.unwrap() as i64) as i32
    }
}

impl<'a, 'b> ITownLayoutMarker for Game {
    const LAYOUT: TownLayout = TownLayout::Basic;
}
impl<'a, 'b> IDefendingTown for Game {
    type AuraId = u32;
    fn auras_in_range(&self, index: &Self::Index, time: Timestamp) -> Vec<(Self::AuraId, i32)> {
        let mut out = vec![];

        let world = self.town_world();
        let auras = world.read_component::<Aura>();
        let buildings = world.read_component::<Building>();
        let entities = world.entities();
        for (aura, e, b) in (&auras, &entities, &buildings).join() {
            if time < b.built.into() {
                continue;
            }
            for tile in &aura.affected_tiles {
                if tile.0 == index.0 && tile.1 == index.1 {
                    out.push((e.id(), aura.effect as i32))
                }
            }
        }
        out
    }
}
