use crate::init::quicksilver_integration::Signal;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events
pub trait Frame {
    type Error;
    type State;
    type Graphics;
    type Event;
    type Signal;
    fn draw(
        &mut self,
        _state: &mut Self::State,
        _graphics: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn update(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        Ok(())
    }
    fn event(&mut self, _state: &mut Self::State, _event: &Self::Event) -> Result<(), Self::Error> {
        Ok(())
    }
    fn left_click(
        &mut self,
        _state: &mut Self::State,
        _pos: (i32, i32),
        _signals: &mut AbstractExperimentalSignalChannel<Self::Signal>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn right_click(
        &mut self,
        _state: &mut Self::State,
        _pos: (i32, i32),
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        Ok(())
    }
    fn enter(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        Ok(())
    }
}

type FrameRef<S, G, Ev, E, Sig> = Rc<RefCell<PositionedFrame<S, G, Ev, E, Sig>>>;

struct PositionedFrame<S, G, Ev, E, Sig> {
    #[allow(dead_code)]
    pos: (i32, i32),
    #[allow(dead_code)]
    size: (i32, i32),
    handler: Box<dyn Frame<State = S, Graphics = G, Event = Ev, Error = E, Signal = Sig>>,
}

/// The frame manager keeps track of which frames need to run
/// It routes events to active frames and can (de-)activate them
pub struct FrameManager<V: Hash + Eq + Copy, S, G, Ev, E, Sig> {
    view_frames: HashMap<V, Vec<FrameRef<S, G, Ev, E, Sig>>>,
    active_frames: Vec<FrameRef<S, G, Ev, E, Sig>>,
    all_frames: Vec<FrameRef<S, G, Ev, E, Sig>>,
    current_view: V,
    signals: AbstractExperimentalSignalChannel<Sig>,
}

/// The frames need a way to cross-communicate.
/// This is a prototype to see how it feels and maybe extend from it, or otherwise remove it again.
pub type AbstractExperimentalSignalChannel<Sig> = VecDeque<Sig>;
pub type ExperimentalSignalChannel = AbstractExperimentalSignalChannel<Signal>;
pub trait FrameSignal<Ev> {
    fn evaluate_signal(&self) -> Option<Ev>;
}

impl<V: Hash + Eq + Copy, S, G, Ev, E, Sig: FrameSignal<Ev>> FrameManager<V, S, G, Ev, E, Sig> {
    pub fn add_frame(
        &mut self,
        frame: Box<dyn Frame<State = S, Graphics = G, Event = Ev, Error = E, Signal = Sig>>,
        views: &[V],
        pos: (i32, i32),
        size: (i32, i32),
    ) {
        let frame_ref = Rc::new(RefCell::new(PositionedFrame {
            handler: frame,
            pos,
            size,
        }));
        let mut frame_displayed = false;
        for view in views {
            if view == &self.current_view {
                frame_displayed = true;
            }
            let vec = self.view_frames.entry(*view).or_insert(Vec::new());
            vec.push(frame_ref.clone());
        }
        if frame_displayed {
            self.active_frames.push(frame_ref.clone());
        }
        self.all_frames.push(frame_ref);
    }
    pub fn left_click(&mut self, state: &mut S, pos: (i32, i32)) -> Result<(), E> {
        // TODO: Check position
        for frame in &mut self.active_frames {
            frame
                .borrow_mut()
                .handler
                .left_click(state, pos, &mut self.signals)?;
        }
        Ok(())
    }
    pub fn right_click(&mut self, state: &mut S, pos: (i32, i32)) -> Result<(), E> {
        // TODO: Check position
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.right_click(state, pos)?;
        }
        Ok(())
    }
    /// Event that only reaches active frames
    pub fn event(&mut self, state: &mut S, event: &Ev) -> Result<(), E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.event(state, event)?;
        }
        Ok(())
    }
    /// Event that reaches all frames, regardless of activation status
    pub fn global_event(&mut self, state: &mut S, event: &Ev) -> Result<(), E> {
        for frame in &mut self.all_frames {
            frame.borrow_mut().handler.event(state, event)?;
        }
        Ok(())
    }
    pub fn update(&mut self, state: &mut S) -> Result<(), E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.update(state)?;
        }
        while let Some(signal) = self.signals.pop_front() {
            self.handle_signal(state, signal)?;
        }
        Ok(())
    }
    pub fn handle_signal(&mut self, state: &mut S, signal: Sig) -> Result<(), E> {
        if let Some(ev) = signal.evaluate_signal() {
            self.global_event(state, &ev)?;
        }
        Ok(())
    }
    pub fn draw(&mut self, state: &mut S, graphics: &mut G) -> Result<(), E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.draw(state, graphics)?;
        }
        Ok(())
    }
    pub fn set_view(&mut self, view: V, state: &mut S) -> Result<(), E> {
        if self.current_view == view {
            return Ok(());
        }
        self.current_view = view;
        self.reload(state)
    }
    fn clear_view(&mut self, state: &mut S) -> Result<(), E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.leave(state)?;
        }
        self.active_frames.clear();
        Ok(())
    }
    pub fn reload(&mut self, state: &mut S) -> Result<(), E> {
        self.clear_view(state)?;
        let frames = self
            .view_frames
            .get(&self.current_view)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        self.active_frames.extend_from_slice(frames);
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.enter(state)?;
        }
        Ok(())
    }
}

impl<V: Hash + Eq + Copy, S, G, Ev, E, Sig: FrameSignal<Ev>> FrameManager<V, S, G, Ev, E, Sig> {
    pub fn new(v: V) -> Self {
        FrameManager {
            active_frames: vec![],
            all_frames: vec![],
            current_view: v,
            view_frames: HashMap::new(),
            signals: VecDeque::new(),
        }
    }
}
