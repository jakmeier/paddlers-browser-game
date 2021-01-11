use crate::generated::QuestName;

/// An action to be performed when a story state is reached
pub enum StoryAction {
    StartQuest(QuestName),
    SendHobo, // TODO: Hobo (group) definition
}
