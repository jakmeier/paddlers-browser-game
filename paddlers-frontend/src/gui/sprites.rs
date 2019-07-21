use quicksilver::prelude::*;
use quicksilver::graphics::Image;
use std::ops::Index;

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
            Image::load("ducks/roger.png"),
            Image::load("ducks/camo_duck_sad.png"),
            Image::load("ducks/white_duck_sad.png"),
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
pub enum SpriteIndex {
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
    Hero,
    CamoDuck,
    WhiteDuck,
}

impl Index<SpriteIndex> for Sprites {
    type Output = Image;

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        let i =
        match index {
            SpriteIndex::Grass => 0,
            SpriteIndex::Water => 1,
            SpriteIndex::Duck => 2,
            SpriteIndex::RedFlowers => 3,
            SpriteIndex::BlueFlowers => 4,
            SpriteIndex::Feathers => 5,
            SpriteIndex::Sticks => 6,
            SpriteIndex::Logs => 7,
            SpriteIndex::Heart => 8,
            SpriteIndex::Ambience => 9,
            SpriteIndex::Tree => 10,
            SpriteIndex::Sapling => 11,
            SpriteIndex::YoungTree => 12,
            SpriteIndex::Hero => 13,
            SpriteIndex::CamoDuck => 14,
            SpriteIndex::WhiteDuck => 15,
        };
        &self.img[i]
    }
}

pub trait WithSprite {
    fn sprite(&self) -> SpriteIndex;
}

use paddlers_shared_lib::models::BuildingType;
impl WithSprite for BuildingType {
    fn sprite(&self) -> SpriteIndex {
        match self {
            BuildingType::BlueFlowers => SpriteIndex::BlueFlowers,
            BuildingType::RedFlowers => SpriteIndex::RedFlowers,
            BuildingType::Tree => SpriteIndex::Tree,
        }
    }
}

use paddlers_shared_lib::models::ResourceType;
impl WithSprite for ResourceType {
    fn sprite(&self) -> SpriteIndex {
        match self {
            ResourceType::Feathers => SpriteIndex::Feathers,
            ResourceType::Sticks => SpriteIndex::Sticks,
            ResourceType::Logs => SpriteIndex::Logs,
        }
    }
}