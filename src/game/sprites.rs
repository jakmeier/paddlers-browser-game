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
        };
        &self.img[i]
    }
}
