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
    // TODO: Get rid of Asset (everywhere, not just here)
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
            Image::load("gui/map_button.png"),
            Image::load("gui/map_button_hov.png"),
            Image::load("buildings/shack.png"),
            Image::load("ducks/roger_front.png"),
            Image::load("ducks/roger_back.png"),
            Image::load("ducks/roger.png"),
            Image::load("gui/leaves/50px_bot.png"),
            Image::load("gui/leaves/50px_mid.png"),
            Image::load("gui/leaves/50px_top.png"),
            Image::load("gui/leaves/leaves.png"),
            Image::load("gui/town_button.png"),
            Image::load("gui/town_button_hov.png"),
            Image::load("gui/steps.png"),
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
    MapButton,
    MapButtonHov,
    TownButton,
    TownButtonHov,
    Shack,
    DuckSteps,
}

#[derive(Debug, Clone, Copy)]
pub enum DirectedSprite {
    Hero,
    VerticalLeaves,
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
                SingleSprite::MapButton => 17,
                SingleSprite::MapButtonHov => 18,
                SingleSprite::Shack => 19,
                SingleSprite::TownButton => 27,
                SingleSprite::TownButtonHov => 28,
                SingleSprite::DuckSteps => 29,
            },
            SpriteIndex::Directed(j,d) => match (j,d) {
                (DirectedSprite::Hero, Direction::South) => 20,
                (DirectedSprite::Hero, Direction::North) => 21,
                (DirectedSprite::Hero, _) => 22,
                (DirectedSprite::VerticalLeaves, Direction::South) => 23,
                (DirectedSprite::VerticalLeaves, Direction::Undirected) => 24,
                (DirectedSprite::VerticalLeaves, Direction::North) => 25,
                (DirectedSprite::VerticalLeaves, _) => 26,
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