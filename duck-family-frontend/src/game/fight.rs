// use quicksilver::prelude::*;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
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
#[storage(VecStorage)]
pub struct Health {
    pub max_hp: i64,
    pub hp: i64,
}
impl Health {
    pub fn new_full_health(hp: i64) -> Self {
        Health {
            max_hp: hp,
            hp: hp,
        }
    }
}