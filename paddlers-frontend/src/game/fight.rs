// use quicksilver::prelude::*;
use specs::storage::BTreeStorage;
use specs::prelude::*;
use specs::world::Index;
use crate::game::{
    movement::Position,
    town::Town,
    UnitLength,
    game_event_manager::{EventPool, GameEvent},
};

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
/// Range to be displayed when hovering
pub struct Range {
    pub range: f32,
}
impl Range {
    pub fn new(range: f32) -> Self {
        Range {
            range: range
        }
    }
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Aura {
    pub affected_tiles: Vec<(usize, usize)>,
    pub effect: i64,
}
impl Aura {
    pub fn new(range: f32, ap: i64, tile: (usize,usize), town: &Town) -> Self {
        let mut tiles = town.lane_in_range(tile, range);
        if !tiles.is_sorted(){
            tiles.sort();
        }
        Aura {
            affected_tiles: tiles,
            effect: ap,
        }
    }
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Health {
    pub max_hp: i64,
    pub hp: i64,
    // Used for effects that affect the unit once per defender
    pub aura_effects: Vec<Index>,
}
impl Health {
    pub fn new_full_health(hp: i64) -> Self {
        Health {
            max_hp: hp,
            hp: hp,
            aura_effects: vec![],
        }
    }
    pub fn make_happy(&mut self, amount: i64, id: Entity, ep: &EventPool) {
        let new_hp = 0.max(self.hp - amount);
        if new_hp == 0 && self.hp != 0 {
            ep.send(GameEvent::HoboSatisfied(id)).expect("sending event");
        }
        self.hp = new_hp;
    }
}

#[derive(Clone)]
pub struct FightSystem {
    counter: usize,
    event_pool: EventPool,
}
impl FightSystem {
    pub fn new(event_pool: EventPool) -> Self {
        FightSystem {
            counter: 0,
            event_pool
        }
    }
}

impl<'a> System<'a> for FightSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Aura>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Health>,
        Read<'a, UnitLength>,
     );

    fn run(&mut self, (entities, aura, position, mut health, ul): Self::SystemData) {
        // It's not necessary to recalculate every frame
        self.counter = (self.counter + 1) % 30;
        if self.counter != 1 {
            return;
        }

        // This algorithm runs in O(n*m*(log(m)+log(t))
        // n attacker, m defenders, t tiles
        // n can be arbitrarily large in late game
        // m will most likely remain limited by the map size
        // t is always smaller than the map lane size
        for (aid, a) in (&entities, &aura).join() { // m
            for (hid, p, h) in (&entities, &position, &mut health).join() { // n
                let tile = Town::find_tile(p.area.pos, ul.0);
                if a.affected_tiles.binary_search(&tile).is_ok() { // log t
                    match h.aura_effects.binary_search(&aid.id()) { // log m
                        Ok(_) => {/* Aura already active*/},
                        Err(i) => { 
                            (*h).make_happy(a.effect, hid, &self.event_pool);
                            (*h).aura_effects.insert(i, aid.id()); // [Theoretically O(m) but not considered above]
                        }
                    }
                }
            }
        }
    }
}