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
use quicksilver_compat::Shape;
use specs::WorldExt;

pub(crate) struct DialogueFrame {
    scenes: SceneLoader,
    image: SpriteIndex,
    buttons: UiBox,
    current_slide_text_style: SlideTextStyle,
    current_button_layout: ButtonLayout,
    text: String,
    text_provider: TableTextProvider,
    text_bubble_to_left: AbstractMesh,
    text_bubble_to_player: AbstractMesh,
    current_scene: Option<SceneIndex>,
    slide_stack: Vec<SlideIndex>,
    mouse: PointerTracker,
    waiting_for_scene_data: bool,
}

impl DialogueFrame {
    pub fn new() -> PadlResult<Self> {
        const ZEROED_RIGHT_AREA: Rectangle = Rectangle {
            pos: Vector::ZERO,
            size: right_area().size,
        };
        const TEXT_AREA: Rectangle = Rectangle {
            pos: Vector {
                x: text_area().pos.x - right_area().pos.x,
                y: text_area().pos.y - right_area().pos.y,
            },
            size: text_area().size,
        };
        let text_bubble_to_left = build_text_bubble_to_left(ZEROED_RIGHT_AREA, TEXT_AREA);
        let text_bubble_to_player = build_text_bubble_to_bottom(ZEROED_RIGHT_AREA, TEXT_AREA);
        let text_provider = TableTextProvider::new_styled("dialogue");
        let button_layout = ButtonLayout::SingleRow;
        let current_slide_text_style = SlideTextStyle::SystemMessage;

        let dialogue = DialogueFrame {
            current_button_layout: button_layout,
            current_slide_text_style,
            buttons: buttons_ui_box(button_layout),
            image: SpriteIndex::Simple(SingleSprite::Roger),
            text: String::new(),
            text_provider,
            text_bubble_to_left,
            text_bubble_to_player,
            current_scene: None,
            slide_stack: vec![],
            mouse: PointerTracker::new(),
            scenes: Default::default(),
            waiting_for_scene_data: false,
        };

        Ok(dialogue)
    }

    pub fn load(&mut self, scene: SceneIndex, slide: SlideIndex, locale: &TextDb) {
        self.current_scene = Some(scene);
        self.slide_stack.push(slide);
        self.reload(locale);
    }
    fn load_slide(&mut self, i: usize, locale: &TextDb) -> PadlResult<()> {
        if self.current_slide() != i {
            self.slide_stack.push(i);
        }
        self.reload(locale);
        Ok(())
    }
    /// Panics if no scene is active
    fn reload(&mut self, locale: &TextDb) {
        self.buttons.clear();
        self.text.clear();

        let current_slide = self.current_slide();
        if let Some(scene_index) = self.current_scene {
            if let Some(scene) = self.scenes.get(scene_index) {
                let key = scene.slide_text_key(current_slide).key();
                let text = locale.gettext(key);
                let image = scene.slide_sprite(current_slide);
                let has_back_button = scene.back_button(current_slide);
                let next_button = scene.next_button(current_slide);
                let layout = scene.button_layout(current_slide);

                if self.current_button_layout != layout {
                    self.buttons = buttons_ui_box(layout);
                    self.current_button_layout = layout;
                }

                self.current_slide_text_style = scene.text_style(current_slide);

                self.text += text;
                self.image = image;

                // Create dialogue buttons for interactions
                let extra_buttons = scene.slide_buttons(current_slide);
                // Create and add navigation buttons
                if has_back_button {
                    let back_button =
                        UiElement::new(ClickOutput::SlideAction(SlideButtonAction::go_back()))
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
                self.waiting_for_scene_data = true;
            }
        } else {
            panic!("Called reload without an active scene.")
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
        let main_text_color = match self.current_slide_text_style {
            SlideTextStyle::SystemMessage => TextColor::White,
            _ => TextColor::Black,
        };
        if self.text.len() > 0 {
            let rows = rows_for_text(self.current_button_layout);
            table.push(TableRow::MultiRowText(
                self.text.clone(),
                rows,
                main_text_color,
            ));
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

    pub fn draw_slide_background(
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
        match self.current_slide_text_style {
            SlideTextStyle::SpeechBubbleToLeft => {
                window.draw_mesh(&self.text_bubble_to_left, right_area(), &Color::WHITE);
            }
            SlideTextStyle::PlayerSpeech => {
                window.draw_mesh(&self.text_bubble_to_player, right_area(), &LIGHT_GREEN);
            }
            SlideTextStyle::SystemMessage => {
                window.draw(&text_area(), &DARK_BLUE);
            }
        }
    }
    pub fn init_listeners(frame_handle: FrameHandle<Self>) {
        frame_handle.listen(DialogueFrame::receive_load_scene);
        frame_handle.listen(DialogueFrame::receive_new_story_state);
        frame_handle.register_receiver(DialogueFrame::receive_scene)
    }
    fn receive_scene(&mut self, state: &mut Game, msg: scene_loader::SceneResponse) {
        if let Some(wait_for_it) = self.current_scene {
            if msg.index == wait_for_it {
                self.waiting_for_scene_data = false;
            }
        }
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
                    match a.next_view {
                        NextView::Stay => {}
                        NextView::GoOneSlideBack => {
                            self.slide_stack.pop();
                            self.reload(&state.locale);
                        }
                        NextView::Slide(i) => {
                            self.load_slide(i, &state.locale).nuts_check();
                        }
                        NextView::UiView(view) => {
                            let evt = GameEvent::SwitchToView(view);
                            game_event(evt);
                        }
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
    fn current_slide(&self) -> SlideIndex {
        self.slide_stack.last().cloned().unwrap_or(0)
    }
}

impl Frame for DialogueFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::SCREEN_W;
    const HEIGHT: u32 = crate::resolution::SCREEN_H;
    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, timestamp: f64) {
        self.text_provider.reset();
        self.draw_background(&mut state.sprites, window, Self::area());
        if self.waiting_for_scene_data {
            let rot = timestamp / 2000.0 * 360.0;
            let splash_area = Rectangle::new_sized((500.0, 500.0)).with_center(Self::size() / 2.0);
            draw_image(
                &mut state.sprites,
                window,
                &splash_area,
                SpriteIndex::Simple(SingleSprite::Karma),
                Z_UI_MENU,
                FitStrategy::Center,
                Transform::rotate(rot),
            );
        } else {
            self.draw_slide_background(&mut state.sprites, window, self.image);
            self.draw_active_area(
                &mut state.sprites,
                state.world.read_resource::<Now>().0,
                window,
                self.mouse.pos(),
            );
        }
        self.text_provider.finish_draw();
    }

    fn leave(&mut self, _state: &mut Self::State) {
        self.text_provider.hide();
        self.current_scene = None;
        self.slide_stack.clear();
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        if let PointerEvent(PointerEventType::PrimaryClick, pos) = event {
            self.left_click(state, pos)
        }
    }
}
fn buttons_ui_box(layout: ButtonLayout) -> UiBox {
    match layout {
        ButtonLayout::SingleColumn => UiBox::new(1, 4, 20.0, 15.0),
        ButtonLayout::SingleRow => UiBox::new(2, 1, 20.0, 15.0),
    }
}
fn rows_for_text(layout: ButtonLayout) -> u32 {
    match layout {
        ButtonLayout::SingleColumn => 5,
        ButtonLayout::SingleRow => 8,
    }
}
