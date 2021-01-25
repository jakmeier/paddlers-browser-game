use crate::api::keys::BuildingKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SettleHobo {
    pub nest: BuildingKey,
}
