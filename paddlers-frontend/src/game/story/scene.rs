use crate::game::story::StoryAction;
use crate::gui::input::UiView;
use crate::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;

/// A Scene consists of a set of slides and can be loaded in the Dialogue view.
/// It starts at a specific slide and the player can click through the, as defined on the slides.
/// Slides are referenced (within a scene) by their index.
pub struct Scene {
    slides: Vec<Slide>,
    active_slide: SlideIndex,
}

/// A Slide shows some text and optionally back/next and other buttons.
/// At least one button should be visible, or players cannot do anything to progress the scene.
pub struct Slide {
    text_key: TextKey,
    buttons: Vec<SlideButton>,
    back_button: bool,
    next_button: bool,
}
pub struct SlideButton {
    pub text_key: TextKey,
    pub action: SlideButtonAction,
}
pub enum SlideButtonAction {
    Slide(SlideIndex),
    ActionAndView(StoryAction, UiView),
}

pub type SlideIndex = usize;

impl Scene {
    pub fn slide_text_key(&self) -> &str {
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
    pub fn set_slide(&mut self, i: SlideIndex) {
        self.active_slide = i;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SceneIndex {
    Entrance,
}

impl SceneIndex {
    // Improvement: The scenes could be loaded from the server dynamically, to reduce the WASM binary size
    pub fn load_scene(&self) -> Scene {
        match self {
            Self::Entrance => load_entry_scene(),
        }
    }
    pub fn from_story_state(story_state: &StoryState) -> Option<Self> {
        match story_state {
            StoryState::Initialized
            | StoryState::TempleBuilt
            | StoryState::VisitorArrived
            | StoryState::FirstVisitorWelcomed
            | StoryState::FlowerPlanted
            | StoryState::MoreHappyVisitors
            | StoryState::TreePlanted
            | StoryState::StickGatheringStationBuild
            | StoryState::GatheringSticks => None,
            StoryState::ServantAccepted => Some(Self::Entrance),
        }
    }
}
fn load_entry_scene() -> Scene {
    let mut slides = Vec::new();

    // 0
    slides.push(Slide {
        text_key: "welcomescene-B10",
        buttons: vec![],
        back_button: false,
        next_button: true,
    });
    // 1
    slides.push(Slide {
        text_key: "welcomescene-B20",
        buttons: vec![],
        back_button: true,
        next_button: true,
    });
    // 2
    slides.push(Slide {
        text_key: "welcomescene-B30",
        buttons: vec![],
        back_button: true,
        next_button: true,
    });
    // 3
    slides.push(Slide {
        text_key: "welcomescene-B40",
        buttons: vec![],
        back_button: true,
        next_button: true,
    });
    let button = SlideButton {
        text_key: "welcomescene-A60",
        action: SlideButtonAction::Slide(5),
    };
    // 4
    slides.push(Slide {
        text_key: "welcomescene-B50",
        buttons: vec![button],
        back_button: true,
        next_button: false,
    });
    // 5
    slides.push(Slide {
        text_key: "welcomescene-B70",
        buttons: vec![],
        back_button: false,
        next_button: true,
    });

    let button = SlideButton {
        text_key: "welcomescene-A90",
        action: SlideButtonAction::ActionAndView(StoryAction::EnableTempleInShop, UiView::Town),
    };
    // 6
    slides.push(Slide {
        text_key: "welcomescene-B80",
        buttons: vec![button],
        back_button: true,
        next_button: false,
    });

    Scene {
        slides,
        active_slide: 0,
    }
}
