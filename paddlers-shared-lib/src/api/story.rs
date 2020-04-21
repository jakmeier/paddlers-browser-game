//! Shared data for network transmission for story

use serde::{Serialize, Deserialize};
use crate::story::story_state::StoryState;

#[derive(Clone, Serialize, Deserialize)]
pub struct StoryStateTransition {
    pub before: StoryState,
    pub after: StoryState,
}