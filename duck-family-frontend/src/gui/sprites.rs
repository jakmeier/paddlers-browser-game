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
            Image::load("deco/flowers.png"),
            Image::load("resources/yellow_feather.png"),
            Image::load("resources/sticks.png"),
            Image::load("resources/logs.png"),
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
    Flowers,
    Feathers,
    Sticks,
    Logs,
}

impl Index<SpriteIndex> for Sprites {
    type Output = Image;

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        let i =
        match index {
            SpriteIndex::Grass => 0,
            SpriteIndex::Water => 1,
            SpriteIndex::Duck => 2,
            SpriteIndex::Flowers => 3,
            SpriteIndex::Feathers => 4,
            SpriteIndex::Sticks => 5,
            SpriteIndex::Logs => 6,
        };
        &self.img[i]
    }
}

pub trait WithSprite {
    fn sprite(&self) -> SpriteIndex;
}

use duck_family_api_lib::types::BuildingType;
impl WithSprite for BuildingType {
    fn sprite(&self) -> SpriteIndex {
        match self {
            BuildingType::BlueFlowers => SpriteIndex::Flowers,
            BuildingType::RedFlowers => SpriteIndex::Flowers,
        }
    }
}

use duck_family_api_lib::types::ResourceType;
impl WithSprite for ResourceType {
    fn sprite(&self) -> SpriteIndex {
        match self {
            ResourceType::Feathers => SpriteIndex::Feathers,
            ResourceType::Sticks => SpriteIndex::Sticks,
            ResourceType::Logs => SpriteIndex::Logs,
        }
    }
}