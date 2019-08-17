use quicksilver::prelude::*;
use quicksilver::graphics::Image;
use std::ops::Index;
use crate::gui::utils::*;

/// Store of the sprites.
/// Cannot easily be in a component because Image is thread local.
#[derive(Clone, Debug)]
pub struct Sprites {
    img: Vec<Image>,
}


impl Sprites {
    pub fn new()-> Asset<Self> {
        let futures = vec![
            Image::load("textures/grass.png"),
            Image::load("textures/water.png"),
            Image::load("ducks/yellow_sad.png"),
            Image::load("plants/red_flowers.png"),
            Image::load("plants/blue_flowers.png"),
            Image::load("resources/yellow_feather.png"),
            Image::load("resources/sticks.png"),
            Image::load("resources/logs.png"),
            Image::load("happy.png"),
            Image::load("ambience.png"),
            Image::load("plants/tree.png"),
            Image::load("plants/sapling.png"),
            Image::load("plants/young_tree.png"),
            Image::load("ducks/camo_duck_sad.png"),
            Image::load("ducks/white_duck_sad.png"),
            Image::load("buildings/bundling_station.png"),
            Image::load("buildings/saw_mill.png"),
            Image::load("ducks/roger_front.png"),
            Image::load("ducks/roger_back.png"),
            Image::load("ducks/roger.png"),
        ];

        Asset::new(
            join_all(futures).map(
                move |loaded| 
                {
                    Sprites { 
                        img: loaded
                    }
                }
            ),
        )
    }
}

#[derive(Debug, Clone, Copy)]
/// An instance of a SpriteIndex is a key for a specific sprite (PNG)
pub enum SpriteIndex {
    // Multi-sprite images 
    Simple(SingleSprite),
    Directed(DirectedSprite, Direction),
}

/// An instance of a SpriteSet summarizes one or many sprites that show 
/// the same object in different states / from different angles
#[derive(Debug, Clone, Copy)]
pub enum SpriteSet {
    Simple(SingleSprite),
    Directed(DirectedSprite),
}

impl SpriteSet {
    pub fn default(&self) -> SpriteIndex {
        match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, Direction::Undirected),
        }
    }
    pub fn directed(&self, d: &Direction) -> (SpriteIndex, Transform) {
        let i = match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, *d),
        };
        let t = match d {
            Direction::East => { horizontal_flip() },
            _ => { Transform::IDENTITY },
        };
        (i,t)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SingleSprite {
    Grass,
    Water,
    Duck,
    RedFlowers,
    BlueFlowers,
    Feathers,
    Sticks,
    Logs,
    Heart,
    Ambience,
    Tree,
    Sapling, 
    YoungTree,
    CamoDuck,
    WhiteDuck,
    BundlingStation,
    SawMill,
}

#[derive(Debug, Clone, Copy)]
pub enum DirectedSprite {
    Hero,
}

impl Index<SpriteIndex> for Sprites {
    type Output = Image;

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        let i =
        match index {
            SpriteIndex::Simple(j) => match j {
                SingleSprite::Grass => 0,
                SingleSprite::Water => 1,
                SingleSprite::Duck => 2,
                SingleSprite::RedFlowers => 3,
                SingleSprite::BlueFlowers => 4,
                SingleSprite::Feathers => 5,
                SingleSprite::Sticks => 6,
                SingleSprite::Logs => 7,
                SingleSprite::Heart => 8,
                SingleSprite::Ambience => 9,
                SingleSprite::Tree => 10,
                SingleSprite::Sapling => 11,
                SingleSprite::YoungTree => 12,
                SingleSprite::CamoDuck => 13,
                SingleSprite::WhiteDuck => 14,
                SingleSprite::BundlingStation => 15,
                SingleSprite::SawMill => 16,
            },
            SpriteIndex::Directed(j,d) => match (j,d) {
                (DirectedSprite::Hero, Direction::South) => 17,
                (DirectedSprite::Hero, Direction::North) => 18,
                (DirectedSprite::Hero, _) => 19,
            },
        };
        &self.img[i]
    }
}

pub trait WithSprite {
    fn sprite(&self) -> SpriteSet;
}

use paddlers_shared_lib::models::BuildingType;
impl WithSprite for BuildingType {
    fn sprite(&self) -> SpriteSet {
        match self {
            BuildingType::BlueFlowers => SpriteSet::Simple(SingleSprite::BlueFlowers),
            BuildingType::RedFlowers => SpriteSet::Simple(SingleSprite::RedFlowers),
            BuildingType::Tree => SpriteSet::Simple(SingleSprite::Sapling),
            BuildingType::BundlingStation => SpriteSet::Simple(SingleSprite::BundlingStation),
            BuildingType::SawMill => SpriteSet::Simple(SingleSprite::SawMill),
        }
    }
}

use paddlers_shared_lib::models::ResourceType;
impl WithSprite for ResourceType {
    fn sprite(&self) -> SpriteSet {
        match self {
            ResourceType::Feathers => SpriteSet::Simple(SingleSprite::Feathers),
            ResourceType::Sticks => SpriteSet::Simple(SingleSprite::Sticks),
            ResourceType::Logs => SpriteSet::Simple(SingleSprite::Logs),
        }
    }
}

pub fn tree_sprite(score: usize) -> SpriteSet {
    match score {
        s if s <= 2 => SpriteSet::Simple(SingleSprite::Sapling),
        s if s <= 9 => SpriteSet::Simple(SingleSprite::YoungTree),
        _ => SpriteSet::Simple(SingleSprite::Tree),
    }
}