use std::collections::VecDeque;
use std::sync::{Mutex,mpsc::Sender, atomic::AtomicBool};
use crate::prelude::*;
use crate::logging::AsyncErr;
use super::{ajax, ajax::AjaxError, url::*, NetUpdateRequest, authentication::read_jwt_preferred_username};
use specs::prelude::*;
use futures_util::future::FutureExt;
use stdweb::PromiseFuture;
use paddlers_shared_lib::api::{
    keys::VillageKey,
    shop::*,
    tasks::TaskList,
    statistics::*,
    PlayerInitData,
    attacks::*,
};

static SENT_PLAYER_CREATION: AtomicBool = AtomicBool::new(false);

pub struct RestApiState {
    pub queue: VecDeque<(stdweb::PromiseFuture<std::string::String, AjaxError>, Option<NetUpdateRequest>)>,
    err_chan: Mutex<Sender<PadlError>>,
}

impl RestApiState {
    pub fn new(err_chan: Sender<PadlError>) -> Self {
        RestApiState {
            queue: VecDeque::new(),
            err_chan: Mutex::new(err_chan),
        }
    }
    pub fn http_place_building(&mut self, pos: (usize, usize), building_type: BuildingType, village: VillageKey) -> PadlResult<()> {
        let msg = BuildingPurchase {
            building_type: building_type, 
            x: pos.0,
            y: pos.1,
            village
        };

        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/shop/building", game_master_url()?), request_string);
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_delete_building(&mut self, idx: (usize, usize), village: VillageKey) -> PadlResult<()>  {
        let msg = BuildingDeletion { x: idx.0, y: idx.1, village };
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/shop/building/delete", game_master_url()?), request_string);
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_buy_prophet(&mut self, msg: ProphetPurchase) -> PadlResult<()>  {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/shop/unit/prophet", game_master_url()?), request_string);
        let afterwards = NetUpdateRequest::PlayerInfo;
        self.push_promise(promise, Some(afterwards));
        Ok(())
    }

    pub fn http_overwrite_tasks(&mut self, msg: TaskList) -> PadlResult<()>  {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/worker/overwriteTasks", game_master_url()?), request_string);
        let afterwards = NetUpdateRequest::WorkerTasks(msg.worker_id.num());
        self.push_promise(promise, Some(afterwards));
        Ok(())
    }

    pub fn http_send_statistics(&mut self, msg: FrontendRuntimeStatistics) -> PadlResult<()>  {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/stats", game_master_url()?), request_string);
        self.push_promise(promise, None);
        Ok(())
    }


    pub fn http_create_player(&mut self) -> PadlResult<()>  {
        if !SENT_PLAYER_CREATION.load(std::sync::atomic::Ordering::Relaxed) {
            let display_name = read_jwt_preferred_username().unwrap_or("Unnamed Player".to_owned());
            let msg = PlayerInitData { display_name };
            let request_string = &serde_json::to_string(&msg).unwrap();
            let promise = ajax::send("POST", &format!("{}/player/create", game_master_url()?), request_string);
            self.push_promise(promise, Some(NetUpdateRequest::CompleteReload));
            SENT_PLAYER_CREATION.store(true, std::sync::atomic::Ordering::Relaxed)
        }
        Ok(())
    }

    pub fn http_send_attack(&mut self, msg: AttackDescriptor) -> PadlResult<()>  {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send("POST", &format!("{}/attacks/create", game_master_url()?), request_string);
        self.push_promise(promise, None);
        Ok(())
    }

    fn push_promise(
        &mut self, 
        maybe_promise: PadlResult<PromiseFuture<String, AjaxError>>, 
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
                                format!("Rest API Error: {:?}", r.unwrap_err())
                            ).dev();
                            error_chan.send(err.unwrap_err()).expect("sending over mpsc");
                        }
                        else {
                            if let Some(req) = afterwards {
                                match req {
                                    NetUpdateRequest::WorkerTasks(unit_id) 
                                        =>  crate::net::request_worker_tasks_update(unit_id),
                                    NetUpdateRequest::CompleteReload 
                                        => crate::net::request_client_state(),
                                    NetUpdateRequest::PlayerInfo
                                        => crate::net::request_player_update(),
                                }
                            }
                        }
                    }
                )
            );
        }
    }
}
