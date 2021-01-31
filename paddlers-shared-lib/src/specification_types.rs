mod hobos;
mod visitor_groups;

pub use hobos::*;
pub use visitor_groups::*;

use crate::generated::QuestName;
// Pseudo const-trait
impl QuestName {
    pub const fn const_eq(self, other: Self) -> bool {
        self as usize == other as usize
    }
}
