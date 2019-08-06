use std::collections::VecDeque;
use std::sync::{Mutex,mpsc::Sender};
use paddlers_shared_lib::api::shop::*;
use paddlers_shared_lib::api::tasks::TaskList;
use crate::prelude::*;
use crate::logging::AsyncErr;
use super::ajax;
use super::{SHOP_PATH, WORKER_PATH, NetUpdateRequest};
use specs::prelude::*;
use futures_util::future::FutureExt;
use stdweb::PromiseFuture;

pub struct RestApiState {
    pub queue: VecDeque<(stdweb::PromiseFuture<std::string::String>, Option<NetUpdateRequest>)>,
    err_chan: Mutex<Sender<PadlError>>,
}

impl RestApiState {
    pub fn new(err_chan: Sender<PadlError>) -> Self {
        RestApiState {
            queue: VecDeque::new(),
            err_chan: Mutex::new(err_chan),
        }
    }
    pub fn http_place_building(&mut self, pos: (usize, usize), building_type: BuildingType) {
        let msg = BuildingPurchase {
            building_type: building_type, 
            x: pos.0,
            y: pos.1,
        };

        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/building", SHOP_PATH), request_string);
        self.push_promise(promise, None);
    }

    pub fn http_delete_building(&mut self, idx: (usize, usize)) {
        let msg = BuildingDeletion { x: idx.0, y: idx.1 };
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/building/delete", SHOP_PATH), request_string);
        self.push_promise(promise, None);
    }

    pub fn http_overwrite_tasks(&mut self, msg: TaskList) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/overwriteTasks", WORKER_PATH), request_string);
        let afterwards = NetUpdateRequest::UnitTasks(msg.unit_id);
        self.push_promise(promise, Some(afterwards));
    }

    fn push_promise(
        &mut self, 
        maybe_promise: PadlResult<PromiseFuture<String>>, 
        afterwards: Option<NetUpdateRequest>,

    ) {
        match maybe_promise {
            Ok(promise) =>
                self.queue.push_back((promise, afterwards)),
            Err(e) => 
                self.err_chan.lock().expect("Lock on err mpsc")
                    .send(e).expect("Sending error over mpsc failed"),
        }
    }
}

pub struct RestApiSystem;
impl<'a> System<'a> for RestApiSystem {
    type SystemData = (
        WriteExpect<'a, RestApiState>,
        ReadExpect<'a, AsyncErr>,
     );

    fn run(&mut self, (mut state, error): Self::SystemData) {
        while let Some((promise, afterwards)) = (*state).queue.pop_front() {
            let error_chan = error.clone_sender();
            stdweb::spawn_local(
                promise.map(
                    move |r| {
                        if r.is_err() {
                            let err: PadlResult<()> = 
                            PadlErrorCode::RestAPI(
                                format!("Rest API Error: {}", r.unwrap_err())
                            ).dev();
                            error_chan.send(err.unwrap_err()).expect("sending over mpsc");
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
