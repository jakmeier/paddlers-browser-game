//! Manages the client-side persistent state of the game.
//!
//! This state should be treated as unreliable, it should only hold temporary data.
//! It may be cached on the client for performance reasons but the game must
//! be able to work even if the state from the previous session is unavailable.

use std::sync::atomic::{AtomicI64, Ordering};

use crate::net::graphql::own_villages_query;
use crate::prelude::*;
use paddlers_shared_lib::prelude::*;

/// Holds information of the current session
struct ClientState {
    village: AtomicI64,
}

static STATE: ClientState = ClientState {
    village: AtomicI64::new(-1),
};

/// Reads the currently displayed village key from the client state
///
/// This function panics if it is called before the server connection
/// has been set up and the village key established
pub fn current_village() -> VillageKey {
    let vid = STATE.village.load(Ordering::Relaxed);
    if vid >= 0 {
        return VillageKey(vid);
    }
    load_current_village().expect("Reading village too early")
}

fn load_current_village() -> Option<VillageKey> {
    if let Ok(key) = crate::net::url::read_current_village_id() {
        Some(key)
    } else {
        let vid = STATE.village.load(Ordering::Relaxed);
        if vid >= 0 {
            Some(VillageKey(vid))
        } else {
            None
        }
    }
}

/// Tries to read the village from the url and otherwise reads it from the server
pub async fn current_village_async() -> PadlResult<VillageKey> {
    let vid = STATE.village.load(Ordering::Relaxed);
    if vid >= 0 {
        return Ok(VillageKey(vid));
    }

    let keys = own_villages_query().await?;
    let key = keys[0];
    STATE.village.store(key.num(), Ordering::Relaxed);
    Ok(key)
}
