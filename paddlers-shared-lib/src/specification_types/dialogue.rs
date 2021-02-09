use super::*;
use crate::story::{story_state::StoryState, story_trigger::StoryChoice};
use serde::Deserialize;
#[cfg(feature = "enum_utils")]
use strum_macros::EnumVariantNames;

/// A Scene consists of a set of slides and can be loaded in the Dialogue view.
/// It starts at a specific slide and the player can click through the, as defined on the slides.
/// Slides are referenced (within a scene) by their index.
#[derive(Deserialize)]
pub struct Scene {
    slides: Vec<Slide>,
    #[serde(default)]
    active_slide: SlideIndex,
}

/// A Slide shows some text and optionally back/next and other buttons.
/// At least one button should be visible, or players cannot do anything to progress the scene.
#[derive(Deserialize)]
pub struct Slide {
    text_key: OwnedTextKey,
    buttons: Vec<SlideButton>,
    sprite: SpriteIndex,
    back_button: bool,
    next_button: bool,
}
#[derive(Deserialize)]
pub struct SlideButton {
    pub text_key: OwnedTextKey,
    pub action: SlideButtonAction,
}
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct SlideButtonAction {
    pub next_slide: Option<SlideIndex>,
    pub next_view: Option<UiView>,
    pub actions: Vec<DialogueAction>,
}

pub type SlideIndex = usize;

impl Scene {
    pub fn slide_text_key(&self) -> &OwnedTextKey {
        &self.slides[self.active_slide].text_key
    }
    pub fn current_slide(&self) -> &Slide {
        &self.slides[self.active_slide]
    }
    pub fn slide_buttons(&self) -> &[SlideButton] {
        self.slides[self.active_slide].buttons.as_slice()
    }
    pub fn back_button(&self) -> Option<SlideIndex> {
        if self.current_slide().back_button {
            Some(self.active_slide - 1)
        } else {
            None
        }
    }
    pub fn next_button(&self) -> Option<SlideIndex> {
        if self.current_slide().next_button {
            Some(self.active_slide + 1)
        } else {
            None
        }
    }
    pub fn slide_sprite(&self) -> SpriteIndex {
        self.slides[self.active_slide].sprite
    }
    #[inline]
    pub fn set_slide(&mut self, i: SlideIndex) {
        self.active_slide = i;
    }
}

#[cfg_attr(feature = "enum_utils", derive(EnumVariantNames))]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum SceneIndex {
    Entrance,
    BuildWatergate,
    WelcomeVisitor,
    NewHobo,
}

// impl SceneIndex {
//     pub fn scene_path(&self, slide: SlideIndex) -> &'static str {
//         match self {
//             Self::Entrance => load_entry_scene(slide),
//             Self::BuildWatergate => load_build_watergate_scene(slide),
//             Self::WelcomeVisitor => load_scene_two(slide),
//             Self::NewHobo => load_new_hobo_scene(slide),
//         }
//     }
// }

impl SlideButtonAction {
    pub fn to_slide(next_slide: SlideIndex) -> Self {
        SlideButtonAction {
            next_slide: Some(next_slide),
            next_view: None,
            actions: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum DialogueAction {
    OpenScene(SceneIndex, SlideIndex),
    StoryProgress(StoryState, Option<StoryChoice>),
    ClearSelectedUnit,
    SettleHobo,
}
