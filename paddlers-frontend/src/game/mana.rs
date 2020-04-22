pub use crate::gui::{gui_components::*, utils::*};
use specs::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
#[storage(HashMapStorage)]
pub struct Mana {
    pub mana: i32,
}
impl Mana {
    pub fn menu_table_infos<'a>(&self) -> Vec<TableRow<'a>> {
        let row = TableRow::ProgressBar(DARK_GREEN, BLUE, self.mana, self.max_mana(), None);
        vec![row]
    }
    fn max_mana(&self) -> i32 {
        paddlers_shared_lib::game_mechanics::worker::hero_max_mana()
    }
}
