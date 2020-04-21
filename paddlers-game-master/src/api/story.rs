//! Game master API for story state changes

use crate::authentication::Authentication;
use crate::db::DB;
use actix_web::{web, HttpResponse, Responder};
use paddlers_shared_lib::api::story::StoryStateTransition;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;

pub fn story_transition(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<StoryStateTransition>,
    mut auth: Authentication,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Some(player) = auth.player_object(&db) {
        db.try_execute_story_transition(player, body.0.before, body.0.after)
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
        before: StoryState,
        after: StoryState,
    ) -> Result<Player, String> {
        if before != player.story_state {
            return Err(format!(
                "Invalid story state: {:?}, database has: {:?}",
                before, player.story_state
            ));
        }
        // TODO: Check that transition is valid
        // TODO: In the future, also execute a defined list of server actions for the story transition
        self.update_story_state(player.key(), after)
            .map_err(|e| e.to_string())
    }
}
