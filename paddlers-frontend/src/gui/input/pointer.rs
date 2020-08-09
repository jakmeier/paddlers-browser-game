//! Processes and routes mouse-like input.
//! Triggers the corresponding mouse-click systems when necessary.
use super::{drag::*, HoverSystem, MouseState};
use crate::prelude::*;
use crate::view::new_frame::{LeftClick, RightClick};
use quicksilver::prelude::*;
use specs::prelude::*;

// Tolerance thresholds
const LONG_CLICK_DELAY: i64 = 500_000; // [us]
const MIN_DRAG_DISTANCE_2: f32 = 1000.0; // [browser pixel coordinates^2]

pub struct PointerManager<'a, 'b> {
    hover_dispatcher: Dispatcher<'a, 'b>,
    drag_dispatcher: Dispatcher<'a, 'b>,
    buffered_click: Option<(Vector, PointerButton)>,
    dragging: bool,
    pointer_down: Option<(Vector, Timestamp)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointerButton {
    // Left click or short tap
    Primary,
    // Right click or long tap
    Secondary,
}

impl PointerManager<'_, '_> {
    pub fn init(mut world: &mut World) -> Self {
        world.insert(MouseState::default());

        let mut hover_dispatcher = DispatcherBuilder::new()
            .with(HoverSystem, "hov", &[])
            .build();
        hover_dispatcher.setup(&mut world);

        world.insert(Drag::default());
        let mut drag_dispatcher = DispatcherBuilder::new()
            .with(DragSystem, "drag", &[])
            .build();
        drag_dispatcher.setup(&mut world);

        PointerManager {
            hover_dispatcher,
            drag_dispatcher,
            buffered_click: None,
            dragging: false,
            pointer_down: None,
        }
    }

    pub(crate) fn run(&mut self, game: &mut crate::game::Game<'static, 'static>) {
        if let Some((fpos, button)) = self.buffered_click {
            let pos = (fpos.x as i32, fpos.y as i32);
            match button {
                PointerButton::Primary => nuts::publish(LeftClick { pos }),
                PointerButton::Secondary => nuts::publish(RightClick { pos }),
            }
            Self::update(&mut game.world, &fpos, Some(button));
        }
        self.buffered_click = None;

        if game.world.read_resource::<Drag>().is_some() {
            self.drag_dispatcher.dispatch(&mut game.world);
            game.world.write_resource::<Drag>().clear();
        }
    }

    pub fn move_pointer(&mut self, mut world: &mut World, position: &Vector) {
        Self::update(world, position, None);
        self.hover_dispatcher.dispatch(&mut world);
        if let Some((pos_before, t)) = self.pointer_down {
            if position.distance_2(&pos_before) >= MIN_DRAG_DISTANCE_2 {
                self.dragging = true;
            }
            if self.dragging {
                world.write_resource::<Drag>().add(pos_before, *position);
                self.pointer_down = Some((*position, t));
            }
        }
    }

    pub fn button_event(
        &mut self,
        now: Timestamp,
        pos: &Vector,
        button: MouseButton,
        state: ButtonState,
    ) {
        match (state, button) {
            (ButtonState::Pressed, MouseButton::Left) => {
                self.pointer_down = Some((*pos, now));
            }
            (ButtonState::Pressed, _) => {
                self.queue_click(pos, PointerButton::Secondary);
            }
            (ButtonState::Released, MouseButton::Left) => {
                if let Some((start_pos, start_t)) = self.pointer_down {
                    if !self.dragging && start_pos.distance_2(pos) < MIN_DRAG_DISTANCE_2 {
                        let key = if now - start_t < Timestamp::from_us(LONG_CLICK_DELAY) {
                            PointerButton::Primary
                        } else {
                            PointerButton::Secondary
                        };
                        self.queue_click(pos, key);
                    }
                    self.dragging = false;
                    self.pointer_down = None;
                }
            }
            _ => { /* NOP */ }
        }
    }

    fn update(world: &mut World, position: &Vector, button: Option<PointerButton>) {
        let key = button.map(|button| match button {
            PointerButton::Primary => MouseButton::Left,
            PointerButton::Secondary => MouseButton::Right,
        });
        world.insert(MouseState(*position, key));
    }

    // Current implementation only queues a single click and drops what doesn't fit
    fn queue_click(&mut self, position: &Vector, button: PointerButton) {
        if self.buffered_click.is_some() {
            // Cannot handle inputs so fast
            return;
        }
        self.buffered_click = Some((*position, button));
    }
}
