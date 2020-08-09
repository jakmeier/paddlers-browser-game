use super::{
    ajax, ajax::AjaxError, authentication::read_jwt_preferred_username, url::*, NetUpdateRequest,
};
use crate::prelude::*;
use futures_util::future::FutureExt;
use paddlers_shared_lib::api::reports::ReportCollect;
use paddlers_shared_lib::api::story::StoryStateTransition;
use paddlers_shared_lib::api::{
    attacks::*, keys::*, shop::*, statistics::*, tasks::TaskList, PlayerInitData,
};
use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use stdweb::PromiseFuture;

static SENT_PLAYER_CREATION: AtomicBool = AtomicBool::new(false);

pub struct RestApiState {
    pub queue: VecDeque<(
        stdweb::PromiseFuture<std::string::String, AjaxError>,
        Option<NetUpdateRequest>,
    )>,
}

impl RestApiState {
    pub fn new() -> Self {
        RestApiState {
            queue: VecDeque::new(),
        }
    }
    /// Get the global instance
    pub fn get<'a>() -> std::sync::MutexGuard<'a, Self> {
        unsafe {
            super::STATIC_NET_STATE
                .rest
                .as_ref()
                .expect("Tried to load REST API state before initialization")
                .lock()
                .unwrap()
        }
    }
    pub fn http_place_building(
        &mut self,
        pos: (usize, usize),
        building_type: BuildingType,
        village: VillageKey,
    ) -> PadlResult<()> {
        let msg = BuildingPurchase {
            building_type: building_type,
            x: pos.0,
            y: pos.1,
            village,
        };

        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/shop/building", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_delete_building(
        &mut self,
        idx: (usize, usize),
        village: VillageKey,
    ) -> PadlResult<()> {
        let msg = BuildingDeletion {
            x: idx.0,
            y: idx.1,
            village,
        };
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/shop/building/delete", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_buy_prophet(&mut self, msg: ProphetPurchase) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/shop/unit/prophet", game_master_url()?),
            request_string,
        );
        let afterwards = NetUpdateRequest::PlayerInfo;
        // TODO: Also update hobos afterwards, not only player info...
        self.push_promise(promise, Some(afterwards));
        Ok(())
    }

    pub fn http_overwrite_tasks(&mut self, msg: TaskList) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/worker/overwriteTasks", game_master_url()?),
            request_string,
        );
        let afterwards = NetUpdateRequest::WorkerTasks(msg.worker_id.num());
        self.push_promise(promise, Some(afterwards));
        Ok(())
    }

    pub fn http_send_statistics(&mut self, msg: FrontendRuntimeStatistics) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/stats", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_create_player(&mut self) -> PadlResult<()> {
        if !SENT_PLAYER_CREATION.load(std::sync::atomic::Ordering::Relaxed) {
            let display_name = read_jwt_preferred_username().unwrap_or("Unnamed Player".to_owned());
            let msg = PlayerInitData { display_name };
            let request_string = &serde_json::to_string(&msg).unwrap();
            let promise = ajax::send(
                "POST",
                &format!("{}/player/create", game_master_url()?),
                request_string,
            );
            self.push_promise(promise, Some(NetUpdateRequest::CompleteReload));
            SENT_PLAYER_CREATION.store(true, std::sync::atomic::Ordering::Relaxed)
        }
        Ok(())
    }

    pub fn http_send_attack(&mut self, msg: AttackDescriptor) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/attacks/create", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_invite(&mut self, msg: InvitationDescriptor) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/attacks/invite", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_notify_visitor_satisfied(&mut self, msg: HoboKey) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!(
                "{}/attacks/notifications/visitor_satisfied",
                game_master_url()?
            ),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_update_story_state(&mut self, msg: StoryStateTransition) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/story/transition", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    pub fn http_collect_reward(&mut self, msg: ReportCollect) -> PadlResult<()> {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/report/collect", game_master_url()?),
            request_string,
        );
        self.push_promise(promise, None);
        Ok(())
    }

    fn push_promise(
        &mut self,
        maybe_promise: PadlResult<PromiseFuture<String, AjaxError>>,
        afterwards: Option<NetUpdateRequest>,
    ) {
        match maybe_promise {
            Ok(promise) => self.queue.push_back((promise, afterwards)),
            Err(e) => nuts::publish(e),
        }
    }
    pub fn poll_queue(&mut self) {
        while let Some((promise, afterwards)) = self.queue.pop_front() {
            stdweb::spawn_local(promise.map(move |r| {
                if r.is_err() {
                    let err: PadlResult<()> =
                        PadlErrorCode::RestAPI(format!("Rest API Error: {:?}", r.unwrap_err()))
                            .dev();
                    nuts::publish(err.unwrap_err());
                } else {
                    if let Some(req) = afterwards {
                        match req {
                            NetUpdateRequest::WorkerTasks(unit_id) => {
                                crate::net::request_worker_tasks_update(unit_id)
                            }
                            NetUpdateRequest::CompleteReload => crate::net::request_client_state(),
                            NetUpdateRequest::PlayerInfo => crate::net::request_player_update(),
                        }
                    }
                }
            }));
        }
    }
}

pub struct RestApiSystem;
impl<'a> System<'a> for RestApiSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        RestApiState::get().poll_queue();
    }
}
