use crate::generated::Quest;

/// An action to be performed when a story state is reached
pub enum StoryAction {
    StartQuest(Quest),
    SendHobo, // TODO: Hobo (group) definition
}
