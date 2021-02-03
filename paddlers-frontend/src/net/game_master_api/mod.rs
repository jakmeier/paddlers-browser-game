use super::{ajax, authentication::keycloak_preferred_name, url::*, RequestHobos, RequestQuests};
use crate::{
    game::{components::NetObj, game_event_manager::game_event},
    prelude::*,
};
use paddle::{Domain, NutsCheck};
use paddlers_shared_lib::api::{
    attacks::*, keys::*, shop::*, statistics::*, tasks::TaskList, PlayerInitData,
};
use paddlers_shared_lib::api::{hobo::SettleHobo, story::StoryStateTransition};
use paddlers_shared_lib::api::{quests::QuestCollect, reports::ReportCollect};
use specs::Entity;
use std::sync::atomic::AtomicBool;

static SENT_PLAYER_CREATION: AtomicBool = AtomicBool::new(false);

// TODO: Decide to use this new pattern for all GM message, or don't use.
/// Message to backend GM
#[derive(Debug, PartialEq, Clone)]
pub enum GameMasterMessage {
    UpgradeBuilding(HttpUpgradeBuilding),
}
#[derive(Debug, PartialEq, Clone)]
pub enum GameMasterResponse {
    /// Only works on home town buildings
    NewBuildingLevel(Entity, usize),
}

