use crate::game::story::DialogueAction;
use crate::gui::input::UiView;
use crate::gui::sprites::*;
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
    sprite: SpriteIndex,
    back_button: bool,
    next_button: bool,
}
pub struct SlideButton {
    pub text_key: TextKey,
    pub action: SlideButtonAction,
}
#[derive(Default, Clone, Debug, PartialEq)]
pub struct SlideButtonAction {
    pub next_slide: Option<SlideIndex>,
    pub next_view: Option<UiView>,
    pub actions: Vec<DialogueAction>,
}

pub type SlideIndex = usize;

impl Scene {
    pub fn slide_text_key(&self) -> &TextKey {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SceneIndex {
    Entrance,
    BuildWatergate,
    WelcomeVisitor,
    NewHobo,
}

impl SceneIndex {
    // Improvement: The scenes could be loaded from the server dynamically, to reduce the WASM binary size
    pub fn load_scene(&self, slide: SlideIndex) -> Scene {
        match self {
            Self::Entrance => load_entry_scene(slide),
            Self::BuildWatergate => load_build_watergate_scene(slide),
            Self::WelcomeVisitor => load_scene_two(slide),
            Self::NewHobo => load_new_hobo_scene(slide),
        }
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
    fn with_action(mut self, a: DialogueAction) -> Self {
        self.actions.push(a);
        self
    }
    fn with_view_change(mut self, v: UiView) -> Self {
        self.next_view = Some(v);
        self
    }
}

// TODO [0.1.5] Define scenes in a different way, not as functions compiled right into the binary

fn load_entry_scene(active_slide: SlideIndex) -> Scene {
    let mut slides = Vec::new();

    // 0
    slides.push(Slide {
        text_key: "welcomescene-B10".into(),
        buttons: vec![],
        sprite: SpriteIndex::Simple(SingleSprite::RogerLargeAstonished),
        back_button: false,
        next_button: true,
    });
    // 1
    slides.push(Slide {
        text_key: "welcomescene-B20".into(),
        sprite: SpriteIndex::Simple(SingleSprite::RogerLarge),
        buttons: vec![],
        back_button: true,
        next_button: true,
    });
    // 2
    slides.push(Slide {
        text_key: "welcomescene-B30".into(),
        sprite: SpriteIndex::Simple(SingleSprite::RogerLarge),
        buttons: vec![],
        back_button: true,
        next_button: true,
    });
    // 3
    slides.push(Slide {
        text_key: "welcomescene-B40".into(),
        sprite: SpriteIndex::Simple(SingleSprite::RogerLargeSad),
        buttons: vec![],
        back_button: true,
        next_button: true,
    });
    let button = SlideButton {
        text_key: "welcomescene-A60".into(),
        action: SlideButtonAction::to_slide(5).with_action(DialogueAction::StoryProgress(
            StoryState::ServantAccepted,
            None,
        )),
    };
    // 4
    slides.push(Slide {
        text_key: "welcomescene-B50".into(),
        sprite: SpriteIndex::Simple(SingleSprite::RogerLargeObedient),
        buttons: vec![button],
        back_button: true,
        next_button: false,
    });
    // 5
    slides.push(Slide {
        text_key: "welcomescene-B70".into(),
        sprite: SpriteIndex::Simple(SingleSprite::RogerLargeCelebrating),
        buttons: vec![],
        back_button: false,
        next_button: true,
    });

    let button = SlideButton {
        text_key: "welcomescene-A90".into(),
        action: SlideButtonAction::default()
            .with_view_change(UiView::Town)
            .with_action(DialogueAction::TownSelectEntity(None)),
    };
    // 6
    slides.push(Slide {
        text_key: "welcomescene-B80".into(),
        sprite: SpriteIndex::Simple(SingleSprite::RogerLarge),
        buttons: vec![button],
        back_button: true,
        next_button: false,
    });

    Scene {
        slides,
        active_slide,
    }
}

fn load_scene_two(active_slide: SlideIndex) -> Scene {
    let mut slides = Vec::new();

    // 0
    slides.push(Slide {
        text_key: "gatebuilt-A0".into(),
        buttons: vec![],
        sprite: SpriteIndex::Simple(SingleSprite::RogerLarge),
        back_button: false,
        next_button: true,
    });
    // 1
    slides.push(Slide {
        text_key: "gatebuilt-A10".into(),
        buttons: vec![],
        sprite: SpriteIndex::Simple(SingleSprite::Duck),
        back_button: true,
        next_button: true,
    });
    // 2
    slides.push(Slide {
        text_key: "gatebuilt-A20".into(),
        buttons: vec![],
        sprite: SpriteIndex::Simple(SingleSprite::RogerLarge),
        back_button: true,
        next_button: true,
    });
    // 3
    let button = SlideButton {
        text_key: "button-back-to-town".into(),
        action: SlideButtonAction::default()
            .with_action(DialogueAction::StoryProgress(StoryState::VisitorArrived, None))
            .with_view_change(UiView::Town),
    };
    slides.push(Slide {
        text_key: "gatebuilt-H30".into(),
        buttons: vec![button],
        sprite: SpriteIndex::Simple(SingleSprite::WelcomeAbility),
        back_button: true,
        next_button: false,
    });
    Scene {
        slides,
        active_slide,
    }
}

fn load_build_watergate_scene(active_slide: SlideIndex) -> Scene {
    let mut slides = Vec::new();

    // 0
    slides.push(Slide {
        text_key: "templebuilt-A0".into(),
        buttons: vec![],
        sprite: SpriteIndex::Simple(SingleSprite::Temple),
        back_button: false,
        next_button: true,
    });
    // 1
    slides.push(Slide {
        text_key: "templebuilt-A10".into(),
        buttons: vec![],
        sprite: SpriteIndex::Simple(SingleSprite::RogerLarge),
        back_button: true,
        next_button: true,
    });
    // 2
    let button = SlideButton {
        text_key: "button-back-to-town".into(),
        action: SlideButtonAction::default()
            .with_action(DialogueAction::StoryProgress(
                StoryState::BuildingWatergate,
                None,
            ))
            .with_action(DialogueAction::TownSelectEntity(None))
            .with_view_change(UiView::Town),
    };
    slides.push(Slide {
        text_key: "templebuilt-H20".into(),
        buttons: vec![button],
        sprite: SpriteIndex::Simple(SingleSprite::Stone2),
        back_button: true,
        next_button: false,
    });
    Scene {
        slides,
        active_slide,
    }
}

fn load_new_hobo_scene(active_slide: SlideIndex) -> Scene {
    let mut slides = Vec::new();
    let yes_button = SlideButton {
        text_key: "button-yes".into(),
        action: SlideButtonAction::default()
            .with_action(DialogueAction::SettleHobo)
            .with_view_change(UiView::Town),
    };
    let no_button = SlideButton {
        text_key: "button-no".into(),
        action: SlideButtonAction::default().with_view_change(UiView::Town),
    };
    slides.push(Slide {
        text_key: "new-hobo-text".into(),
        buttons: vec![no_button, yes_button],
        sprite: SpriteIndex::Simple(SingleSprite::DuckHappy),
        back_button: false,
        next_button: false,
    });
    Scene {
        slides,
        active_slide,
    }
}
