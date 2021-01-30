use crate::{
    const_list::ConstList, generated::QuestName, specification_types::VisitorGroupDefinition,
};

pub type StoryActionList = ConstList<StoryAction>;

/// An action to be performed on specific story state transitions
#[derive(Copy, Clone, Debug)]
pub enum StoryAction {
    StartQuest(QuestName),
    SendHobo(VisitorGroupDefinition),
}
