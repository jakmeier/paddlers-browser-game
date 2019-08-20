//! Processes and routes mouse-like input. 
//! Triggers the corresponding mouse-click systems when necessary.
use specs::prelude::*;
use quicksilver::prelude::*;
use crate::prelude::*;
use super::{MouseState, LeftClickSystem, RightClickSystem, HoverSystem};
use crate::net::game_master_api::RestApiSystem;

const DOUBLE_CLICK_DELAY: i64 = 400_000; // [us]
const DOUBLE_CLICK_DISTANCE_2: f32 = 1000.0; // [browser pixel coordinates]

pub struct PointerManager<'a, 'b> {
    click_dispatcher: Dispatcher<'a, 'b>,
    hover_dispatcher: Dispatcher<'a, 'b>,
    tentative_left_click: Option<(Vector, Timestamp)>,
    definitive_click: Option<(Vector, MouseButton)>,
}

impl PointerManager<'_,'_> {
    pub fn init(mut world: &mut World) -> Self {

        world.insert(MouseState::default());

        let mut click_dispatcher = DispatcherBuilder::new()
            .with(LeftClickSystem, "lc", &[])
            .with(RightClickSystem, "rc", &[])
            .with(RestApiSystem, "rest", &["lc", "rc"])
            .build();
        click_dispatcher.setup(&mut world);

        let mut hover_dispatcher = DispatcherBuilder::new()
            .with(HoverSystem, "hov", &[])
            .build();
        hover_dispatcher.setup(&mut world);

        PointerManager {
            click_dispatcher: click_dispatcher,
            hover_dispatcher: hover_dispatcher,
            tentative_left_click: None,
            definitive_click: None,
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
    }

    pub fn move_pointer(&mut self, mut world: &mut World, position: &Vector) {
        Self::update(world, position, None);
        self.hover_dispatcher.dispatch(&mut world);
    }

    pub fn button_event(&mut self, now: Timestamp, pos: &Vector, button: MouseButton, state: ButtonState) {
        match state {
            ButtonState::Pressed => self.new_click(now, pos, button),
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

