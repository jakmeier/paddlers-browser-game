mod dialogue;
mod hobos;
mod sprites;
mod ui_specification;
mod visitor_groups;
mod text_keys;

pub use hobos::*;
pub use visitor_groups::*;
pub use text_keys::*;

pub use dialogue::*;
pub use sprites::*;
pub use ui_specification::*;

use crate::generated::QuestName;
// Pseudo const-trait
impl QuestName {
    pub const fn const_eq(self, other: Self) -> bool {
        self as usize == other as usize
    }
}
