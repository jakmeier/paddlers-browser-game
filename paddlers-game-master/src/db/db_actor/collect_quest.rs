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
        if let Some(quest) = msg.follow_up_quest {
            if let Err(e) = db.assign_player_quest(msg.player, quest) {
                eprintln!("Assigning follow up quest failed: {}", e);
            }
        }
        db.delete_player_quest(msg.player, msg.quest);
    }
}
