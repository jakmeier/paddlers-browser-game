use crate::{
    civilization::CivilizationPerk, const_list::ConstList, generated::QuestName,
    specification_types::VisitorGroupDefinition,
};

pub type StoryActionList = ConstList<StoryAction>;

/// An action to be performed on specific story state transitions
#[derive(Copy, Clone, Debug)]
pub enum StoryAction {
    AddMana(i16),
    SendHobo(VisitorGroupDefinition),
    StartQuest(QuestName),
    UnlockPerk(CivilizationPerk),
}
