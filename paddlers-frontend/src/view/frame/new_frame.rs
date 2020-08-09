//! WIP: transition form frame + frame manager to something with nuts activities
//! Whats now defiend in here should eventually go to its own crate. (maybe called Paddle, or paddlers-engine)
//! The (basic) interface should be simple and clean. Ideally, easy enough for someone who never programmed before to pick up. But at least easy enough for a Rust beginner.
//! If there is a need for direct communicaiton with nuts, it should still be possible. But that will then be a more advanced API for Rust experts only.
//!
//! One tricky thing is communication between activities.
//! Nuts gives a framework that is flexible but difficult to use. (Closures with black magic types are not very beginner friendly)
//! For Paddle, I want someting that can send just as easily as publish. But receiving should be easier.
//! Potentially desired receiver interface:
//!     paddle_register!(ActivityX, Type1, Type2, Type3, Type3, ...);
//!     fn event(&self: &ActivityX, ev: ActivityXEvent) {
//!         match ev {
//!             ActivityXEvent::Type1(data) => { ... },
//!             ActivityXEvent::Type2(data) => { ... },
//!             ActivityXEvent::Type3(data) => { ... },
//!         }
//!     }

use super::*;
use crate::prelude::{GameEvent, PadlEvent};
use nuts::*;
use quicksilver::lifecycle::Event;
use quicksilver::prelude::Window;

// #[derive(Clone, Copy)]
// pub enum ActivityDomain {
//     Global,
// }
// domain_enum!(ActivityDomain);

#[derive(Clone, Copy)]
pub enum Domain {
    Main,
}
domain_enum!(Domain);

pub struct LeftClick {
    pub pos: (i32, i32),
}
pub struct RightClick {
    pub pos: (i32, i32),
}

pub struct UpdateWorld {
    window: *mut Window,
}
pub struct DrawWorld {
    window: *mut Window,
}
pub struct WorldEvent {
    window: *mut Window,
    event: Event,
}
impl UpdateWorld {
    pub fn new(window: &mut Window) -> Self {
        Self {
            window: window as *mut Window,
        }
    }
    pub fn window(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
}
impl DrawWorld {
    pub fn new(window: &mut Window) -> Self {
        Self {
            window: window as *mut Window,
        }
    }
    pub fn window(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
}
impl WorldEvent {
    pub fn new(window: &mut Window, event: &Event) -> Self {
        Self {
            window: window as *mut Window,
            event: event.clone(),
        }
    }
    pub fn window(&mut self) -> &mut Window {
        unsafe { self.window.as_mut().unwrap() }
    }
    pub fn event(&self) -> Event {
        self.event.clone()
    }
}

/// Goes to active and inactive frames
struct GlobalEvent<Ev>(pub(crate) Ev);
/// Goes to active frames only
struct ActiveEvent<Ev>(pub(crate) Ev);

/// Share a PaddlEvent with all other activities in background and foreground
pub(crate) fn share(ev: PadlEvent) {
    nuts::publish(GlobalEvent(ev));
}

/// Share a PaddlEvent with all foreground activities
pub(crate) fn share_foreground(ev: PadlEvent) {
    nuts::publish(ActiveEvent(ev));
}

/// Send a GameEvent to the game event manager (replaces endpoints that were copied everywhere before)
pub fn game_event(ev: GameEvent) {
    nuts::publish(ev);
}

pub fn frame_to_activity<F>(frame: F) -> ActivityId<F>
where
    F: Frame<Graphics = Window> + Activity,
{
    let activity = nuts::new_domained_activity(frame, Domain::Main, false);

    activity.subscribe_domained(|a, d, _msg: &UpdateWorld| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.update(global_state) {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained_mut(|a: &mut F, d: &mut DomainState, msg: &mut DrawWorld| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        let window = msg.window();
        if let Err(e) = a.draw(global_state, window) {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained_masked(
        SubscriptionFilter::no_filter(),
        |a, d, msg: &GlobalEvent<F::Event>| {
            let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
            let err = a.event(global_state, &msg.0);
            if let Err(e) = err {
                nuts::publish(e);
            }
        },
    );

    activity.subscribe_domained(|a, d, msg: &ActiveEvent<F::Event>| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        let err = a.event(global_state, &msg.0);
        if let Err(e) = err {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained(|a, d, msg: &LeftClick| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.left_click(global_state, msg.pos) {
            nuts::publish(e);
        }
    });

    activity.subscribe_domained(|a, d, msg: &RightClick| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.right_click(global_state, msg.pos) {
            nuts::publish(e);
        }
    });

    activity.on_enter_domained(|a, d| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.enter(global_state) {
            nuts::publish(e);
        }
    });

    activity.on_leave_domained(|a, d| {
        let global_state: &mut F::State = d.try_get_mut().expect("Global state missing");
        if let Err(e) = a.leave(global_state) {
            nuts::publish(e);
        }
    });

    activity
}

// /// Calls nuts::draw() in every animation frame as managed by the browser. (Using requestAnimationFrame)
// pub fn auto_draw() {
//     stdweb::web::window().request_animation_frame(|_| crate::draw());
// }

// /// Calls nuts::update() in intervals managed by the browser. (Using setInterval)
// /// The defined interval will be the maximum number of calls but may be less if the computation takes too long
// pub fn auto_update(delay_ms: u32) {
//     let callback = crate::update;

//     js!( @(no_return)
//         setInterval( @{callback}, @{delay_ms});
//     );
// }

// Switches between views by activating and deactivating activities
pub struct ViewManager<V> {
    views_to_activities: HashMap<V, Vec<UncheckedActivityId>>,
    current_view: V,
}

impl<V: Hash + Eq + Copy> ViewManager<V> {
    pub fn new(v: V) -> Self {
        Self {
            views_to_activities: HashMap::new(),
            current_view: v,
        }
    }

    pub fn link_activity_to_view(&mut self, aid: impl Into<UncheckedActivityId>, view: V) {
        self.views_to_activities
            .entry(view)
            .or_default()
            .push(aid.into());
    }
    /// Activity with position and associated view(s)
    // TODO: Does this interface need to be simplified?
    pub fn add_frame<S, Ev, E>(
        &mut self,
        frame: impl Frame<State = S, Graphics = Window, Event = Ev, Error = E> + nuts::Activity,
        views: &[V],
        _pos: (i32, i32),
        _size: (i32, i32),
    ) {
        let aid: ActivityId<_> = new_frame::frame_to_activity(frame).into();
        for view in views {
            if view == &self.current_view {
                nuts::set_active(aid, view == &self.current_view);
            }
            self.link_activity_to_view(aid, *view);
        }
    }
    pub fn set_view(&mut self, view: V) {
        if self.current_view == view {
            return;
        }
        let _before = self
            .views_to_activities
            .entry(self.current_view)
            .or_default();
        let _after: &Vec<_> = self.views_to_activities.entry(view).or_default();
        let after = &self.views_to_activities[&view];
        let before = &self.views_to_activities[&self.current_view];
        // deactivate all in before that are not in after
        for b in before {
            if !after.iter().any(|a| a == b) {
                nuts::set_active(*b, false);
            }
        }
        // activate all in after (activating when already active does nothing)
        for a in after {
            nuts::set_active(*a, true);
        }
        self.current_view = view;
    }
}
