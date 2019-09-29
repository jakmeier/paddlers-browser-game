use specs::prelude::*;
use crate::game::{
    Now,
    movement::Moving,
    units::workers::*,
    town::Town,
    components::*,
    abilities::use_welcome_ability,
};
use crate::gui::animation::*;
use crate::gui::utils::*;
use crate::gui::gui_components::ClickOutput;
use crate::gui::render::Renderable;
use crate::prelude::*;
use crate::logging::ErrorQueue;
use quicksilver::geom::about_equal;

pub struct WorkerSystem;

impl<'a> System<'a> for WorkerSystem {
    type SystemData = (
        Entities<'a>,    
        Read<'a, LazyUpdate>,
        WriteStorage<'a, Worker>,
        WriteStorage<'a, Moving>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, StatusEffects>,
        WriteStorage<'a, AnimationState>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, UiMenu>,
        WriteStorage<'a, Mana>,
        ReadStorage<'a, Renderable>,
        Write<'a, Town>,
        Write<'a, ErrorQueue>,
        Read<'a, Now>,
     );

    fn run(&mut self, (
        entities,
        lazy,
        mut workers,
        mut velocities,
        mut health,
        mut status_effects,
        mut animations,
        mut container,
        mut ui_menus,
        mut mana,
        rend,
        mut town,
        mut errq,
        now
    ): Self::SystemData) {
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
                        anim.direction = Direction::from_vector(&mov.momentum);
                    },
                    TaskType::Idle => {
                        mov.stand_still(task.start_time);
                        anim.direction = Direction::Undirected;
                    }
                    TaskType::GatherSticks 
                    | TaskType::ChopTree 
                    => {
                        mov.stand_still(task.start_time);
                        anim.direction = Direction::Undirected;
                        move_worker_into_building(&mut container, &mut ui_menus, &mut town, &lazy, &rend, e, task.position);
                    },
                    TaskType::WelcomeAbility => {
                        mov.stand_still(task.start_time);
                        anim.direction = Direction::Undirected;
                        let err = use_welcome_ability(
                            e,
                            task.target.expect("Welcoming required target"),
                            &mut health,
                            &mut status_effects,
                            &mut mana,
                        );
                        if let Err(e) = err {
                            errq.push(e);
                        } else {
                            let ui = ui_menus.get_mut(e).expect("Ui menu vanished");
                            update_cooldown(&mut *ui, AbilityType::Welcome, now.0);
                        }
                    }
                    _ => {debug_assert!(false, "Unexpected task")},
                }
            }
        }
    }
}

fn update_cooldown(ui: &mut UiMenu, ability: AbilityType, now: Timestamp) {
    let click = ClickOutput::Ability(ability);
    if let Some(el) = ui.ui.find_by_on_click(click) {
        el.overlay = Some((now, now + ability.cooldown().num_microseconds().unwrap()));
    }
}

