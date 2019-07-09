use quicksilver::prelude::*;
use quicksilver::graphics::Image;
use std::ops::Index;

/// Store of the sprites.
/// Cannot easily be in a component because Image is thread local.
#[derive(Clone, Debug)]
pub struct Sprites {
    grass: Image,
    water: Image,
    duck: Image,
}

impl Sprites {
    pub fn new()-> Asset<Self> {
        let grass_future = Image::load("textures/grass.png");
        let water_future = Image::load("textures/water.png");
        let duck_future = Image::load("ducks/yellow_sad.png");

        Asset::new(
            join_all(vec![
                grass_future,
                water_future,
                duck_future,
            ]).map(
                move |mut all| 
                {
                    Sprites { 
                        duck: all.remove(2),
                        water: all.remove(1),
                        grass: all.remove(0),
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
}

impl Index<SpriteIndex> for Sprites {
    type Output = Image;

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        match index {
            SpriteIndex::Grass => &self.grass,
            SpriteIndex::Water => &self.water,
            SpriteIndex::Duck => &self.duck,
        }
    }
}
