use quicksilver::prelude::*;
use specs::prelude::*;
use crate::game::{
    map::GlobalMapSharedState,
};

/// Consumes dragging movements and applies them to the game state.
pub struct DragSystem;

impl<'a> System<'a> for DragSystem {
    type SystemData = (
        Write<'a, Drag>,
        Write<'a, GlobalMapSharedState>,
    );

    fn run(&mut self, 
        (
            mut drag,
            mut map,
        ): Self::SystemData) 
    {
        if let Some((start, end)) = drag.0.take() {
            let v = end - start;
            map.drag(v * 0.02);
        }
    }
}


#[derive(Default, Clone, Copy)]
/// Represent a drag input waiting to be processed by the DragSystem.
/// Can only hold one drag at the time. 
/// When more drags are added, they are summarized to one single movement.
pub struct Drag(Option<(Vector,Vector)>);

impl Drag{
    pub fn add(&mut self, start: Vector, end: Vector) {
        if let Some(old) = self.0 {
            self.0 = Some((old.0, end));
        }
        else {
            self.0 = Some((start, end));
        }
    }
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
    pub fn clear(&mut self) {
        self.0 = None;
    }
}