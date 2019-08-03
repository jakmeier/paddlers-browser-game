use std::collections::VecDeque;
use paddlers_shared_lib::api::shop::*;
use paddlers_shared_lib::api::tasks::TaskList;
use crate::prelude::*;
use super::ajax;
use super::{SHOP_PATH, WORKER_PATH, NetUpdateRequest};
use specs::prelude::*;
use futures_util::future::FutureExt;

#[derive(Default)]
pub struct RestApiState {
    pub queue: VecDeque<(stdweb::PromiseFuture<std::string::String>, Option<NetUpdateRequest>)>,
}

impl RestApiState {
    pub fn http_place_building(&mut self, pos: (usize, usize), building_type: BuildingType) {
        let msg = BuildingPurchase {
            building_type: building_type, 
            x: pos.0,
            y: pos.1,
        };

        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/building", SHOP_PATH), request_string);
        self.queue.push_back((promise, None));
    }

    pub fn http_delete_building(&mut self, idx: (usize, usize)) {
        let msg = BuildingDeletion { x: idx.0, y: idx.1 };
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/building/delete", SHOP_PATH), request_string);
        self.queue.push_back((promise, None));
    }

    pub fn http_overwrite_tasks(&mut self, msg: TaskList) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/overwriteTasks", WORKER_PATH), request_string);
        let afterwards = NetUpdateRequest::UnitTasks(msg.unit_id);
        self.queue.push_back((promise, Some(afterwards)));
    }
}

pub struct RestApiSystem;
impl<'a> System<'a> for RestApiSystem {
    type SystemData = (
        Write<'a, RestApiState>,
     );

    fn run(&mut self, mut state: Self::SystemData) {
        while let Some((promise, afterwards)) = (*state.0).queue.pop_front() {
            stdweb::spawn_local(
                promise.map(
                    |r| {
                        if r.is_err() {
                            println!("Rest API Error: {:?}", r);
                        }
                        else {
                            if let Some(req) = afterwards {
                                match req {
                                    NetUpdateRequest::UnitTasks(unit_id) 
                                        =>  crate::net::request_unit_tasks_update(unit_id),
                                }
                            }
                        }
                    }
                )
            );
        }
    }
}
