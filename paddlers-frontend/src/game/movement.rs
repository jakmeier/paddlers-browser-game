use crate::Timestamp;
use crate::game::resources::Now;
use quicksilver::geom::{Vector, Rectangle};
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub area: Rectangle,
    pub z: i32, 
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Moving {
    pub start_ts: Timestamp,
    pub start_pos: Vector,
    // Speed: f32 in pixels per second
    pub momentum: Vector,
    pub max_speed: f32,
}


pub struct MoveSystem;
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, Now>,
        ReadStorage<'a, Moving>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, (t, vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.area.pos = vel.position(t.0);
        }
    }

}

impl Position {
    pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>, z: i32) -> Self {
        Position {
            area: Rectangle::new(pos.into(), size.into()),
            z: z,
        }
    }
}

impl Moving {
    pub fn new (t0: Timestamp, start_pos: impl Into<Vector>, vel: impl Into<Vector>, max_speed: f32) -> Self {
        let  v = vel.into();
        let start = start_pos.into();
        Moving {
            start_ts: t0,
            start_pos: start,
            momentum: v,
            max_speed: max_speed,
        }
    }
    pub fn position(&self, t: Timestamp) -> Vector {
        self.start_pos + self.momentum * (t - self.start_ts) as f32 / 1_000_000
    }
    pub fn stand_still(&mut self, timestamp: i64) {
        self.start_pos = self.position(timestamp);
        self.start_ts = timestamp;
        self.momentum = (0.0,0.0).into();
    }
}