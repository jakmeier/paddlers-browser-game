//! Game master API for story state changes

use crate::authentication::Authentication;
use crate::db::NewHoboMessage;
use crate::db::DB;
use crate::game_master::attack_funnel::PlannedAttack;
use actix::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use paddlers_shared_lib::api::story::StoryStateTransition;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;

pub(crate) fn story_transition(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<StoryStateTransition>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> impl Responder {
    let db: crate::db::DB = pool.get_ref().into();
    if let Some(player) = auth.player_object(&db) {
        db.try_execute_story_transition(player, body.0.before, body.0.after, addr)
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
        addr: web::Data<crate::ActorAddresses>,
    ) -> Result<(), String> {
        if before != player.story_state {
            return Err(format!(
                "Invalid story state: {:?}, database has: {:?}",
                before, player.story_state
            ));
        }
        // TODO: Check that transition is valid

        self.update_story_state(player.key(), after, addr)
    }

    pub fn update_story_state(
        &self,
        p: PlayerKey,
        new_story_state: StoryState,
        addr: web::Data<crate::ActorAddresses>,
    ) -> Result<(), String> {
        self.set_story_state(p, new_story_state)
            .map_err(|e| e.to_string())?;
        self.perform_story_actions(new_story_state, addr, p);
        Ok(())
    }

    // TODO [0.1.5]: External specification
    fn perform_story_actions(
        &self,
        new_state: StoryState,
        addr: web::Data<crate::ActorAddresses>,
        player: PlayerKey,
    ) {
        println!("Player changes to {:?}", new_state);
        match new_state {
            StoryState::TempleBuilt => {
                // Send first attacker
                let village = self
                    .player_villages(player)
                    .pop()
                    .expect("player must have at least one village");
                // TODO: This should be something like a prefab
                let hobo = NewHobo {
                    color: Some(UnitColor::Yellow),
                    hp: 1,
                    speed: 0.25,
                    home: village.key().num(),
                    hurried: true,
                };
                let msg = NewHoboMessage(hobo);
                let future = addr
                    .db_actor
                    .send(msg)
                    .map_err(|e| eprintln!("Attack spawn failed: {:?}", e))
                    .map(move |hobo| {
                        let pa = PlannedAttack {
                            origin_village: None,
                            destination_village: village,
                            hobos: vec![hobo.0],
                        };
                        addr.attack_funnel
                            .try_send(pa)
                            .expect("Spawning attack failed");
                    });
                Arbiter::spawn(future);
            }
            // | StoryState::VisitorArrived
            // | StoryState::FirstVisitorWelcomed
            // | StoryState::FlowerPlanted
            // | StoryState::MoreHappyVisitors
            _ => {}
        }
    }
}
