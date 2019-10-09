//! Manages the client-side persistent state of the game.
//! This state should be treated as unreliable.

use paddlers_shared_lib::prelude::{VillageKey, TEST_VILLAGE_ID};

/// Tries to read the village from the url and otherwise falls back to the player's default village
pub fn current_village() -> VillageKey {
    crate::net::url::read_current_village_id()
        .unwrap_or(TEST_VILLAGE_ID)
}