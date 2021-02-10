use super::*;
use crate::story::{story_state::StoryState, story_trigger::StoryChoice};
use serde::Deserialize;
#[cfg(feature = "enum_utils")]
use strum_macros::{AsRefStr, EnumVariantNames};
/// A Scene consists of a set of slides and can be loaded in the Dialogue view.
/// It starts at a specific slide and the player can click through the, as defined on the slides.
/// Slides are referenced (within a scene) by their index.
#[derive(Deserialize)]
pub struct Scene {
    slides: Vec<Slide>,
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
    pub fn slide_text_key(&self, i: SlideIndex) -> &OwnedTextKey {
        &self.slides[i].text_key
    }
    pub fn current_slide(&self, i: SlideIndex) -> &Slide {
        &self.slides[i]
    }
    pub fn slide_buttons(&self, i: SlideIndex) -> &[SlideButton] {
        self.slides[i].buttons.as_slice()
    }
    pub fn back_button(&self, i: SlideIndex) -> Option<SlideIndex> {
        if self.current_slide(i).back_button {
            Some(i - 1)
        } else {
            None
        }
    }
    pub fn next_button(&self, i: SlideIndex) -> Option<SlideIndex> {
        if self.current_slide(i).next_button {
            Some(i + 1)
        } else {
            None
        }
    }
    pub fn slide_sprite(&self, i: SlideIndex) -> SpriteIndex {
        self.slides[i].sprite
    }
}

#[cfg_attr(feature = "enum_utils", derive(EnumVariantNames, AsRefStr))]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Hash, Eq)]
pub enum SceneIndex {
    Entrance,
    BuildWatergate,
    WelcomeVisitor,
    NewHobo,
}

#[cfg(feature = "enum_utils")]
impl SceneIndex {
    pub fn scene_path(&self) -> String {
        let mut s = String::new();
        s += "dialogue_scenes/";
        s += self.as_ref();
        s += ".ron";
        s
    }
}

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
