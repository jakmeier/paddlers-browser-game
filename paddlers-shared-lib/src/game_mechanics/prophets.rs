use crate::api::keys::VillageKey;
use crate::models::*;
use serde::{Serialize, Deserialize};
use crate::api::shop::Price;

pub fn prophets_allowed(karma: i64) -> i64 {
    match karma {
            0 ..=   999 => 0,
         1000 ..=  1999 => 1,
         2000 ..=  2999 => 2,
         3000 ..=  4999 => 3,
         5000 ..=  7499 => 4,
         7500 ..=  9999 => 5,
        10000 ..= 12499 => 6,
        12500 ..= 15999 => 7,
        16000 ..= 19999 => 8,
            k           => 9 + (k-20000) / 10000,
    }
}
pub fn prophet_cost(existing: i64) -> Price {
    let factor = existing + 2;
    Price(vec![
        (ResourceType::Feathers, 500 * factor),
        (ResourceType::Sticks, 350 * factor),
        (ResourceType::Logs, 150 * factor),
    ])
}