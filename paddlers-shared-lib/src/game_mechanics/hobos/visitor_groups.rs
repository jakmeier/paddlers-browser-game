use crate::const_list::ConstList;

use super::{HoboLevel, HoboType, VisitorDefinition};

pub type VisitorGroupDefinition = ConstList<VisitorDefinition>;

pub const SINGLE_ONE_HP: VisitorGroupDefinition = ConstList::<VisitorDefinition>::singleton(VisitorDefinition::new(
    HoboType::Yellow,
    HoboLevel::zero(),
    false,
));
pub const SINGLE_ONE_HP_HURRIED: VisitorGroupDefinition = ConstList::<VisitorDefinition>::singleton(
    VisitorDefinition::new(HoboType::Yellow, HoboLevel::zero(), true),
);
