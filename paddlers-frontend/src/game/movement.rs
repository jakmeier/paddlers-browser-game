use crate::game::Now;
use chrono::NaiveDateTime;
use paddle::quicksilver_compat::about_equal;
use paddle::*;
use specs::prelude::*;
use specs::storage::BTreeStorage;

#[derive(Component, Debug)]
#[storage(VecStorage)]
/// A position on the town view
pub struct Position {
    pub area: Rectangle,
    pub z: i16,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Moving {
    pub start_ts: NaiveDateTime,
    pub start_pos: Vector,
    // Speed: f32 in pixels per second
    pub momentum: Vector,
    pub max_speed: f32,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct TargetPosition {
    pub pos: Vector,
}

pub struct MoveSystem;
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Now>,
        WriteStorage<'a, Moving>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, TargetPosition>,
    );

    fn run(&mut self, (entities, t, mut vel, mut pos, mut target_pos): Self::SystemData) {
        let mut remove_from_vel = vec![];
        for (e, v, pos) in (&entities, &mut vel, &mut pos).join() {
            let mut new_pos = v.position(t.0);
            if let Some(target) = target_pos.get(e) {
                // test if target position is reached or crossed
                if target.is_reached(pos.area.pos, new_pos) {
                    new_pos = target.pos;
                    remove_from_vel.push(e);
                    target_pos.remove(e);
                }
            }
            pos.area.pos = new_pos;
        }
        for e in remove_from_vel {
            vel.remove(e);
        }
    }
}

impl Position {
    pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>, z: i16) -> Self {
        Position {
            area: Rectangle::new(pos.into(), size.into()),
            z: z,
        }
    }
}

impl Moving {
    pub fn new(
        t0: NaiveDateTime,
        start_pos: impl Into<Vector>,
        vel: impl Into<Vector>,
        max_speed: f32,
    ) -> Self {
        let v = vel.into();
        let start = start_pos.into();
        Moving {
            start_ts: t0,
            start_pos: start,
            momentum: v,
            max_speed: max_speed,
        }
    }
    pub fn position(&self, t: NaiveDateTime) -> Vector {
        self.start_pos
            + self.momentum * (t - self.start_ts).num_microseconds().unwrap() as f32 / 1_000_000
    }
    pub fn stand_still(&mut self, timestamp: NaiveDateTime) {
        self.start_pos = self.position(timestamp);
        self.start_ts = timestamp;
        self.momentum = (0.0, 0.0).into();
    }
}

impl TargetPosition {
    pub fn new(pos: Vector) -> Self {
        TargetPosition { pos }
    }
    /// Given a movement between the two points, is the target position reached now?
    fn is_reached(&self, before: Vector, after: Vector) -> bool {
        // uses about_equal for floats
        if before == self.pos {
            true
        } else if about_equal(self.pos.y, after.y) {
            (after.x - self.pos.x).signum() != (before.x - self.pos.x).signum()
        } else if about_equal(self.pos.x, after.x) {
            (after.y - self.pos.y).signum() != (before.y - self.pos.y).signum()
        } else {
            false
        }
    }
}
