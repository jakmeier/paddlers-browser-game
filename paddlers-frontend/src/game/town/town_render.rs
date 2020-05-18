use super::*;
use quicksilver::graphics::{Drawable, Mesh};
use quicksilver::prelude::*;

impl Town {
    pub fn render_background(
        &self,
        mesh: &mut Mesh,
        sprites: &mut Sprites,
        unit_length: f32,
    ) -> Result<()> {
        let d = unit_length;

        for (x, col) in self.map.0.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    TileType::EMPTY | TileType::BUILDING(_) => {
                        let img = sprites.index(SpriteIndex::Simple(SingleSprite::Grass));
                        let bkg = Img(&img);
                        let rect = Rectangle::new((d * x as f32, d * y as f32), (d, d));
                        rect.draw(mesh, bkg.into(), Transform::IDENTITY, Z_TEXTURE);
                    }
                    TileType::LANE => {
                        // Nothing cacheable for lane
                    }
                }
            }
        }
        Ok(())
    }
    pub fn render(
        &self,
        window: &mut Window,
        sprites: &mut Sprites,
        tick: u32,
        unit_length: f32,
    ) -> Result<()> {
        let d = unit_length;

        for (x, col) in self.map.0.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    TileType::EMPTY | TileType::BUILDING(_) => {
                        // Already drawn in background
                    }

                    TileType::LANE => {
                        // println!("Lane {} {}", x, y);
                        let shifted = ((tick / 10) % (d as u32)) as i32;
                        let t = Transform::translate((shifted, 0));
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                            Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Water))),
                            t,
                            Z_TEXTURE,
                        );
                        // XXX: Hack only works for basic map
                        if x == 0 {
                            let x = -1;
                            window.draw_ex(
                                &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                                Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Water))),
                                t,
                                Z_TEXTURE,
                            );
                        }
                        let grass_top_img =
                            &sprites.index(SpriteIndex::Simple(SingleSprite::GrassTop));
                        let h = d / grass_top_img.area().width() * grass_top_img.area().height();
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32 + d - h), (d, h)),
                            Img(grass_top_img),
                            Transform::IDENTITY,
                            Z_VISITOR + 1, // This should be above visitors
                        );
                        let grass_bot_img =
                            &sprites.index(SpriteIndex::Simple(SingleSprite::GrassBot));
                        let h = d / grass_bot_img.area().width() * grass_bot_img.area().height();
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, h)),
                            Img(grass_bot_img),
                            Transform::IDENTITY,
                            Z_TEXTURE + 1,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub fn shadow_rectified_circle(
        resolution: ScreenResolution,
        window: &mut Window,
        center: impl Into<Vector>,
        radius: f32,
    ) {
        let tile = resolution.tile(center);
        for (x, y) in Town::tiles_in_rectified_circle(tile, radius) {
            Self::shadow_tile(resolution, window, (x, y));
        }
    }

    fn shadow_tile(resolution: ScreenResolution, window: &mut Window, coordinates: (usize, usize)) {
        let shadow_col = Color {
            r: 1.0,
            g: 1.0,
            b: 0.5,
            a: 0.3,
        };
        let (x, y) = coordinates;
        let ul = resolution.unit_length();
        let pos = (x as f32 * ul, y as f32 * ul);
        let size = (ul, ul);
        let area = Rectangle::new(pos, size);
        window.draw_ex(&area, Col(shadow_col), Transform::IDENTITY, Z_TILE_SHADOW);
    }
}
