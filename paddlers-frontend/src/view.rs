use std::collections::HashMap;
use crate::gui::input::UiView;
use specs::{Dispatcher, World};

#[derive(Default)]
pub struct ViewManager<'a,'b> {
    dispatchers: HashMap<UiView, Vec<Dispatcher<'a,'b>>>,
}

impl<'a,'b> ViewManager<'a,'b> {
    pub fn add_dispatcher(&mut self, v: UiView, d: Dispatcher<'a,'b>) {
        let entry = self.dispatchers.entry(v).or_insert(vec![]);
        entry.push(d);
    }
    pub fn update(&mut self, world: &mut World, view: UiView) {
        if let Some(view_dispatchers) = self.dispatchers.get_mut(&view) {
            for dispatcher in view_dispatchers.iter_mut() {
                dispatcher.dispatch(world);
            }
        }
    }
}