//! In the dialogue view, the full screen us used to display text and images.
//! It is mainly used to display conversations with paddlers to explain the story of Paddland.

mod dialogue_frame;
mod text_bubble;
pub use dialogue_frame::*;
pub use text_bubble::*;

use crate::game::story::scene::*;
use paddle::*;
use paddlers_shared_lib::story::story_state::StoryState;

/// Command struct to update current dialogue scene
pub struct LoadNewDialogueScene {
    scene: SceneIndex,
    slide: SlideIndex,
}
impl LoadNewDialogueScene {
    pub fn new(scene: SceneIndex, slide: SlideIndex) -> Self {
        Self { scene, slide }
    }
}
/// Command struct to update current story state
pub struct NewStoryState {
    pub new_story_state: StoryState,
}

const AREA_DIVISION_RATIO: f32 = 0.38195;
const fn inner_frame_area() -> Rectangle {
    let size = Vector {
        x: DialogueFrame::WIDTH as f32,
        y: DialogueFrame::HEIGHT as f32,
    };
    let area = Rectangle {
        pos: Vector::ZERO,
        size,
    };
    area.const_shrink_to_center(0.875)
}
const fn left_area() -> Rectangle {
    let area = inner_frame_area();

    Rectangle {
        pos: area.pos,
        size: Vector {
            x: area.size.x * AREA_DIVISION_RATIO,
            y: area.size.y,
        },
    }
}

const fn active_area() -> Rectangle {
    let area = inner_frame_area();

    Rectangle {
        pos: Vector {
            x: area.pos.x + area.size.x * AREA_DIVISION_RATIO,
            y: area.pos.y,
        },
        size: Vector {
            x: area.size.x * (1.0 - AREA_DIVISION_RATIO),
            y: area.size.y,
        },
    }
}

const fn text_area() -> Rectangle {
    let active_area = active_area();
    let mut text_area = active_area.const_shrink_to_center(0.875);
    let d = 0.1875 * active_area.size.x;
    text_area.size.x -= d;
    text_area.pos.x += d;
    text_area
}
