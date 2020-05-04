use super::defence::*;
use super::town_layout::*;
use super::*;
use std::collections::HashMap;

struct TestHobo {
    max_hp: u32,
    speed: f32,
    hurried: bool,
    arrival: i64,
    released: Option<i64>,
    effects_strength: i32,
}
struct TestTown {
    building_auras: HashMap<TownLayoutIndex, Vec<TestAura>>,
}
#[derive(Copy, Clone, Debug)]
struct TestAura {
    id: usize,
    strength: i32,
}

const Y: i32 = TOWN_LANE_Y as i32;

#[test]
fn hurried_hobo_satisfied_and_gone() {
    let mut hobo = TestHobo::new();
    hobo.max_hp = 10;
    hobo.effects_strength = 8;
    let mut town = TestTown::new();
    let aura = TestAura::new(3);
    town.add_aura(aura, &[(1, Y), (2, Y), (3, Y)]);

    let now = 100_000;

    let hobo_left_town = town.hobo_left_town(&hobo, now);
    assert!(hobo_left_town);

    let hobo_hp_left = town.hp_left(&hobo, now);
    assert_eq!(hobo_hp_left, 0);

    let dmg = town.aura_damage(&hobo, now);
    assert_eq!(dmg, 3);

    let dmg = town.total_damage(&hobo, now);
    assert_eq!(dmg, 11);
}

#[test]
fn unhurried_hobo_satisfied_and_gone() {
    let mut hobo = TestHobo::new();
    hobo.max_hp = 10;
    hobo.effects_strength = 8;
    hobo.hurried = false;
    hobo.released = Some(0);
    let mut town = TestTown::new();
    let aura = TestAura::new(3);
    town.add_aura(aura, &[(1, Y), (2, Y), (3, Y)]);

    let now = 100_000;

    let hobo_left_town = town.hobo_left_town(&hobo, now);
    assert!(hobo_left_town);

    let hobo_hp_left = town.hp_left(&hobo, now);
    assert_eq!(hobo_hp_left, 0);

    let dmg = town.aura_damage(&hobo, now);
    assert_eq!(dmg, 3);

    let dmg = town.total_damage(&hobo, now);
    assert_eq!(dmg, 11);
}

#[test]
fn unhurried_hobo_resting() {
    let mut hobo = TestHobo::new();
    hobo.max_hp = 10;
    hobo.effects_strength = 8;
    hobo.hurried = false;
    hobo.released = None;
    let mut town = TestTown::new();
    let aura = TestAura::new(3);
    town.add_aura(aura, &[(1, Y), (2, Y), (3, Y)]);

    let now = 100_000;

    let hobo_left_town = town.hobo_left_town(&hobo, now);
    assert!(!hobo_left_town);

    let hobo_hp_left = town.hp_left(&hobo, now);
    assert_eq!(hobo_hp_left, 2);

    let dmg = town.aura_damage(&hobo, now);
    assert_eq!(dmg, 0);

    let dmg = town.total_damage(&hobo, now);
    assert_eq!(dmg, 8);
}

impl TestHobo {
    fn new() -> Self {
        TestHobo {
            max_hp: 100,
            speed: 0.5,
            hurried: true,
            arrival: 0,
            released: None,
            effects_strength: 0,
        }
    }
}

impl IAttackingHobo for TestHobo {
    fn max_hp(&self) -> u32 {
        self.max_hp
    }
    fn speed(&self) -> f32 {
        self.speed
    }
    fn hurried(&self) -> bool {
        self.hurried
    }
    fn arrival(&self) -> i64 {
        self.arrival
    }
    fn released(&self) -> Option<i64> {
        self.released
    }
    fn effects_strength(&self) -> i32 {
        self.effects_strength
    }
}
impl ITownLayoutMarker for TestTown {
    const LAYOUT: TownLayout = TownLayout::Basic;
}
impl IDefendingTown for TestTown {
    type AuraId = usize;
    fn auras_in_range(&self, index: &Self::Index, _time: i64) -> Vec<(Self::AuraId, i32)> {
        if let Some(auras) = self.building_auras.get(index) {
            auras.iter().map(|aura| (aura.id, aura.strength)).collect()
        } else {
            Vec::new()
        }
    }
}
impl TestAura {
    pub fn new(strength: i32) -> Self {
        static mut N: usize = 0;
        let id = unsafe { N };
        unsafe {
            N = N + 1;
        }
        TestAura { id, strength }
    }
}
impl TestTown {
    pub fn new() -> Self {
        TestTown {
            building_auras: HashMap::new(),
        }
    }
    fn add_aura(&mut self, aura: TestAura, idx: &[TownLayoutIndex]) {
        for i in idx {
            self.building_auras
                .entry(*i)
                .or_insert_with(Vec::new)
                .push(aura);
        }
    }
}
