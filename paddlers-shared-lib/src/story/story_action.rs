use crate::{
    civilization::CivilizationPerk, const_list::ConstList, generated::QuestName,
    specification_types::VisitorGroupDefinition,
};

pub type StoryActionList = ConstList<StoryAction>;

/// An action to be performed on specific story state transitions
#[derive(Copy, Clone, Debug)]
pub enum StoryAction {
    AddMana(i16),
    SendHobo(StoryVisitDefinition),
    StartQuest(QuestName),
    UnlockPerk(CivilizationPerk),
}

#[derive(Copy, Clone, Debug)]
pub struct StoryVisitDefinition {
    pub fixed_travel_time_s: Option<i32>,
    pub visitors: VisitorGroupDefinition,
}

impl StoryVisitDefinition {
    pub const fn new(visitors: VisitorGroupDefinition) -> Self {
        Self {
            visitors,
            fixed_travel_time_s: None,
        }
    }
    pub const fn delayed(visitors: VisitorGroupDefinition, delay: i32) -> Self {
        Self {
            visitors,
            fixed_travel_time_s: Some(delay),
        }
    }
}
