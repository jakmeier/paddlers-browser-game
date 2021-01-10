use crate::prelude::BuildingType;
use crate::generated::Quest;

/// Event that can trigger a story transition
pub enum StoryTrigger {
    DialogueStoryTrigger,
    BuildingBuilt(BuildingType),
    FinishedQuest(Quest),
}
