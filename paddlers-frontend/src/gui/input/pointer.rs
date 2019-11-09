//! Processes and routes mouse-like input. 
//! Triggers the corresponding mouse-click systems when necessary.
use specs::prelude::*;
use quicksilver::prelude::*;
use crate::prelude::*;
use super::{MouseState, LeftClickSystem, RightClickSystem, HoverSystem, drag::*};

// Tolerance thresholds
const DOUBLE_CLICK_DELAY: i64 = 400_000; // [us]
const DOUBLE_CLICK_DISTANCE_2: f32 = 1000.0; // [browser pixel coordinates]
const CLICK_DISTANCE_2: f32 = 1000.0; // [browser pixel coordinates]

pub struct PointerManager<'a, 'b> {
    click_dispatcher: Dispatcher<'a, 'b>,
    hover_dispatcher: Dispatcher<'a, 'b>,
    drag_dispatcher: Dispatcher<'a, 'b>,
    // for double-tap detection
    tentative_left_click: Option<(Vector, Timestamp)>,
    definitive_click: Option<(Vector, MouseButton)>,
    // for dragging
    moved: bool,
    left_down: Option<Vector>,
}

impl PointerManager<'_,'_> {
    pub fn init(mut world: &mut World) -> Self {

        world.insert(MouseState::default());

        let mut click_dispatcher = DispatcherBuilder::new()
            .with(LeftClickSystem, "lc", &[])
            .with(RightClickSystem, "rc", &[])
            .build();
        click_dispatcher.setup(&mut world);

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
            click_dispatcher,
            hover_dispatcher,
            drag_dispatcher,
            tentative_left_click: None,
            definitive_click: None,
            moved: false,
            left_down: None,
        }
    }

    pub fn run(&mut self, mut world: &mut World, now: Timestamp) {
        if let Some((pos, t)) = self.tentative_left_click {
            if t + DOUBLE_CLICK_DELAY < now {
                Self::update(world, &pos, Some(MouseButton::Left));
                self.click_dispatcher.dispatch(&mut world);
                self.tentative_left_click = None;
            }
        }
        
        if let Some((pos, button)) = self.definitive_click {
                Self::update(world, &pos, Some(button));
                self.click_dispatcher.dispatch(&mut world);
        }
        self.definitive_click = None;

        if world.read_resource::<Drag>().is_some() {
            self.drag_dispatcher.dispatch(&mut world);
            world.write_resource::<Drag>().clear();
        }
    }

    pub fn move_pointer(&mut self, mut world: &mut World, position: &Vector) {
        Self::update(world, position, None);
        self.hover_dispatcher.dispatch(&mut world);
        if let Some(pos_before) = self.left_down {
            if position.distance_2(&pos_before) >= CLICK_DISTANCE_2 {
                self.moved = true;
            }
            if self.moved {
                world.write_resource::<Drag>().add(pos_before, *position);
                self.left_down = Some(*position);
            }
        }
    }

    pub fn button_event(&mut self, now: Timestamp, pos: &Vector, button: MouseButton, state: ButtonState) {
        match (state, button) {
            (ButtonState::Pressed, MouseButton::Left) => {
                self.left_down = Some(*pos);
            },
            (ButtonState::Pressed, _) => {
                self.new_click(now, pos, button);
            },
            (ButtonState::Released, MouseButton::Left) => {
                if !self.moved {
                    self.new_click(now, pos, button);
                }
                self.moved = false;
                self.left_down = None;
            },
            _ => { /* NOP */ }
        }
    }

    fn update(world: &mut World, position: &Vector, button: Option<MouseButton>) {
        let mut ms = world.write_resource::<MouseState>();
        *ms = MouseState(*position, button);
    }

    fn new_click(&mut self, now: Timestamp, position: &Vector, button: MouseButton) {
        if self.definitive_click.is_some() {
            // Cannot handle inputs so fast
            return;
        }
        match button {
            MouseButton::Left => {
                // Double-click handling
                if let Some((p, _)) = self.tentative_left_click {
                    if position.distance_2(&p) < DOUBLE_CLICK_DISTANCE_2 {
                        // println!("Double click");
                        self.definitive_click = Self::double_click(p);
                        self.tentative_left_click = None;
                    }
                    else {
                        // println!("Distance2 too big: {}", position.distance_2(&p));
                        self.tentative_left_click = Some((*position, now));
                    }
                } else {
                    self.tentative_left_click = Some((*position, now));
                }
            }
            MouseButton::Right
            | MouseButton::Middle 
            => {
                self.definitive_click = Some((*position, button));
            }
        }
    }
    // Map all double-clicks to same events as right-clicks
    const fn double_click(position: Vector) -> Option<(Vector, MouseButton)> {
        Some((position, MouseButton::Right))
    }
}

