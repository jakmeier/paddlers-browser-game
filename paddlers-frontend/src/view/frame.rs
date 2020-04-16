use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events
pub trait Frame {
    type Error;
    type State;
    type Graphics;
    type Event;
    fn draw(&mut self, _state: &mut Self::State, _graphics: &mut Self::Graphics) -> Result<(),Self::Error> {
        Ok(())
    }
    fn update(&mut self, _state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
    fn event(&mut self, _state: &mut Self::State, _event: &Self::Event) -> Result<(),Self::Error> {
        Ok(())
    }
    fn left_click(&mut self, _state: &mut Self::State, _pos: (i32,i32)) -> Result<(),Self::Error> {
        Ok(())
    }
    fn right_click(&mut self, _state: &mut Self::State, _pos: (i32,i32)) -> Result<(),Self::Error> {
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
    fn enter(&mut self, _state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
}

type FrameRef<S,G,Ev,E> = Rc<RefCell<PositionedFrame<S,G,Ev,E>>>;

struct PositionedFrame<S,G,Ev,E> {
    pos: (i32,i32),
    size: (i32,i32),
    handler: Box<dyn Frame<State=S,Graphics=G,Event=Ev,Error=E>>,
}

/// The frame manager keeps track of which frames need to run
/// It routes events to active frames and can (de-)activate them
pub struct FrameManager<V: Hash + Eq+Copy,S,G,Ev,E> {
    view_frames: HashMap<V, Vec<FrameRef<S,G,Ev,E>>>,
    active_frames: Vec<FrameRef<S,G,Ev,E>>,
    current_view: V,
}
impl<V: Hash+Eq+Copy,S,G,Ev,E> FrameManager<V,S,G,Ev,E> {
    pub fn add_frame(
        &mut self, 
        frame: Box<dyn Frame<State=S,Graphics=G,Event=Ev,Error=E>>,
        views: &[V],
        pos: (i32,i32),
        size: (i32,i32),
    ) 
        {
        let frame_ref = Rc::new(RefCell::new(
            PositionedFrame {
                handler: frame,
                pos,
                size,
            }
        ));
        let mut frame_displayed = false;
        for view in views {
            if view == &self.current_view {
                frame_displayed = true;
            }
            let vec = self.view_frames.entry(*view).or_insert(Vec::new());
            vec.push(frame_ref.clone());
        }
        if frame_displayed {
            self.active_frames.push(frame_ref);
        }
    }
    pub fn left_click(&mut self, state: &mut S, pos: (i32,i32)) -> Result<(),E> {
        // TODO: Check position
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.left_click(state, pos)?;
        }
        Ok(())
    }
    pub fn right_click(&mut self, state: &mut S, pos: (i32,i32)) -> Result<(),E> {
        // TODO: Check position
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.right_click(state, pos)?;
        }
        Ok(())
    }
    pub fn event(&mut self, state: &mut S, event: &Ev) -> Result<(),E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.event(state, event)?;
        }
        Ok(())
    }
    pub fn global_event(&mut self, state: &mut S, event: &Ev) -> Result<(),E> {
        for frames in &mut self.view_frames.values_mut() {
            for frame in frames.as_mut_slice() {
                frame.borrow_mut().handler.event(state, event)?;
            }
        }
        Ok(())
    }
    pub fn update(&mut self, state: &mut S) -> Result<(),E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.update(state)?;
        }
        Ok(())
    }
    pub fn draw(&mut self, state: &mut S, graphics: &mut G) -> Result<(),E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.draw(state, graphics)?;
        }
        Ok(())
    }
    pub fn set_view(&mut self, view: V, state: &mut S) -> Result<(),E> {
        if self.current_view == view {
            return Ok(());
        }
        self.current_view = view;
        self.reload(state)
    }
    fn clear_view(&mut self, state: &mut S) -> Result<(),E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.leave(state)?;
        }
        self.active_frames.clear();
        Ok(())
    }
    pub fn reload(&mut self, state: &mut S) -> Result<(),E> {
        self.clear_view(state)?;
        let frames = self.view_frames.get(&self.current_view)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        self.active_frames.extend_from_slice(frames);
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.enter(state)?;
        }
        Ok(())
    }
}

impl<V: Hash+Eq+Copy,S,G,Ev,E> FrameManager<V,S,G,Ev,E> {
    pub fn new(v: V) -> Self {
        FrameManager {
            active_frames: vec![],
            current_view: v,
            view_frames: HashMap::new(),
        }
    }
}