use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use paddlers_shared_lib::{prelude::*, story::story_state::StoryState};

#[derive(Debug)]
/// Deferred DB requests should not be dependent on the state of the DB
/// and instead be logically guaranteed to work. For example, the resource
/// price should already be payed before-hand.
pub enum DeferredDbStatement {
    NewProphet(VillageKey),
    NewAttack(ScheduledAttack),
}
impl Message for DeferredDbStatement {
    type Result = ();
}
#[derive(Debug)]
pub struct ScheduledAttack {
    pub attack: NewAttack,
    pub hobos: Vec<HoboKey>,
}

pub struct NewHoboMessage(pub NewHobo);
pub struct NewHoboResponse(pub Hobo);
impl Message for NewHoboMessage {
    type Result = NewHoboResponse;
}

impl<A, M> MessageResponse<A, M> for NewHoboResponse
where
    A: Actor,
    M: Message<Result = NewHoboResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

/// Collect rewards and karma for report
pub struct CollectReportRewardsMessage(pub VisitReport);
impl Message for CollectReportRewardsMessage {
    type Result = ();
}

/// Delete quest from active quests of player and perform all reward actions
pub struct CollectQuestMessage {
    pub player: PlayerKey,
    pub quest: QuestKey,
    pub village: VillageKey,
    pub next_story_state: Option<StoryState>,
}
impl Message for CollectQuestMessage {
    type Result = ();
}
