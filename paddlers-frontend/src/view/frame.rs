use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events
pub trait Frame {
    type Error;
    type State;
    type Graphics;
    fn draw(&mut self, state: &mut Self::State, graphics: &mut Self::Graphics) -> Result<(),Self::Error> {
        Ok(())
    }
    fn update(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
    fn event(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
    fn hide(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
    fn show(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        Ok(())
    }
}

// struct FrameRef<S,G,E>(Rc<RefCell<PositionedFrame<S,G,E>>>);
type FrameRef<S,G,E> = Rc<RefCell<PositionedFrame<S,G,E>>>;

struct PositionedFrame<S,G,E> {
    pos: (i32,i32),
    size: (i32,i32),
    handler: Box<dyn Frame<State=S,Graphics=G,Error=E>>,
}

/// The frame manager keeps track of which frames need to run
/// It routes events to active frames and can (de-)activate them
pub struct FrameManager<V: Hash + Eq+Copy,S,G,E> {
    view_frames: HashMap<V, Vec<FrameRef<S,G,E>>>,
    active_frames: Vec<FrameRef<S,G,E>>,
    current_view: Option<V>,
}
impl<V: Hash+Eq+Copy,S,G,E> FrameManager<V,S,G,E> {
    pub fn add_frame(
        &mut self, 
        frame: Box<dyn Frame<State=S,Graphics=G,Error=E>>,
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
        for view in views {
            let mut vec = self.view_frames.entry(*view).or_insert(Vec::new());
            vec.push(frame_ref.clone());
        }
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
        if self.current_view.as_ref() == Some(&view) {
            return Ok(());
        }
        self.clear_view(state)?;
        let frames = self.view_frames.get(&view)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        self.active_frames.extend_from_slice(frames);
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.show(state)?;
        }
        self.current_view = Some(view);
        Ok(())
    }
    fn clear_view(&mut self, state: &mut S) -> Result<(),E> {
        for frame in &mut self.active_frames {
            frame.borrow_mut().handler.hide(state)?;
        }
        self.active_frames.clear();
        Ok(())
    }
}

impl<V: Hash+Eq+Copy,S,G,E> Default for  FrameManager<V,S,G,E> {
    fn default() -> Self {
        FrameManager {
            active_frames: vec![],
            current_view: None,
            view_frames: HashMap::new(),
        }
    }
}