use crate::game::game_event_manager::EventPool;
use crate::game::{
    abilities::use_welcome_ability,
    components::*,
    movement::Moving,
    town::{TileIndex, Town},
    units::workers::*,
    Now,
};
use crate::gui::animation::*;
use crate::gui::gui_components::ClickOutput;
use crate::gui::render::Renderable;
use crate::gui::utils::*;
use crate::logging::ErrorQueue;
use crate::prelude::*;
use quicksilver::geom::about_equal;
use specs::prelude::*;

pub struct WorkerSystem {
    event_pool: EventPool,
}
impl WorkerSystem {
    pub fn new(event_pool: EventPool) -> Self {
        WorkerSystem { event_pool }
    }
}

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
        WriteStorage<'a, Level>,
        ReadStorage<'a, Renderable>,
        WriteExpect<'a, Town>,
        WriteExpect<'a, ErrorQueue>,
        ReadExpect<'a, Now>,
    );

    fn run(
        &mut self,
        (
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
            mut levels,
            rend,
            mut town,
            mut errq,
            now,
        ): Self::SystemData,
    ) {
        for (e, worker, mut mov, mut anim) in
            (&entities, &mut workers, &mut velocities, &mut animations).join()
        {
            if let Some(task) = worker.poll(now.0) {
                match task.task_type {
                    TaskType::Walk => {
                        let position_now = mov.position(task.start_time);
                        let position_after = town.resolution.tile_area(task.position).pos;
                        if about_equal(position_now.x, position_after.x)
                            && about_equal(position_now.y, position_after.y)
                        {
                            continue;
                        }
                        let dir = position_after - position_now;
                        mov.start_ts = task.start_time;
                        mov.start_pos = position_now;
                        mov.momentum = dir.normalize() * mov.max_speed;
                        anim.direction = Direction::from_vector(&mov.momentum);
                    }
                    TaskType::Idle => {
                        mov.stand_still(task.start_time);
                        anim.direction = Direction::Undirected;
                    }
                    TaskType::GatherSticks | TaskType::ChopTree => {
                        mov.stand_still(task.start_time);
                        anim.direction = Direction::Undirected;
                        move_worker_into_building(
                            &mut container,
                            &mut ui_menus,
                            &mut town,
                            &lazy,
                            &rend,
                            e,
                            task.position,
                        );
                    }
                    TaskType::WelcomeAbility => {
                        mov.stand_still(task.start_time);
                        anim.direction = Direction::Undirected;
                        let err = use_welcome_ability(
                            e,
                            task.target.expect("Welcoming required target"),
                            &mut health,
                            &mut status_effects,
                            &mut mana,
                            &self.event_pool,
                        );
                        if let Err(e) = err {
                            errq.push(e);
                        } else {
                            let ui = ui_menus.get_mut(e).expect("Ui menu vanished");
                            update_cooldown(&mut *ui, AbilityType::Welcome, now.0);
                        }
                    }
                    TaskType::CollectReward => {
                        mov.stand_still(task.start_time);
                        let worker_exp = levels.get_mut(e);
                        let e = collect_reward(&mut town, task.position, &entities, worker_exp);
                        if let Err(e) = e {
                            errq.push(e)
                        }
                    }
                    _ => debug_assert!(false, "Unexpected task"),
                }
            }
        }
    }
}

fn update_cooldown(ui: &mut UiMenu, ability: AbilityType, now: Timestamp) {
    let click = ClickOutput::Ability(ability);
    if let Some(el) = ui.ui.find_by_on_click(click) {
        el.overlay = Some((now, now + ability.cooldown()));
    }
}

fn collect_reward(
    town: &mut Town,
    position: TileIndex,
    entities: &Entities,
    level: Option<&mut Level>,
) -> PadlResult<()> {
    let bt = town.building_type(position)?;
    let collected_building = town.remove_building(position);
    if let Err(_e) = entities.delete(collected_building) {
        return PadlErrorCode::EcsError("Deleting collected reward building").dev();
    }
    let level = level.ok_or(PadlError::dev_err(PadlErrorCode::EcsError(
        "No experience pool given to add to",
    )))?;
    if let Some(reward) = bt.reward_exp() {
        level.add_exp(reward);
    }
    Ok(())
    // TODO: Level up
}
