use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AbilityUse {
    pub unit_id: UnitKey,
    pub ability_type: AbilityType,
}