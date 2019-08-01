use specs::prelude::*;
use crate::Timestamp;
use crate::gui::sprites::*;
use crate::gui::render::Renderable;
use crate::gui::utils::RenderVariant;
use paddlers_shared_lib::game_mechanics::forestry::tree_size;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ForestComponent {
    pub planted: Timestamp,
    pub score: usize,
}

#[derive(Default,Clone,Copy)]
pub struct ForestrySystem;

impl<'a> System<'a> for ForestrySystem {
    type SystemData = (
        WriteStorage<'a, ForestComponent>,
        WriteStorage<'a, Renderable>,
     );

    fn run(&mut self, (mut forest, mut rend): Self::SystemData) {
        let now = crate::wasm_setup::utc_now();
        for (tree, r) in (&mut forest, &mut rend).join() {
            let before = tree.score;
            let t = chrono::Duration::microseconds(now - tree.planted);
            tree.score = tree_size(t);
            if tree.score != before {
                if let RenderVariant::ImgWithImgBackground(ref mut img, _bkg) = r.kind {
                    *img = tree_sprite(tree.score);
                }
            }
        }
    }
}

impl ForestComponent {
    pub fn new(planted: Timestamp) -> Self {
        ForestComponent {
            score: 0, // Updated by Forestsystem before use
            planted: planted,
        }
    }
}