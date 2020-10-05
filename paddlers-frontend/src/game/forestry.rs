use crate::game::town::Town;
use crate::gui::render::Renderable;
use crate::gui::sprites::*;
use crate::gui::utils::RenderVariant;
use chrono::NaiveDateTime;
use paddle::utc_now;
use paddlers_shared_lib::game_mechanics::forestry::tree_size;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct ForestComponent {
    pub planted: NaiveDateTime,
    pub score: usize,
}

#[derive(Default, Clone, Copy)]
pub struct ForestrySystem;

impl<'a> System<'a> for ForestrySystem {
    type SystemData = (
        WriteStorage<'a, ForestComponent>,
        WriteStorage<'a, Renderable>,
        WriteExpect<'a, Town>,
    );

    fn run(&mut self, (mut forest, mut rend, mut town): Self::SystemData) {
        let now = utc_now();
        let mut total = 0;
        for (tree, r) in (&mut forest, &mut rend).join() {
            let before = tree.score;
            let t = now - tree.planted;
            tree.score = tree_size(t.into());
            if tree.score != before {
                if let RenderVariant::ImgWithImgBackground(ref mut img, _bkg) = r.kind {
                    *img = tree_sprite(tree.score);
                }
            }
            total += tree.score;
        }
        town.update_forest_size(total);
    }
}

impl ForestComponent {
    pub fn new(planted: NaiveDateTime) -> Self {
        ForestComponent {
            score: 0, // Updated by Forestsystem before use
            planted: planted,
        }
    }
}
