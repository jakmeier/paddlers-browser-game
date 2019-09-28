use specs::prelude::*;
pub use crate::gui::{
    utils::*,
    gui_components::*,
    sprites::{SingleSprite, SpriteIndex},
};

#[derive(Component, Debug, Clone, Copy)]
#[storage(HashMapStorage)]
pub struct Mana {
    pub mana: i32,
}
impl Mana {
    pub fn menu_table_infos<'a>(&self) -> Vec<TableRow<'a>> {
        let text = format!("Mana {}/{}", self.mana, self.max_mana());
        let row = TableRow::TextWithImage(
            text,
            SpriteIndex::Simple(SingleSprite::Water),
        );
        vec![row]
    }
    fn max_mana(&self) -> i32 {
        paddlers_shared_lib::game_mechanics::worker::hero_max_mana()
    }
}