use specs::prelude::*;
use crate::game::{
    Now,
    movement::Moving,
    units::workers::Worker,
    town::Town,
};
use crate::gui::animation::*;
use paddlers_shared_lib::models::*;
use quicksilver::geom::about_equal;

pub struct WorkerSystem;

impl<'a> System<'a> for WorkerSystem {
    type SystemData = (
        Entities<'a>,    
        WriteStorage<'a, Worker>,
        WriteStorage<'a, Moving>,
        WriteStorage<'a, AnimationState>,
        Read<'a, Town>,
        Read<'a, Now>,
     );

    fn run(&mut self, (entities, mut workers, mut velocities, mut animations, town, now): Self::SystemData) {
        for (e, worker, mut mov, mut anim) in (&entities, &mut workers, &mut velocities, &mut animations).join() {
            if let Some(task) = worker.poll(now.0) {
                match task.task_type {
                    TaskType::Walk => {
                        let position_now = mov.position(task.start_time);
                        let position_after = town.tile_area(task.position).pos;
                        if about_equal(position_now.x, position_after.x)
                        && about_equal(position_now.y, position_after.y)  {
                            continue;
                        }
                        let dir = position_after - position_now;
                        mov.start_ts = task.start_time;
                        mov.start_pos = position_now;
                        mov.momentum = dir.normalize() * mov.max_speed;
                        if let Some(dir) = Direction::from_vector(&mov.momentum) {
                            anim.direction = dir;
                        }
                    },
                    TaskType::Idle => {
                        mov.start_pos = mov.position(task.start_time);
                        mov.start_ts = task.start_time;
                        mov.momentum = (0.0,0.0).into();
                    }
                    TaskType::GatherSticks => {
                        // TODO: Add worker to bundling station
                        entities.delete(e).unwrap();
                    }
                    _ => {debug_assert!(false, "Unexpected task")},
                }
            }
        }
    }
}

