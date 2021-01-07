use super::{CollectQuestMessage, DbActor};
use actix::{Handler, SyncContext};
use paddlers_shared_lib::prelude::GameDB;

impl Handler<CollectQuestMessage> for DbActor {
    type Result = ();
    fn handle(&mut self, msg: CollectQuestMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let db = self.db();
        for reward in db.quest_res_rewards(msg.quest) {
            if let Err(e) = db.add_resource(reward.resource_type, msg.village, reward.amount) {
                eprintln!("Reward collection failed: {}", e);
            }
        }
        if let Some(story_state) = msg.next_story_state {
            if let Err(e) = db.set_story_state(msg.player, story_state) {
                eprintln!("Setting new story state failed: {}", e);
            }
        }
        db.delete_player_quest(msg.player, msg.quest);
    }
}