pub struct RestApiState {
    pub game_master_url: String,
}
pub struct HttpCreatePlayer;
pub struct HttpDeleteBuilding {
    pub idx: (usize, usize),
    pub village: VillageKey,
}
#[derive(Debug, PartialEq, Clone)]
pub struct HttpUpgradeBuilding {
    pub entity: Entity,
    pub building: BuildingKey,
    pub current_level: usize,
}
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
            game_master_url: game_master_url().nuts_check().unwrap_or_default(),
        };
        let rest_activity = nuts::new_domained_activity(rest, &Domain::Network);
        rest_activity.private_channel(Self::http_buy_prophet);
        rest_activity.private_channel(Self::http_collect_reward);
        rest_activity.private_channel(Self::http_collect_quest);
        rest_activity.private_channel(Self::http_create_player);
        rest_activity.private_channel(Self::http_delete_building);
        rest_activity.private_channel(Self::http_upgrade_building);
        rest_activity.private_channel(Self::http_invite);
        rest_activity.private_channel(Self::http_let_visitor_in);
        rest_activity.private_channel(Self::http_notify_visitor_satisfied);
        rest_activity.private_channel(Self::http_overwrite_tasks);
        rest_activity.private_channel(Self::http_place_building_0);
        rest_activity.private_channel(Self::http_send_attack);
        rest_activity.private_channel(Self::http_send_statistics);
        rest_activity.private_channel(Self::http_update_story_state);
        rest_activity.private_channel(Self::http_settle_hobo);
    }
    pub fn http_place_building(
        pos: (usize, usize),
        building_type: BuildingType,
        village: VillageKey,
        entity: Entity,
    ) {
        nuts::send_to::<RestApiState, _>((
            entity,
            HttpPlaceBuilding {
                pos,
                building_type,
                village,
            },
        ));
    }
    fn http_place_building_0(&mut self, (entity, input): (Entity, HttpPlaceBuilding)) {
        let msg = BuildingPurchase {
            building_type: input.building_type,
            x: input.pos.0,
            y: input.pos.1,
            village: input.village,
        };
        let uri = self.game_master_url.clone() + "/shop/building";
        let future = async move {
            let id: i64 = ajax::fetch_json("POST", &uri, &msg).await?;
            let net_obj = NetObj::building(id);
            game_event(GameEvent::NetObjId(entity, net_obj));
            Ok(())
        };
        spawn_future(future);
    }

    fn http_delete_building(&mut self, input: HttpDeleteBuilding) {
        let msg = BuildingDeletion {
            x: input.idx.0,
            y: input.idx.1,
            village: input.village,
        };
        let future = ajax::fetch_json(
            "POST",
            &format!("{}/shop/building/delete", self.game_master_url),
            &msg,
        );
        spawn_future(future);
    }

    fn http_upgrade_building(&mut self, input: HttpUpgradeBuilding) {
        let uri = self.game_master_url.clone() + "/shop/building/upgrade";
        let msg = BuildingUpgrade {
            building: input.building,
            current_level: input.current_level,
        };
        let future = async move {
            ajax::fetch_empty_response("POST", &uri, &msg).await?;
            game_event(GameEvent::GameMasterResponse(
                GameMasterResponse::NewBuildingLevel(input.entity, input.current_level + 1),
            ));
            Ok(())
        };
        spawn_future(future);
    }

    fn http_buy_prophet(&mut self, msg: ProphetPurchase) {
        let uri = self.game_master_url.clone() + "/shop/unit/prophet";
        let future = async move {
            ajax::fetch_json("POST", &uri, &msg).await?;
            crate::net::request_player_update();
            // TODO: Also update hobos afterwards, not only player info...
            Ok(())
        };
        spawn_future(future);
    }

    fn http_overwrite_tasks(&mut self, msg: TaskList) {
        let uri = self.game_master_url.clone() + "/worker/overwriteTasks";
        let future = async move {
            ajax::fetch_empty_response("POST", &uri, &msg).await?;
            crate::net::request_worker_tasks_update(msg.worker_id.num());
            Ok(())
        };
        spawn_future(future);
    }

    pub fn http_send_statistics(&mut self, msg: FrontendRuntimeStatistics) {
        let future = ajax::fetch_json("POST", &format!("{}/stats", &self.game_master_url), &msg);
        spawn_future(future);
    }

    fn http_create_player(&mut self, _: HttpCreatePlayer) {
        if !SENT_PLAYER_CREATION.load(std::sync::atomic::Ordering::Relaxed) {
            let display_name = keycloak_preferred_name().unwrap_or("Unnamed Player".to_owned());
            let uri = self.game_master_url.clone() + "/player/create";
            let msg = PlayerInitData { display_name };
            let future = async move {
                ajax::fetch_empty_response("POST", &uri, &msg).await?;
                crate::net::request_client_state();
                Ok(())
            };
            spawn_future(future);
            SENT_PLAYER_CREATION.store(true, std::sync::atomic::Ordering::Relaxed)
        }
    }

    fn http_send_attack(&mut self, msg: AttackDescriptor) {
        let future = ajax::fetch_json(
            "POST",
            &format!("{}/attacks/create", self.game_master_url),
            &msg,
        );
        spawn_future(future);
    }

    fn http_invite(&mut self, msg: InvitationDescriptor) {
        let future = ajax::fetch_json(
            "POST",
            &format!("{}/attacks/invite", self.game_master_url),
            &msg,
        );
        spawn_future(future);
    }

    fn http_let_visitor_in(&mut self, msg: StartFightRequest) {
        let future = ajax::fetch_empty_response(
            "POST",
            &format!("{}/attacks/startFight", self.game_master_url),
            &msg,
        );
        spawn_future(future);
    }

    fn http_notify_visitor_satisfied(&mut self, msg: HttpNotifyVisitorSatisfied) {
        let future = ajax::fetch_json(
            "POST",
            &format!(
                "{}/attacks/notifications/visitor_satisfied",
                &self.game_master_url
            ),
            &msg.hobo,
        );
        spawn_future(future);
    }

    fn http_update_story_state(&mut self, msg: StoryStateTransition) {
        let future = ajax::fetch_empty_response(
            "POST",
            &format!("{}/story/transition", &self.game_master_url),
            &msg,
        );
        spawn_future(future);
    }

    fn http_collect_reward(&mut self, msg: ReportCollect) {
        let future = ajax::fetch_json(
            "POST",
            &format!("{}/report/collect", &self.game_master_url),
            &msg,
        );
        spawn_future(future);
    }

    fn http_collect_quest(&mut self, msg: QuestCollect) {
        let uri = format!("{}/quest/collect", &self.game_master_url);
        let future = async move {
            ajax::fetch_empty_response("POST", &uri, &msg).await?;
            nuts::publish(RequestQuests);
            Ok(())
        };
        spawn_future(future);
    }
    fn http_settle_hobo(&mut self, msg: SettleHobo) {
        let uri = format!("{}/hobo/settle", &self.game_master_url);
        let future = async move {
            ajax::fetch_empty_response("POST", &uri, &msg).await?;
            nuts::publish(RequestHobos);
            Ok(())
        };
        spawn_future(future);
    }
}

fn spawn_future(future: impl std::future::Future<Output = PadlResult<()>> + 'static) {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(err) = future.await {
            nuts::publish(err);
        }
    });
}
