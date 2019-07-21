use specs::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct NetObj {
    pub id: i64, 
    // Maybe one could add a type field to make it unique. But not sure right now if it is even necessary.
}