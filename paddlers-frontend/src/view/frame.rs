pub mod new_frame;

use std::collections::HashMap;
use std::hash::Hash;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events
pub trait Frame {
    type Error;
    type State;
    type Graphics;
    type Event;
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

// struct PositionedFrame<S, G, Ev, E, Sig> {
//     #[allow(dead_code)]
//     pos: (i32, i32),
//     #[allow(dead_code)]
//     size: (i32, i32),
//     handler: Box<dyn Frame<State = S, Graphics = G, Event = Ev, Error = E, Signal = Sig>>,
// }

/// The frames need a way to cross-communicate.
/// This is a prototype to see how it feels and maybe extend from it, or otherwise remove it again.
pub trait FrameSignal<Ev> {
    fn evaluate_signal(&self) -> Option<Ev>;
}
