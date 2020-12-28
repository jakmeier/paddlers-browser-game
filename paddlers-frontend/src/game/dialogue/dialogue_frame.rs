use super::*;
use crate::game::{game_event_manager::game_event, Game};
use crate::gui::{
    decoration::draw_leaf_border, gui_components::*, shapes::PadlShapeIndex, sprites::*,
    ui_state::Now, utils::colors::LIGHT_BLUE, utils::*, z::*,
};
use crate::prelude::*;
use crate::{
    game::story::scene::*,
    gui::menu::{LEAVES_BORDER_H, LEAVES_BORDER_W},
};
use chrono::NaiveDateTime;
use paddle::Frame;
use paddle::*;
use paddle::{graphics::AbstractMesh, quicksilver_compat::Col};
use specs::WorldExt;

pub(crate) struct DialogueFrame {
    image: SpriteIndex,
    buttons: UiBox,
    text_lines: Vec<String>,
    text_provider: TableTextProvider,
    text_bubble: AbstractMesh,
    current_scene: Option<Scene>,
    mouse: PointerTracker,
}

impl DialogueFrame {
    pub fn new() -> PadlResult<Self> {
        let text_bubble = build_text_bubble(active_area(), text_area());
        let text_provider = TableTextProvider::new_styled("dialogue");

        let dialogue = DialogueFrame {
            buttons: UiBox::new(2, 1, 20.0, 15.0),
            image: SpriteIndex::Simple(SingleSprite::Roger),
            text_lines: Vec::new(),
            text_provider,
            text_bubble,
            current_scene: None,
            mouse: PointerTracker::new(),
        };

        Ok(dialogue)
    }

    pub fn load_scene(&mut self, scene: Scene, locale: &TextDb) {
        self.current_scene = Some(scene);
        self.reload(locale);
    }
    #[inline(always)]
    fn scene_mut(&mut self) -> PadlResult<&mut Scene> {
        self.current_scene
            .as_mut()
            .ok_or(PadlError::dev_err(PadlErrorCode::DialogueEmpty))
    }
    #[inline(always)]
    fn scene(&mut self) -> PadlResult<&Scene> {
        self.current_scene
            .as_ref()
            .ok_or(PadlError::dev_err(PadlErrorCode::DialogueEmpty))
    }
    fn load_slide(&mut self, i: usize, locale: &TextDb) -> PadlResult<()> {
        let scene = self.scene_mut()?;
        scene.set_slide(i);
        self.reload(locale);
        Ok(())
    }
    /// Panics if no scene is loaded
    fn reload(&mut self, locale: &TextDb) {
        self.buttons.clear();
        self.text_lines.clear();
        let scene = self.current_scene.as_ref().unwrap();

        // Write text into text bubble
        let key = scene.slide_text_key().key();
        let text = locale.gettext(key);
        for s in text.split("\n") {
            self.text_lines.push(s.to_owned());
        }

        // load sprite
        self.image = scene.slide_sprite();

        // Create navigation buttons
        if let Some(i) = self.scene().unwrap().back_button() {
            let back_button =
                UiElement::new(ClickOutput::SlideAction(SlideButtonAction::to_slide(i)))
                    .with_render_variant(RenderVariant::Shape(PadlShapeIndex::LeftArrow));
            self.buttons.add(back_button);
        } else {
            self.buttons.add(UiElement::empty());
        }
        if let Some(i) = self.scene().unwrap().next_button() {
            let next_button =
                UiElement::new(ClickOutput::SlideAction(SlideButtonAction::to_slide(i)))
                    .with_render_variant(RenderVariant::Shape(PadlShapeIndex::RightArrow));
            self.buttons.add(next_button);
        }

        // Create dialogue buttons for interactions
        self.load_slide_buttons(locale);
    }
    fn load_slide_buttons(&mut self, texts: &TextDb) {
        for b in self.current_scene.as_ref().unwrap().slide_buttons() {
            let button = UiElement::new(ClickOutput::SlideAction(b.action.clone()))
                .with_text(texts.gettext(b.text_key.key()).to_owned())
                .with_background_color(LIGHT_GREEN);
            self.buttons.add(button);
        }
    }

