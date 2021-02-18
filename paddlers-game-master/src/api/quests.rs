use crate::game_master::story_worker::StoryWorkerMessage;
use crate::{authentication::Authentication, db::CollectQuestMessage};
use actix::prelude::*;
use actix_web::error::BlockingError;
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

pub(crate) fn collect_quest(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<QuestCollect>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    web::block(move || {
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
        let future = addr
            .db_actor
            .send(msg)
            .map_err(|e| eprintln!("Quest collection spawn failed: {:?}", e));
        Arbiter::spawn(future);

        let quest_id = quest
            .quest_key
            .parse()
            .expect("Couldn't parse QuestName from value found in DB");
        let msg = StoryWorkerMessage::new_verified(
            player_key,
            player.story_state,
            StoryTrigger::FinishedQuest(quest_id),
        );
        let future = addr
            .story_worker
            .send(msg)
            .map_err(|e| eprintln!("Quest finished spawn failed: {:?}", e));
        Arbiter::spawn(future);

        Ok(())
    })
    .then(
        |result: Result<(), BlockingError<std::string::String>>| match result {
            Err(BlockingError::Error(msg)) => Ok(HttpResponse::Forbidden().body(msg).into()),
            Err(BlockingError::Canceled) => Ok(HttpResponse::InternalServerError().into()),
            Ok(()) => Ok(HttpResponse::Ok().into()),
        },
    )
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
        if karma_required < player.karma {
            return Err("Need more Karma".to_owned());
        }
    }
    Ok(())
}
