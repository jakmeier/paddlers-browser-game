use super::{scene_loader::SceneLoader, text_area, *};
use crate::game::{game_event_manager::game_event, Game};
use crate::gui::menu::{LEAVES_BORDER_H, LEAVES_BORDER_W};
use crate::gui::{
    decoration::draw_leaf_border, gui_components::*, shapes::PadlShapeIndex, sprites::*,
    ui_state::Now, utils::colors::LIGHT_BLUE, utils::*, z::*,
};
use crate::prelude::*;
use chrono::NaiveDateTime;
use paddle::Frame;
use paddle::*;
use paddle::{graphics::AbstractMesh, quicksilver_compat::Color};
use specs::WorldExt;

pub(crate) struct DialogueFrame {
    scenes: SceneLoader,
    image: SpriteIndex,
    buttons: UiBox,
    text: String,
    text_provider: TableTextProvider,
    text_bubble: AbstractMesh,
    current_scene: Option<SceneIndex>,
    current_slide: SlideIndex,
    mouse: PointerTracker,
}

impl DialogueFrame {
    pub fn new() -> PadlResult<Self> {
        let text_bubble = build_text_bubble(
            right_area().const_translate(-right_area().pos),
            text_area().const_translate(-right_area().pos),
        );
        let text_provider = TableTextProvider::new_styled("dialogue");

        let dialogue = DialogueFrame {
            buttons: UiBox::new(2, 1, 20.0, 15.0),
            image: SpriteIndex::Simple(SingleSprite::Roger),
            text: String::new(),
            text_provider,
            text_bubble,
            current_scene: None,
            current_slide: 0,
            mouse: PointerTracker::new(),
            scenes: Default::default(),
        };

        Ok(dialogue)
    }

    pub fn load(&mut self, scene: SceneIndex, slide: SlideIndex, locale: &TextDb) {
        self.current_scene = Some(scene);
        self.current_slide = slide;
        self.reload(locale);
    }
    fn load_slide(&mut self, i: usize, locale: &TextDb) -> PadlResult<()> {
        self.current_slide = i;
        self.reload(locale);
        Ok(())
    }
    /// Panics if no scene is active
    fn reload(&mut self, locale: &TextDb) {
        self.buttons.clear();
        self.text.clear();

        if let Some(scene_index) = self.current_scene {
            if let Some(scene) = self.scenes.get(scene_index) {
                let key = scene.slide_text_key(self.current_slide).key();
                let text = locale.gettext(key);
                let image = scene.slide_sprite(self.current_slide);
                let back_button = scene.back_button(self.current_slide);
                let next_button = scene.next_button(self.current_slide);

                self.text += text;
                self.image = image;

                // Create dialogue buttons for interactions
                let extra_buttons = scene.slide_buttons(self.current_slide);
                // Create and add navigation buttons
                if let Some(i) = back_button {
                    let back_button =
                        UiElement::new(ClickOutput::SlideAction(SlideButtonAction::to_slide(i)))
                            .with_render_variant(RenderVariant::Shape(PadlShapeIndex::LeftArrow));
                    self.buttons.add(back_button);
                } else if extra_buttons.len() < 2 {
                    self.buttons.add(UiElement::empty());
                }
                if let Some(i) = next_button {
                    let next_button =
                        UiElement::new(ClickOutput::SlideAction(SlideButtonAction::to_slide(i)))
                            .with_render_variant(RenderVariant::Shape(PadlShapeIndex::RightArrow));
                    self.buttons.add(next_button);
                }

                // Add dialogue buttons for interactions
                for b in extra_buttons {
                    let button = UiElement::new(ClickOutput::SlideAction(b.action.clone()))
                        .with_text(locale.gettext(b.text_key.key()).to_owned())
                        .with_background_color(LIGHT_GREEN);
                    self.buttons.add(button);
                }
            } else {
                // TODO: display something that shows that data is loading
            }
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
        if self.text.len() > 0 {
            let rows = 8;
            table.push(TableRow::MultiRowText(self.text.clone(), rows));
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
        window.draw_ex(&main_area, &LIGHT_BLUE, Transform::IDENTITY, Z_TEXTURE);
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
        window.draw_mesh(&self.text_bubble, right_area(), &Color::WHITE);
    }
    pub fn init_listeners(frame_handle: FrameHandle<Self>) {
        frame_handle.listen(DialogueFrame::receive_load_scene);
        frame_handle.listen(DialogueFrame::receive_new_story_state);
        frame_handle.register_receiver(DialogueFrame::receive_scene)
    }
    fn receive_scene(&mut self, state: &mut Game, msg: scene_loader::SceneResponse) {
        self.scenes.add(msg);
        self.reload(&state.locale);
    }
    fn receive_load_scene(&mut self, state: &mut Game, msg: &LoadNewDialogueScene) {
        self.load(msg.scene, msg.slide, &state.locale);
    }
    fn receive_new_story_state(&mut self, state: &mut Game, msg: &NewStoryState) {
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
                        let evt = GameEvent::DialogueActions(a.actions);
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