    pub fn draw_active_area(
        &mut self,
        sprites: &mut Sprites,
        now: NaiveDateTime,
        window: &mut DisplayArea,
        mouse_pos: Option<Vector>,
    ) {
        let mut table = Vec::new();
        for s in &self.text_lines {
            table.push(TableRow::Text(s.to_owned()));
        }
        table.push(TableRow::InteractiveArea(&mut self.buttons));
        draw_table(
            window,
            sprites,
            &mut table,
            &text_area(),
            &mut self.text_provider,
            60.0,
            Z_UI_MENU,
            now,
            TableVerticalAlignment::Center,
            mouse_pos,
        );
    }
    pub fn draw_background(
        &self,
        sprites: &mut Sprites,
        window: &mut DisplayArea,
        main_area: Rectangle,
    ) {
        window.draw_ex(&main_area, Col(LIGHT_BLUE), Transform::IDENTITY, Z_TEXTURE);
        let leaf_w = LEAVES_BORDER_W;
        let leaf_h = LEAVES_BORDER_H;
        let mut leaf_area = main_area.clone();
        let dx = leaf_w / 2.0;
        leaf_area.pos.x += dx;
        leaf_area.size.x -= dx;
        draw_leaf_border(window, sprites, &leaf_area, leaf_w, leaf_h);
    }

    pub fn draw_image_with_text_bubble(
        &self,
        sprites: &mut Sprites,
        window: &mut DisplayArea,
        image: SpriteIndex,
    ) {
        draw_static_image(
            sprites,
            window,
            &left_area(),
            image,
            Z_TEXTURE + 1,
            FitStrategy::Center,
        );
        window.draw_mesh(&self.text_bubble);
    }

    pub fn receive_load_scene(&mut self, state: &mut Game, msg: &LoadNewDialogueScene) {
        self.load_scene(msg.scene.load_scene(msg.slide), &state.locale);
    }
    pub fn receive_new_story_state(&mut self, state: &mut Game, msg: &NewStoryState) {
        state.set_story_state(msg.new_story_state);
        state.load_story_state().nuts_check();
    }
    fn left_click(&mut self, state: &mut Game, pos: Vector) {
        if let Some(output) = self.buttons.click(pos.into()) {
            match output {
                (ClickOutput::SlideAction(a), None) => {
                    if let Some(i) = a.next_slide {
                        self.load_slide(i, &state.locale).nuts_check();
                    }
                    if let Some(v) = a.next_view {
                        let evt = GameEvent::SwitchToView(v);
                        game_event(evt);
                    }
                    if a.actions.len() > 0 {
                        let evt = GameEvent::StoryActions(a.actions);
                        game_event(evt);
                    }
                }
                _ => {
                    PadlErrorCode::DevMsg("Unimplemented ClickOutput in dialogue.rs")
                        .dev::<()>()
                        .nuts_check();
                }
            }
        }
    }
}

impl Frame for DialogueFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::SCREEN_W;
    const HEIGHT: u32 = crate::resolution::SCREEN_H;
    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        self.text_provider.reset();
        let main_area = Rectangle::new_sized(Self::size());
        self.draw_background(&mut state.sprites, window, main_area);
        self.draw_image_with_text_bubble(&mut state.sprites, window, self.image);
        self.draw_active_area(
            &mut state.sprites,
            state.world.read_resource::<Now>().0,
            window,
            self.mouse.pos(),
        );
        self.text_provider.finish_draw();
    }

    fn leave(&mut self, _state: &mut Self::State) {
        self.text_provider.hide();
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        if let PointerEvent(PointerEventType::PrimaryClick, pos) = event {
            self.left_click(state, pos)
        }
    }
}
