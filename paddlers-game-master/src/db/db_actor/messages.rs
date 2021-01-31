use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use paddlers_shared_lib::{
    civilization::CivilizationPerk, generated::QuestName, prelude::*,
    story::story_state::StoryState,
};

#[derive(Debug)]
/// Deferred DB requests should not be dependent on the state of the DB
/// and instead be logically guaranteed to work. For example, the resource
/// price should already be payed before-hand.
pub enum DeferredDbStatement {
    AddMana(PlayerKey, i16),
    AssignQuest(PlayerKey, QuestName),
    NewAttack(ScheduledAttack),
    NewProphet(VillageKey),
    PlayerUpdate(PlayerKey, StoryState),
    UnlockCivPerk(PlayerKey, CivilizationPerk),
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
    pub follow_up_quest: Option<QuestKey>,
}
impl Message for CollectQuestMessage {
    type Result = ();
}
/// Get the/a village from a player
pub struct PlayerHomeLookup {
    pub player: PlayerKey,
}
pub struct PlayerHome(pub VillageKey);
impl Message for PlayerHomeLookup {
    type Result = PlayerHome;
}
impl<A, M> MessageResponse<A, M> for PlayerHome
where
    A: Actor,
    M: Message<Result = PlayerHome>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
