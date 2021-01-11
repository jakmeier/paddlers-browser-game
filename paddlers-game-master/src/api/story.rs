//! Game master API for story state changes

use crate::db::DB;
use crate::{authentication::Authentication, game_master::story_worker::StoryWorkerMessage};
use actix_web::{web, HttpResponse, Responder};
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;
use paddlers_shared_lib::{api::story::StoryStateTransition, story::story_trigger::StoryTrigger};

pub(crate) fn story_transition(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<StoryStateTransition>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Some(player) = auth.player_object(&db) {
        db.try_execute_story_transition(player, body.0.now, addr)
            .map_or_else(|e| HttpResponse::from(&e), |_| HttpResponse::Ok().into())
    } else {
        HttpResponse::BadRequest().body(format!(
            "No such player in the database: {}",
            auth.user.uuid
        ))
    }
}

impl DB {
    fn try_execute_story_transition(
        &self,
        player: &Player,
        claimed_story_state: StoryState,
        addr: web::Data<crate::ActorAddresses>,
    ) -> Result<(), String> {
        if claimed_story_state != player.story_state {
            return Err(format!(
                "Invalid story state: {:?}, database has: {:?}",
                claimed_story_state, player.story_state
            ));
        }
        addr.story_worker.do_send(StoryWorkerMessage::new_verified(
            player.key(),
            claimed_story_state,
            StoryTrigger::DialogueStoryTrigger,
        ));
        Ok(())
    }
}
