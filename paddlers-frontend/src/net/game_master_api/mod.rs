use super::{
    ajax, ajax::AjaxError, authentication::read_jwt_preferred_username, url::*, NetUpdateRequest,
};
use crate::prelude::*;
use futures_util::future::FutureExt;
use paddle::{Domain, NutsCheck};
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
    pub game_master_url: String,
}
pub struct UpdateRestApi;

pub struct HttpBuyProphet;
pub struct HttpCollectReward;
pub struct HttpCreatePlayer;
pub struct HttpDeleteBuilding {
    pub idx: (usize, usize),
    pub village: VillageKey,
}
pub struct HttpInvite;
pub struct HttpNotifyVisitorSatisfied {
    pub hobo: HoboKey,
}
pub struct HttpPlaceBuilding {
    pub pos: (usize, usize),
    pub building_type: BuildingType,
    pub village: VillageKey,
}

impl RestApiState {
    pub fn init() {
        let rest = RestApiState {
            queue: VecDeque::new(),
            game_master_url: game_master_url().nuts_check().unwrap_or_default(),
        };
        let rest_activity = nuts::new_domained_activity(rest, &Domain::Network);
        rest_activity.subscribe_owned(Self::http_buy_prophet);
        rest_activity.subscribe_owned(Self::http_collect_reward);
        rest_activity.subscribe_owned(Self::http_create_player);
        rest_activity.subscribe_owned(Self::http_delete_building);
        rest_activity.subscribe_owned(Self::http_invite);
        rest_activity.subscribe_owned(Self::http_notify_visitor_satisfied);
        rest_activity.subscribe_owned(Self::http_overwrite_tasks);
        rest_activity.subscribe_owned(Self::http_place_building_0);
        rest_activity.subscribe_owned(Self::http_send_attack);
        rest_activity.subscribe_owned(Self::http_send_statistics);
        rest_activity.subscribe_owned(Self::http_update_story_state);
    }
    pub fn http_place_building(
        pos: (usize, usize),
        building_type: BuildingType,
        village: VillageKey,
    ) {
        nuts::publish(HttpPlaceBuilding {
            pos,
            building_type,
            village,
        });
    }
    fn http_place_building_0(&mut self, input: HttpPlaceBuilding) {
        let msg = BuildingPurchase {
            building_type: input.building_type,
            x: input.pos.0,
            y: input.pos.1,
            village: input.village,
        };

        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/shop/building", self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_delete_building(&mut self, input: HttpDeleteBuilding) {
        let msg = BuildingDeletion {
            x: input.idx.0,
            y: input.idx.1,
            village: input.village,
        };
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/shop/building/delete", self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_buy_prophet(&mut self, msg: ProphetPurchase) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/shop/unit/prophet", self.game_master_url),
            request_string,
        );
        let afterwards = NetUpdateRequest::PlayerInfo;
        // TODO: Also update hobos afterwards, not only player info...
        self.push_promise(promise, Some(afterwards));
    }

    fn http_overwrite_tasks(&mut self, msg: TaskList) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/worker/overwriteTasks", self.game_master_url),
            request_string,
        );
        let afterwards = NetUpdateRequest::WorkerTasks(msg.worker_id.num());
        self.push_promise(promise, Some(afterwards));
    }

    pub fn http_send_statistics(&mut self, msg: FrontendRuntimeStatistics) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/stats", &self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_create_player(&mut self, _: &HttpCreatePlayer) {
        if !SENT_PLAYER_CREATION.load(std::sync::atomic::Ordering::Relaxed) {
            let display_name = read_jwt_preferred_username().unwrap_or("Unnamed Player".to_owned());
            let msg = PlayerInitData { display_name };
            let request_string = &serde_json::to_string(&msg).unwrap();
            let promise = ajax::send(
                "POST",
                &format!("{}/player/create", self.game_master_url),
                request_string,
            );
            self.push_promise(promise, Some(NetUpdateRequest::CompleteReload));
            SENT_PLAYER_CREATION.store(true, std::sync::atomic::Ordering::Relaxed)
        }
    }

    fn http_send_attack(&mut self, msg: AttackDescriptor) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/attacks/create", self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_invite(&mut self, msg: InvitationDescriptor) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/attacks/invite", self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_notify_visitor_satisfied(&mut self, msg: HttpNotifyVisitorSatisfied) {
        let request_string = &serde_json::to_string(&msg.hobo).unwrap();
        let promise = ajax::send(
            "POST",
            &format!(
                "{}/attacks/notifications/visitor_satisfied",
                &self.game_master_url
            ),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_update_story_state(&mut self, msg: StoryStateTransition) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/story/transition", &self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
    }

    fn http_collect_reward(&mut self, msg: ReportCollect) {
        let request_string = &serde_json::to_string(&msg).unwrap();
        let promise = ajax::send(
            "POST",
            &format!("{}/report/collect", &self.game_master_url),
            request_string,
        );
        self.push_promise(promise, None);
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
    pub fn poll_queue(&mut self, _: &UpdateRestApi) {
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
