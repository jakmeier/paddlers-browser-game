//! Shared data for network transmission for story

use crate::story::{story_state::StoryState, story_trigger::StoryChoice};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
/// Request a story state transition on the backend.
/// This is used for moving the story state froward by an explicit user interaction, such as clicking through a dialogue.
/// For story transitions waiting for a condition to be reached, the frontend does not need to send anything explicitly, the backend will instead observe the condition and act automatically.
pub struct StoryStateTransition {
    /// Current story state for idempotence
    pub now: StoryState,
    /// Choice tha player made, if there is one associated with this story transition
    pub choice: Option<StoryChoice>,
}
