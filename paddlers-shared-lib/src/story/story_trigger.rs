use crate::prelude::BuildingType;
use crate::{const_list::ConstList, generated::QuestName};
use serde::{Deserialize, Serialize};

pub type StoryTriggerList = ConstList<StoryTrigger>;

/// Event that can trigger a story transition
#[derive(Clone, Copy, Debug)]
pub enum StoryTrigger {
    /// Client acknowledges that player went through the blocking UI states of the current story
    DialogueStoryTrigger,
    /// The player has made one of several choices available in the current story state
    DialogueChoice(StoryChoice),
    /// Player has built a certain building
    BuildingBuilt(BuildingType),
    /// Quest has been completed
    FinishedQuest(QuestName),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct StoryChoice {
    chosen_option: u8,
}

impl StoryChoice {
    pub const fn new(chosen_option: u8) -> Self {
        Self { chosen_option }
    }
}

// Pseudo const-trait
impl StoryChoice {
    pub const fn const_eq(self, other: Self) -> bool {
        self.chosen_option == other.chosen_option
    }
}
