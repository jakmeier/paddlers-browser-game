use crate::db::DB;
use crate::StringErr;
use crate::{buildings::BuildingFactory, game_master::story_worker::StoryWorkerMessage};
use paddlers_shared_lib::story::story_trigger::StoryTrigger;
use paddlers_shared_lib::{api::shop::*, game_mechanics::attributes::Attributes, prelude::*};

impl DB {
    pub fn try_buy_building(
        &self,
        typ: BuildingType,
        pos: (usize, usize),
        village: VillageKey,
        player: &Player,
        addr: actix_web::web::Data<crate::ActorAddresses>,
    ) -> StringErr {
        self.building_has_space(typ, pos, village)
            .map(|_| self.try_spend(&typ.price(), village))
            .map(|_| {
                self.insert_building(&BuildingFactory::new(typ, pos, village));
            })
            .map(|_| {
                addr.story_worker.do_send(StoryWorkerMessage::new_verified(
                    player.key(),
                    player.story_state,
                    StoryTrigger::BuildingBuilt(typ),
                ))
            })
    }

    fn building_has_space(
        &self,
        typ: BuildingType,
        pos: (usize, usize),
        village: VillageKey,
    ) -> StringErr {
        // Check conflict with existing building
        let (w, h) = typ.size();
        debug_assert_eq!(w, 1, "Not implemented yet");
        debug_assert_eq!(h, 1, "Not implemented yet");
        let (x0, y0) = (pos.0 as usize, pos.1 as usize);
        // let(x1,y1) = (x0+w, y0+h);
        for other in self.buildings(village) {
            let typ: BuildingType = other.building_type;
            let (w, h) = typ.size();
            debug_assert_eq!(w, 1, "Not implemented yet");
            debug_assert_eq!(h, 1, "Not implemented yet");
            let (x, y) = (other.x as usize, other.y as usize);
            if x == x0 && y == y0 {
                return Err("Space occupied".to_owned());
            }
        }

        // Check conflict with map
        // Note: Cleaner handling of map shape might be necessary in the future
        if y0 == 6 {
            return Err("Cannot build here".to_owned());
        }

        // Check conflict with stationary units
        let workers = self.workers(village);
        let (x0, y0) = (pos.0 as i32, pos.1 as i32);
        for w in workers {
            if w.x == x0 && w.y == y0 {
                return Err("Unit blocks space".to_owned());
            }
        }
        // Check conflict with walking units
        let workers = self.workers_with_job(village, &[TaskType::Walk]);
        for w in workers {
            let mut worker_x = w.x;
            let mut worker_y = w.y;
            for task in self.worker_tasks(w.key()) {
                if is_between(x0, worker_x, task.x) || is_between(y0, worker_y, task.y) {
                    return Err("Walking unit blocks space".to_owned());
                }
                worker_x = task.x;
                worker_y = task.y;
            }
        }
        Ok(())
    }
    pub fn player_allowed_to_build(
        &self,
        typ: BuildingType,
        _vid: VillageKey,
        player: &Player,
    ) -> bool {
        typ.player_can_build(player.karma, player.story_state)
    }
}

fn is_between(x: i32, a: i32, b: i32) -> bool {
    (x - a) * (x - b) <= 0
}
