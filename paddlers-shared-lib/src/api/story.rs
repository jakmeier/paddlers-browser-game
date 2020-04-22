//! Shared data for network transmission for story

use crate::story::story_state::StoryState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StoryStateTransition {
    pub before: StoryState,
    pub after: StoryState,
}
