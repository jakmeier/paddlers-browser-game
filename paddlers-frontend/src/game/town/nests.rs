use crate::game::town::*;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
/// A nest can provide space for some hobos and those can be invited to other towns
pub struct Nest {
    hobos: Vec<Entity>,
    capacity: usize,
}

impl Nest {
    pub fn new(capacity: usize) -> Self {
        Self {
            hobos: vec![],
            capacity,
        }
    }
    pub fn add_hobo(&mut self, hobo: Entity) {
        self.hobos.push(hobo);
    }
    pub fn clear_hobos(&mut self, lazy: &LazyUpdate) -> PadlResult<()> {
        if self.hobos.len() == 0 {
            PadlErrorCode::NestEmpty.usr()
        } else {
            let hobos = std::mem::replace(&mut self.hobos, vec![]);
            lazy.exec_mut(move |world| world.delete_entities(&hobos).expect("Delete failed"));
            Ok(())
        }
    }
}
