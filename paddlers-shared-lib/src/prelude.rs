pub use crate::models::*;
#[cfg(feature = "sql_db")] 
pub use crate::sql_db::sql::GameDB;
pub use crate::config::Config;
pub use crate::PadlId;
#[cfg(feature = "sql_db")] 
pub use crate::sql_db::keys::{VillageKey, SqlKey};

//TODO Delete this once multiple villages are properly supported
#[cfg(feature = "sql_db")] 
pub const TEST_VILLAGE_ID: VillageKey = VillageKey(1);
#[cfg(feature = "sql_db")] 
pub const TEST_AI_VILLAGE_ID: VillageKey = VillageKey(2);