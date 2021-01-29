use crate::{
    const_list::ConstList, game_mechanics::hobos::VisitorGroupDefinition, generated::QuestName,
};

pub type StoryActionList = ConstList<StoryAction>;

/// An action to be performed on specific story state transitions
#[derive(Copy, Clone)]
pub enum StoryAction {
    StartQuest(QuestName),
    SendHobo(VisitorGroupDefinition),
}
