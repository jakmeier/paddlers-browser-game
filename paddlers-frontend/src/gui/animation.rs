use paddle::quicksilver_compat::*;
use paddle::*;
use paddlers_shared_lib::specification_types::Direction;
use specs::prelude::*;
use specs::storage::BTreeStorage;

#[derive(Component, Debug, Clone)]
#[storage(BTreeStorage)]
pub struct AnimationState {
    pub direction: Direction,
}

impl AnimationState {
    pub fn new(direction: Direction) -> Self {
        Self { direction }
    }
}

pub trait IDirection {
    fn from_vector(vec: &Vector) -> Self;
    fn unit_vector(&self) -> Vector;
}
impl IDirection for Direction {
    fn from_vector(vec: &Vector) -> Self {
        if about_equal(vec.x, vec.y) {
            return Direction::Undirected;
        }
        if vec.x.abs() > vec.y.abs() {
            if vec.x < 0.0 {
                Direction::West
            } else {
                Direction::East
            }
        } else {
            if vec.y < 0.0 {
                Direction::North
            } else {
                Direction::South
            }
        }
    }
    #[allow(dead_code)]
    fn unit_vector(&self) -> Vector {
        match self {
            Direction::Undirected => Vector::new(0, 0),
            Direction::North => Vector::new(0, -1),
            Direction::East => Vector::new(1, 0),
            Direction::South => Vector::new(0, 1),
            Direction::West => Vector::new(-1, 0),
        }
    }
}
