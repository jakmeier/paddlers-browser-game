pub use crate::models::*;
#[cfg(feature = "sql_db")] 
pub use crate::sql_db::sql::GameDB;
pub use crate::config::Config;
pub use crate::{PadlId, VillageKey};

//TODO Delete this once multiple villages are properly supported
pub const TEST_VILLAGE_ID: VillageKey = 1;
pub const TEST_AI_VILLAGE_ID: VillageKey = 2;