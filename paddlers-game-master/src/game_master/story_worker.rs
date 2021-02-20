use crate::db::{DbActor, DeferredDbStatement, PlayerHome, PlayerHomeLookup};
use actix::prelude::*;
use paddlers_shared_lib::{
    prelude::PlayerKey,
    story::{story_state::StoryState, story_trigger::StoryTrigger},
};

use super::attack_spawn::{AttackSpawner, SendAnonymousAttack};

/// Actor for performing story state transitions
pub struct StoryWorker {
    db_actor: Addr<DbActor>,
    attack_spawner: Addr<AttackSpawner>,
}

impl StoryWorker {
    pub fn new(db_actor: Addr<DbActor>, attack_spawner: Addr<AttackSpawner>) -> Self {
        StoryWorker {
            db_actor,
            attack_spawner,
        }
    }
}

impl Actor for StoryWorker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("StoryWorker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("StoryWorker stopped");
    }
}

/// `confirmed_current_story_state` must be verified before sending!
pub struct StoryWorkerMessage {
    player: PlayerKey,
    confirmed_current_story_state: StoryState,
    trigger: StoryTrigger,
}
impl Message for StoryWorkerMessage {
    type Result = ();
}
impl StoryWorkerMessage {
    pub fn new_verified(
        player: PlayerKey,
        confirmed_current_story_state: StoryState,
        trigger: StoryTrigger,
    ) -> Self {
        Self {
            player,
            confirmed_current_story_state,
            trigger,
        }
    }
}

impl Handler<StoryWorkerMessage> for StoryWorker {
    type Result = ();
    fn handle(&mut self, msg: StoryWorkerMessage, _ctx: &mut Context<Self>) {
        let transition = msg.confirmed_current_story_state.transition(&msg.trigger);
        if transition.is_none() {
            eprintln!(
                "Invalid transition attempt: {:?} --( {:?} )--> ???",
                msg.confirmed_current_story_state, msg.trigger
            );
            return;
        }
        let t = transition.unwrap();
        for action in t.actions.into_iter() {
            match action {
                paddlers_shared_lib::story::story_action::StoryAction::StartQuest(q) => {
                    self.db_actor
                        .do_send(DeferredDbStatement::AssignQuest(msg.player, q));
                }
                paddlers_shared_lib::story::story_action::StoryAction::SendHobo(attack_def) => {
                    let village_lookup = PlayerHomeLookup { player: msg.player };
                    let visitors = attack_def.visitors.into_iter().collect::<Vec<_>>();
                    let fixed_travel_time_s = attack_def.fixed_travel_time_s;

                    let attack_spawner = self.attack_spawner.clone();
                    let future = self
                        .db_actor
                        .send(village_lookup)
                        .and_then(move |PlayerHome(destination)| {
                            let origin = destination;
                            let msg = SendAnonymousAttack {
                                destination,
                                origin,
                                visitors,
                                fixed_travel_time_s,
                            };
                            attack_spawner.send(msg)
                        })
                        .map_err(|e| eprintln!("Attack spawn failed: {:?}", e));
                    Arbiter::spawn(future);
                }
                paddlers_shared_lib::story::story_action::StoryAction::AddMana(mana) => {
                    self.db_actor
                        .do_send(DeferredDbStatement::AddMana(msg.player, mana));
                }
                paddlers_shared_lib::story::story_action::StoryAction::UnlockPerk(perk) => {
                    self.db_actor
                        .do_send(DeferredDbStatement::UnlockCivPerk(msg.player, perk));
                }
            }
        }
        if t.next_state != msg.confirmed_current_story_state {
            self.db_actor
                .do_send(DeferredDbStatement::PlayerUpdate(msg.player, t.next_state));
        }
    }
}
