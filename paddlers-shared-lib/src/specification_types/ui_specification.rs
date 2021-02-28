use serde::Deserialize;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum UiView {
    Dialogue,
    Leaderboard(LeaderboardViewTab),
    Mailbox,
    Map,
    Quests,
    Religion,
    Town,
    TownHelp,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum LeaderboardViewTab {
    // TownOverview, // Forestry, Karma, ...
    KarmaLeaderboard,
    IncomingAttacks,
    // PopulationLeaderboard,
}
