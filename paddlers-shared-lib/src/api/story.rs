//! Shared data for network transmission for story

use crate::story::story_state::StoryState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
/// Request a story state transition on the backend.
/// This is used for moving the story state froward by a user interaction that has otherwise on effect, such as clicking through a dialogue.
/// For story transitions waiting for a condition to be reache, the frontend does not need to send anything explcitily, the backend will instead observe the condition and act automatically.
pub struct StoryStateTransition {
    /// Current story state for idempotence
    pub now: StoryState,
}
