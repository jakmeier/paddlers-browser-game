use crate::net::graphql::query_types::PlayerQueryResponse;
use paddlers_shared_lib::game_mechanics::prophets::*;
use paddlers_shared_lib::story::story_state::StoryState;
use paddlers_shared_lib::{api::shop::Price, civilization::CivilizationPerks};

#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    karma: i64,
    /// Prophets currently owned by player that are not ruling a village, yet
    prophets: i64,
    village_count: i64,
    story_state: StoryState,
    civilization_perks: CivilizationPerks,
}

impl From<PlayerQueryResponse> for PlayerInfo {
    fn from(p: PlayerQueryResponse) -> Self {
        PlayerInfo {
            karma: p.karma,
            prophets: p.prophet_count,
            village_count: p.village_count,
            story_state: p.story_state.into(),
            civilization_perks: CivilizationPerks::decode(p.civilization as i32),
        }
    }
}

impl PlayerInfo {
    #[inline]
    pub fn karma(&self) -> i64 {
        self.karma
    }
    #[inline]
    pub fn set_story_state(&mut self, s: StoryState) {
        self.story_state = s;
    }
    #[inline]
    pub fn story_state(&self) -> StoryState {
        self.story_state
    }
    #[inline]
    pub fn civilization_perks(&self) -> CivilizationPerks {
        self.civilization_perks
    }
    /// Count of current hobo prophets available to the player, either idle or on a mission
    #[inline]
    pub fn prophets_available(&self) -> i64 {
        self.prophets
    }
    #[inline]
    pub fn prophets_limit(&self) -> i64 {
        prophets_allowed(self.karma) - self.village_count + 1
    }
    pub fn karma_for_next_prophet(&self) -> i64 {
        // FIXME: This could be a single lookup
        let mut karma = self.karma;
        let current = prophets_allowed(karma);
        while prophets_allowed(karma) == current {
            karma += 50;
        }
        while prophets_allowed(karma) != current {
            karma -= 1;
        }
        karma + 1
    }
    #[inline]
    pub fn prophets_total(&self) -> i64 {
        self.prophets + self.village_count - 1
    }
    #[inline]
    #[allow(dead_code)]
    pub fn prophet_price(&self) -> Price {
        prophet_cost(self.prophets_total())
    }
}
