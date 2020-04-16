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
    text_key: String,
    button: Option<SlideButton>,
    back_button: bool,
    next_button: bool,
}
pub struct SlideButton {
    text_key: String,
    next_slide: Option<SlideIndex>,
}

pub type SlideIndex = usize;

impl Scene {
    pub fn slide_text_key(&self) -> &str {
        &self.slides[self.active_slide].text_key
    }
    pub fn current_slide(&self) -> &Slide {
        &self.slides[self.active_slide]
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

// TODO: This is not a permanent interface, just for testing
pub fn load_entry_scene() -> Scene {
    let mut slides = Vec::new();

    // 0
    slides.push(
        Slide {
            text_key: "welcomescene-B10".to_owned(),
            button: None,
            back_button: false,
            next_button: true,
        }
    );
    
    // 1
    slides.push(
        Slide {
            text_key: "welcomescene-B20".to_owned(),
            button: None,
            back_button: true,
            next_button: true,
        }
    );
    
    // 2
    slides.push(
        Slide {
            text_key: "welcomescene-B30".to_owned(),
            button: None,
            back_button: true,
            next_button: true,
        }
    );
    
    // 3
    slides.push(
        Slide {
            text_key: "welcomescene-B40".to_owned(),
            button: None,
            back_button: true,
            next_button: true,
        }
    );
    
    let button = SlideButton {
        text_key: "welcomescene-A60".to_owned(),
        next_slide: Some(5),
    };
    // 4
    slides.push(
        Slide {
            text_key: "welcomescene-B50".to_owned(),
            button: Some(button),
            back_button: true,
            next_button: false,
        }
    );
    
    // 5
    slides.push(
        Slide {
            text_key: "welcomescene-B70".to_owned(),
            button: None,
            back_button: false,
            next_button: true,
        }
    );
    

    let button = SlideButton {
        text_key: "welcomescene-A90".to_owned(),
        next_slide: None,
    };
    // 6
    slides.push(
        Slide {
            text_key: "welcomescene-B80".to_owned(),
            button: Some(button),
            back_button: true,
            next_button: false,
        }
    );

    Scene {
        slides,
        active_slide: 0,
    }
}