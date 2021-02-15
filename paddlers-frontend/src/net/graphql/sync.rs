//! Schedules requests that need to be sent frequently or at specific moments to keep the client in sync.

use crate::net::NetState;

use super::GraphQlState;

pub enum ForceRequest {
    SyncAsap(PeriodicalSyncRequest),
    /// With delay ticks
    Extra(ScheduledRequest, usize),
}

/// Any request placed in here must be justified.
/// If there is no scenario where an update could not be forseen by the client, then that requests should probably not be in here.
/// Sync requests can be forced, too, to fast-forward when they are executed.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum PeriodicalSyncRequest {
    /// New attack can appear anytime, even send by other players.
    Attacks,
    /// The exact time when reports are generated are a bit unpredictable, thus it remains in here. Would be nice to remove it, though.
    Reports,
    /// May change from workers. Right now, this is not a client-known event.
    Resources,
    /// For Karma changes, civ perks, etc. Could probably be removed here if all cases are handled properly.
    PlayerInfo,
}
/// For irregular requests that sometime need to be scheduled extra.
/// They are queued and sent indirectly, just like forced sync requests, for two reasons.
/// 1) Timing:
///      This allows to schedule with the nuts execution flow.
///      Otherwise, here are weir timing issues where request are sent in different orders than they are triggered in the code.
///      Additionally, this allows to use some additional delay where necessary, to have some best-effort bad timing avoidance.
/// 2) Efficiency:
///      The strategy allows multiple requests for the same item to be summarized to a single request.
///      Scheduled requests also just fast-forward the already scheduled requests, instead of being entirely additional.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum ScheduledRequest {
    Workers,
}

pub struct SyncState {
    periodical: SyncElementList<PeriodicalSyncRequest>,
    scheduled: SyncElementList<ScheduledRequest>,
}
struct SyncElementList<R: RequestDescriptor>(Vec<SyncElement<R>>);

impl SyncState {
    pub fn new() -> Self {
        use PeriodicalSyncRequest::*;
        Self {
            periodical: SyncElementList(vec![
                SyncElement::new(Attacks, 100),
                SyncElement::new(Reports, 100),
                SyncElement::new(Resources, 100),
                SyncElement::new(PlayerInfo, 100),
            ]),
            scheduled: SyncElementList(Vec::new()),
        }
    }
}

/// Maintains a counter after how many ticks a new request should be sent
struct SyncElement<R: RequestDescriptor> {
    request: R,
    reset_value: usize,
    counter: usize,
}

impl<R: RequestDescriptor> SyncElement<R> {
    pub(super) fn new(request: R, reset_value: usize) -> Self {
        Self {
            request,
            reset_value,
            counter: reset_value,
        }
    }
}

impl<R: RequestDescriptor> SyncElementList<R> {
    fn send_due(&self, net_state: &NetState) {
        for req in &self.0 {
            if req.counter == 0 {
                req.request.send(&net_state);
            }
        }
    }
}
impl NetState {
    /// Decrease counter for all sync requests, send requests that are due
    pub fn sync_tick(&mut self) {
        // Periodical
        self.sync.periodical.send_due(&self);
        for req in &mut self.sync.periodical.0 {
            if req.counter == 0 {
                req.counter = req.reset_value;
            } else {
                req.counter -= 1;
            }
        }
        self.sync.scheduled.send_due(&self);
        self.sync.scheduled.0.retain(|el| el.counter > 0);
        for req in &mut self.sync.scheduled.0 {
            req.counter -= 1;
        }
    }
    pub fn scheduled_update(&mut self, msg: &ForceRequest) {
        match msg {
            ForceRequest::SyncAsap(sync_req) => {
                for p in &mut self.sync.periodical.0 {
                    if p.request == *sync_req {
                        p.counter = 0;
                        return;
                    }
                }
                panic!("PeriodicalSynRequest not found: {:?}", sync_req);
            }
            ForceRequest::Extra(req, delay) => {
                self.sync.scheduled.0.push(SyncElement::new(*req, *delay));
            }
        }
    }
}

trait RequestDescriptor {
    fn send(&self, net_state: &NetState);
}

impl RequestDescriptor for PeriodicalSyncRequest {
    fn send(&self, net_state: &NetState) {
        match self {
            PeriodicalSyncRequest::Attacks => {
                net_state.transfer_response(net_state.gql_state.attacks_query());
            }
            PeriodicalSyncRequest::Reports => {
                net_state.transfer_response(net_state.gql_state.reports_query());
            }
            PeriodicalSyncRequest::Resources => {
                net_state.transfer_response(GraphQlState::resource_query());
            }
            PeriodicalSyncRequest::PlayerInfo => {
                net_state.transfer_response(GraphQlState::player_info_query());
            }
        }
    }
}
impl RequestDescriptor for ScheduledRequest {
    fn send(&self, net_state: &NetState) {
        match self {
            ScheduledRequest::Workers => {
                net_state.transfer_response(GraphQlState::workers_query());
            }
        }
    }
}
