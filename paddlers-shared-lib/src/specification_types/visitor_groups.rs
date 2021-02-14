use crate::{
    const_list::ConstList,
    specification_types::hobos::{HoboLevel, HoboType, VisitorDefinition},
};

pub type VisitorGroupDefinition = ConstList<VisitorDefinition>;

pub const SINGLE_ONE_HP: VisitorGroupDefinition = ConstList::<VisitorDefinition>::singleton(
    VisitorDefinition::new_fixed_hp(HoboType::Yellow, false, 1),
);
pub const SINGLE_ONE_HP_HURRIED: VisitorGroupDefinition = ConstList::<VisitorDefinition>::singleton(
    VisitorDefinition::new_fixed_hp(HoboType::Yellow, true, 1),
);
pub const PAIR_OF_ONE_HP: VisitorGroupDefinition = ConstList::<VisitorDefinition>::new()
    .push(VisitorDefinition::new_fixed_hp(HoboType::Yellow, true, 1))
    .push(VisitorDefinition::new_fixed_hp(HoboType::Yellow, false, 1));

pub const PAIR_OF_LV0: VisitorGroupDefinition = ConstList::<VisitorDefinition>::new()
    .push(VisitorDefinition::new(
        HoboType::Yellow,
        HoboLevel::zero(),
        true,
    ))
    .push(VisitorDefinition::new(
        HoboType::Yellow,
        HoboLevel::zero(),
        false,
    ));
