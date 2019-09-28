use specs::storage::BTreeStorage;
use specs::prelude::*;
use quicksilver::geom::*;
use crate::gui::utils::*;

#[derive(Component, Debug, Clone)]
#[storage(BTreeStorage)]
pub struct AnimationState {
    pub direction: Direction,
}

impl Direction {
    pub fn from_vector(vec: &Vector) -> Option<Self> {
        if vec.x != 0.0 && vec.y == 0.0 {
            Some(
                if vec.x < 0.0 { Direction::West }
                else { Direction::East }
            )
        } else if vec.x == 0.0 && vec.y != 0.0 {
            Some(
                if vec.y < 0.0 { Direction::North }
                else { Direction::South }
            )
        } else if vec.x == 0.0 && vec.y == 0.0 {
            Some( Direction::Undirected )
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn unit_vector(&self) -> Vector {
        match self {
            Direction::Undirected => Vector::new(0,0),
            Direction::North => Vector::new(0,-1),
            Direction::East => Vector::new(1,0),
            Direction::South => Vector::new(0,1),
            Direction::West => Vector::new(-1,0),
        }
    }
}