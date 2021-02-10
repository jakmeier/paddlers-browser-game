mod dialogue;
mod hobos;
mod sprites;
mod text_keys;
mod ui_specification;
mod visitor_groups;

pub use hobos::*;
pub use text_keys::*;
pub use visitor_groups::*;

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
