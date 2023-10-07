use crate::game_master::story_worker::StoryWorkerMessage;
use crate::{authentication::Authentication, db::CollectQuestMessage};
use actix_web::{web, HttpResponse};
use paddlers_shared_lib::{
    api::quests::QuestCollect,
    prelude::{Player, Quest, QuestKey},
    story::story_trigger::StoryTrigger,
};
use paddlers_shared_lib::{
    keys::SqlKey,
    prelude::{GameDB, VillageKey},
};

pub(crate) async fn collect_quest(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<QuestCollect>,
    auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> HttpResponse {
    let result = collect_quest_impl(pool, body, auth, addr).await;
    match result {
        Err(msg) => HttpResponse::Forbidden().body(msg).into(),
        Ok(()) => HttpResponse::Ok().into(),
    }
}
pub(crate) async fn collect_quest_impl(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<QuestCollect>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> Result<(), String> {
    // Check that quest is active and all conditions are met, then forward request to DB actor
    let db: crate::db::DB = pool.get_ref().into();
    let player_key = auth.player_key(&db)?;
    let quest_key = body.quest;
    let quest = db
        .player_quests(player_key)
        .into_iter()
        .find(|q| q.key() == quest_key)
        .ok_or_else(|| "Player does not have this quest".to_string())?;
    let village = db.player_villages(player_key)[0]; // Assuming one village per player
    let player = auth
        .player_object(&db)
        .ok_or_else(|| "No player?".to_owned())?;

    // TODO (performance) avoid sequential DB lookups throughout checks
    check_building_conditions(&db, quest_key, village.key())?;
    check_resource_conditions(&db, quest_key, village.key())?;
    check_karma_conditions(&quest, &player)?;
    check_pop_conditions(&db, &quest, village.key())?;
    check_worker_conditions(&db, quest_key, village.key())?;

    let follow_up_quest = quest.follow_up_quest.map(|name| {
        db.quest_by_name(
            name.parse()
                .expect("Couldn't parse QuestName from value found in DB"),
        )
        .map(|quest| quest.key())
        .expect("Quest not found in DB")
    });

    let msg = CollectQuestMessage {
        quest: quest_key,
        player: player_key,
        village: village.key(),
        follow_up_quest,
    };
    addr.db_actor
        .send(msg)
        .await
        .map_err(|e| format!("Quest collection spawn failed: {:?}", e))?;

    let quest_id = quest
        .quest_key
        .parse()
        .expect("Couldn't parse QuestName from value found in DB");
    let msg = StoryWorkerMessage::new_verified(
        player_key,
        player.story_state,
        StoryTrigger::FinishedQuest(quest_id),
    );
    addr.story_worker
        .send(msg)
        .await
        .map_err(|e| format!("Quest finished spawn failed: {:?}", e))?;

    Ok(())
}

fn check_building_conditions(
    db: &crate::db::DB,
    quest_key: QuestKey,
    village: VillageKey,
) -> Result<(), std::string::String> {
    let building_conditions = db.quest_building_conditions(quest_key);
    if building_conditions.len() > 0 {
        let buildings = db.buildings(village);
        for condition in building_conditions {
            let mut n = condition.amount;
            for building in &buildings {
                if building.building_type == condition.building_type {
                    n -= 1;
                }
            }
            if n > 0 {
                return Err("Missing ".to_owned() + &condition.building_type.to_string());
            }
        }
    }
    Ok(())
}

fn check_worker_conditions(
    db: &crate::db::DB,
    quest_key: QuestKey,
    village_key: VillageKey,
) -> Result<(), std::string::String> {
    let worker_conditions = db.quest_worker_conditions(quest_key);
    if worker_conditions.len() > 0 {
        for condition in worker_conditions {
            if (db
                .workers_with_job(village_key, &[condition.task_type])
                .len() as i64)
                < condition.amount
            {
                return Err("Missing ".to_owned() + &condition.task_type.to_string() + " workers");
            }
        }
    }
    Ok(())
}

fn check_resource_conditions(
    db: &crate::db::DB,
    quest_key: QuestKey,
    village_key: VillageKey,
) -> Result<(), std::string::String> {
    let res_conditions = db.quest_res_conditions(quest_key);
    if res_conditions.len() > 0 {
        for condition in res_conditions {
            if db.resource(condition.resource_type, village_key) < condition.amount {
                return Err("Missing ".to_owned() + &condition.resource_type.to_string());
            }
        }
    }
    Ok(())
}

fn check_karma_conditions(quest: &Quest, player: &Player) -> Result<(), std::string::String> {
    if let Some(karma_required) = quest.karma_condition {
        if player.karma < karma_required {
            return Err("Need more Karma".to_owned());
        }
    }
    Ok(())
}

fn check_pop_conditions(
    db: &crate::db::DB,
    quest: &Quest,
    village_key: VillageKey,
) -> Result<(), std::string::String> {
    if let Some(pop_required) = quest.pop_condition {
        let pop = db.settled_hobo_count(village_key) + db.worker_count(village_key);
        if pop < pop_required {
            return Err("Need more Population".to_owned());
        }
    }
    Ok(())
}
