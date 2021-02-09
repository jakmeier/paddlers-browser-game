use serde::Deserialize;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum UiView {
    Visitors(VisitorViewTab),
    Leaderboard,
    Map,
    Town,
    TownHelp,
    Dialogue,
    Religion,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum VisitorViewTab {
    IncomingAttacks,
    Letters,
    Quests,
}