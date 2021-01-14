use crate::resolution::TOWN_TILE_S;

use super::*;
use paddle::quicksilver_compat::*;
use paddle::DisplayArea;
use paddle::*;
use std::f32::consts::PI;

impl Town {
    pub fn render(
        &self,
        window: &mut DisplayArea,
        sprites: &mut Sprites,
        tick: u32,
    ) -> PadlResult<()> {
        let d = TOWN_TILE_S as f32;

        for (x, col) in self.map.0.iter().enumerate() {
            for (y, _tile) in col.iter().enumerate() {
                if y == TOWN_LANE_Y {
                    let shifted = ((tick / 10) % (d as u32)) as i32;
                    let t = Transform::translate((shifted, 0));
                    window.draw_ex(
                        &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                        Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Water))),
                        t,
                        Z_TEXTURE,
                    );
                    if x == 0 {
                        let x = -1;
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                            Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Water))),
                            t,
                            Z_TEXTURE,
                        );
                    }
                    let grass_top_img = &sprites.index(SpriteIndex::Simple(SingleSprite::GrassTop));
                    let h = d / 200.0 * 30.0;
                    window.draw_ex(
                        &Rectangle::new((d * x as f32, d * y as f32 + d - h), (d, h)),
                        Img(grass_top_img),
                        Transform::IDENTITY,
                        Z_VISITOR + 1, // This should be above visitors
                    );
                    let grass_bot_img = &sprites.index(SpriteIndex::Simple(SingleSprite::GrassBot));
                    let h = d / 200.0 * 42.0;
                    window.draw_ex(
                        &Rectangle::new((d * x as f32, d * y as f32), (d, h)),
                        Img(grass_bot_img),
                        Transform::IDENTITY,
                        Z_TEXTURE + 1,
                    );
                } else {
                    let img = sprites.index(SpriteIndex::Simple(SingleSprite::Grass));
                    let rect = Rectangle::new((d * x as f32, d * y as f32), (d, d));
                    window.draw(&rect, &img);
                }
            }
        }
        Ok(())
    }

    pub fn shadow_rectified_circle(
        window: &mut DisplayArea,
        center: impl Into<Vector>,
        radius: f32,
    ) {
        let center_tile = tiling::tile(center);
        let tiles = Town::tiles_in_rectified_circle(center_tile, radius);
        let shadow_col = Color {
            r: 1.0,
            g: 1.0,
            b: 0.5,
            a: 0.3,
        };
        Self::shadow_tiles(window, &tiles, shadow_col);
    }

    pub fn shadow_tiles(window: &mut DisplayArea, tiles: &[TileIndex], shadow_col: Color) {
        let ul = TOWN_TILE_S as f32;
        let size = (ul, ul);
        let mut area = Rectangle::new_sized(size);
        for (x, y) in tiles {
            area.pos = (*x as f32 * ul, *y as f32 * ul).into();
            window.draw_ex(&area, shadow_col, Transform::IDENTITY, Z_TILE_SHADOW);
        }
    }
}

/// Draws a simple animation around the border of a specified area
pub fn draw_shiny_border(window: &mut DisplayArea, area: Rectangle, tick: u32) {
    let animation_length = 200;
    let side_length = animation_length / 4;
    let side_progress = (tick % side_length) as f32 / side_length as f32;
    // Use non-linear scale to create smooth acceleration effect
    let side_progress = (side_progress * PI).cos() / 2.0 + 0.5;

    match (tick / side_length) % 2 {
        0 => draw_dot(window, relative_top(area, side_progress)),
        1 => draw_dot(window, relative_right(area, side_progress)),
        _ => unreachable!(),
    }

    match (tick / side_length) % 2 {
        0 => draw_dot(window, relative_bottom(area, side_progress)),
        1 => draw_dot(window, relative_left(area, side_progress)),
        _ => unreachable!(),
    }
}

fn draw_dot(window: &mut DisplayArea, p: Vector) {
    let dot = Circle::new(p, 3.0);
    let dot_col = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.8,
    };
    window.draw_ex(&dot, dot_col, Transform::IDENTITY, Z_UNIT_UI_HINT);
}

const fn relative_top(area: Rectangle, r: f32) -> Vector {
    Vector {
        x: area.pos.x + area.size.x * r,
        y: area.pos.y,
    }
}
const fn relative_right(area: Rectangle, r: f32) -> Vector {
    Vector {
        x: area.pos.x + area.size.x,
        y: area.pos.y + area.size.y * r,
    }
}
const fn relative_bottom(area: Rectangle, r: f32) -> Vector {
    Vector {
        x: area.pos.x + area.size.x * (1.0 - r),
        y: area.pos.y + area.size.y,
    }
}
const fn relative_left(area: Rectangle, r: f32) -> Vector {
    Vector {
        x: area.pos.x,
        y: area.pos.y + area.size.y * (1.0 - r),
    }
}
