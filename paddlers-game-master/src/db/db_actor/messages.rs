use actix::prelude::*;
use actix::dev::{MessageResponse, ResponseChannel};
use paddlers_shared_lib::prelude::*;

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
pub struct NewHoboResponse(pub HoboKey);
impl Message for NewHoboMessage {
    type Result = NewHoboResponse;
}

impl<A,M> MessageResponse<A,M> for NewHoboResponse 
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