use crate::generated::QuestName;
use crate::prelude::BuildingType;

/// Event that can trigger a story transition
pub enum StoryTrigger {
    DialogueStoryTrigger,
    BuildingBuilt(BuildingType),
    FinishedQuest(QuestName),
}
