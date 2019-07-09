use super::sprites::{Sprites, SpriteIndex};
use quicksilver::{
    geom::{Rectangle, Shape},
    graphics::{
        Background::{Img},
        Color,
    },
    lifecycle::{Window},
    Result,
};

#[derive(Debug)]
pub struct Town {
    map: TileMap,
}

type TileMap = [[TileType; Y]; X];
#[derive(Clone, Copy, Debug)]
enum TileType {
    EMPTY,
    LANE,
}

pub const X: usize = 23;
const Y: usize = 13;
pub const TOWN_RATIO: f32 = X as f32 / Y as f32;

impl Town {
    pub fn new() -> Self {
        let mut map = [[TileType::EMPTY; Y]; X];
        for x in 0..X {
            for y in 0..Y {
                if y == (Y - 1) / 2 {
                    map[x][y] = TileType::LANE;
                }
            }
        }
        Town {
            map: map,
        }
    }

    pub fn render(&self, window: &mut Window, sprites: &Sprites, tick: u32, unit_length: f32) -> Result<()> {
        let d = unit_length;
        window.clear(Color::WHITE)?;

        for (x, col) in self.map.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    TileType::EMPTY => {
                        // println!("Empty {} {}", x, y);
                        window.draw(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                            Img(&sprites[SpriteIndex::Grass]),
                        );
                    }

                    TileType::LANE => {
                        // println!("Lane {} {}", x, y);
                        let shifted = ((tick / 10) % (d as u32)) as i32;
                        window.draw(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d))
                            .translate((shifted,0)),
                            Img(&sprites[SpriteIndex::Water]),
                        );
                        // XXX: Hack only works for basic map
                        if x == 0 {
                            let x = -1;
                            window.draw(
                                &Rectangle::new((d * x as f32, d * y as f32), (d, d))
                                .translate((shifted,0)),
                                Img(&sprites[SpriteIndex::Water]),
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
}