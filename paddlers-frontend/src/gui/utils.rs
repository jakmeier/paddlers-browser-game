//! Generic GUI Utilities
//! Keep the dependencies to a minimum,
//! no connection with game logic in here

use quicksilver::prelude::*; 
use crate::gui::sprites::{SpriteIndex, Sprites};
use crate::gui::animation::{AnimationState, Direction};


pub const BLACK: Color =    Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
pub const GREEN: Color =    Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 };
pub const LIME_GREEN: Color =    Color { r: 0.6, g: 0.9, b: 0.25, a: 1.0 };
pub const GREY: Color =    Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
pub const WHITE: Color =    Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

#[derive(Debug, Clone)]
pub enum RenderVariant {
    #[allow(dead_code)]
    Img(SpriteIndex),
    ImgWithImgBackground(SpriteIndex, SpriteIndex),
    ImgWithColBackground(SpriteIndex, Color),
}

#[derive(Copy, Clone, Debug)]
pub enum FitStrategy {
    TopLeft,
    Center
}

pub fn draw_animated_sprite(asset: &mut Asset<Sprites>, window: &mut Window, max_area: &Rectangle, i: SpriteIndex, z: i32, fit_strat: FitStrategy, animation_state: &AnimationState) -> Result<()> {
    let transform = match animation_state.direction {
        Direction::Undirected | Direction::West
            => { Transform::IDENTITY },
        Direction::East 
            => { horizontal_flip() },
        Direction::North  
            => { Transform::IDENTITY },
        Direction::South
            => { Transform::IDENTITY },
    };
    draw_image(asset, window, max_area, i, z, fit_strat, transform)
}
pub fn draw_static_image(asset: &mut Asset<Sprites>, window: &mut Window, max_area: &Rectangle, i: SpriteIndex, z: i32, fit_strat: FitStrategy) -> Result<()> {
    draw_image(asset, window, max_area, i, z, fit_strat, Transform::IDENTITY)
}
fn draw_image(asset: &mut Asset<Sprites>, window: &mut Window, max_area: &Rectangle, i: SpriteIndex, z: i32, fit_strat: FitStrategy, transform: Transform) -> Result<()> {
    asset.execute( |sprites| {
        let img = &sprites[i];
        let mut area = *max_area;
        let img_slope = img.area().height() / img.area().width();
        if img_slope < area.height() / area.width() {
            // high image
            area.size.y = area.width() * img_slope;
            match fit_strat {
                FitStrategy::Center => {
                    area = area.translate((0,(max_area.height() - area.height())/2.0));
                },
                FitStrategy::TopLeft => {},
            }
        } else {
            area.size.x = area.height() / img_slope;
            match fit_strat {
                FitStrategy::Center => {
                    area = area.translate(((max_area.width() - area.width())/2.0, 0.0));
                },
                FitStrategy::TopLeft => {},
            }
        }
        
        window.draw_ex(
            &area,
            Img(img),
            transform, 
            z
        );
        Ok(())
    })
}

pub fn write_text(asset: &mut Asset<Font>, window: &mut Window, max_area: &Rectangle, z: i32, fit_strat: FitStrategy, text: &str) -> Result<f32> {
    let mut res = 0.0;
    asset.execute(
        |font| {
            let style = FontStyle::new(max_area.height(), Color::BLACK);
            let img = font.render(text, &style).unwrap();
            let area = img.area().fit_into(max_area, fit_strat);
            window.draw_ex(&area, Img(&img), Transform::IDENTITY, z);
            res = area.width();
            Ok(())
        }
    )?;
    Ok(res)
}


pub trait JmrRectangle {
    fn shrink_to_center(&self, shrink_to_center: f32) -> Rectangle;
    fn padded(&self, padding: f32) -> Rectangle;
    fn fit_into(self, frame: &Rectangle, fit_strat: FitStrategy) -> Rectangle;
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle;
    fn grid(&self, cols: usize, rows: usize) -> Grid ;
    fn cut_horizontal(&self, h: f32) -> (Rectangle, Rectangle);
}

impl JmrRectangle for Rectangle{
    fn shrink_to_center(&self, shrink_to_center: f32) -> Rectangle {
        Rectangle::new_sized(self.size() * shrink_to_center)
            .with_center(self.center())
    }
    /// Padds constant pixels around the rectangle
    fn padded(&self, padding: f32) -> Rectangle {
        Rectangle::new_sized(self.size() - (2.0*padding, 2.0*padding).into() )
            .with_center(self.center())
    }
    /// Shrinks and moves the rectangle to fit within the given frame, without changing proportions 
    fn fit_into(mut self, frame: &Rectangle, fit_strat: FitStrategy) -> Rectangle {
        let stretch_factor = ( frame.width() / self.width() ).min( frame.height() / self.height() );
        if stretch_factor < 1.0 {
            self.size *= stretch_factor;
        }
        match fit_strat {
            FitStrategy::TopLeft => self.pos = frame.pos,
            FitStrategy::Center => { 
                self.pos = frame.pos; 
                self.pos = frame.pos + frame.center() - self.center() 
            },
        }
        self
    }
    /// Finds the largest square that fits into the given rectangle 
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle {
        let s = self.width().min(self.height());
        let mut rect = Rectangle::new(self.pos, (s,s));
        match fit_strat {
            FitStrategy::Center => {
                rect = rect.translate(((self.width() - rect.width())/2.0, 0.0));
                rect = rect.translate((0.0, (self.height() - rect.height())/2.0));
            },
            FitStrategy::TopLeft => {},
        }
        rect
    }
    fn grid(&self, cols: usize, rows: usize) -> Grid {
        let dx = self.width() / cols as f32;
        let dy = self.height() / rows as f32;
        Grid {
            base: Rectangle::new(self.pos, (dx,dy)),
            i: 0,
            x: cols,
            y: rows,
        }
    }
    fn cut_horizontal(&self, h: f32) -> (Rectangle, Rectangle) {
        let mut top = self.clone();
        top.size.y = h;
        let mut bottom = self.clone();
        bottom.size.y -= h;
        bottom.pos.y += h;
        (top, bottom)
    }
}

pub struct Grid {
    base: Rectangle,
    i: usize,
    x: usize,
    y: usize,
}
impl Iterator for Grid {
    type Item = Rectangle;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;
        let i = self.i;
        if i < x*y {
            let w = self.base.width();
            let h = self.base.height();
            let mut r = self.base.clone();
            r.pos.x = self.base.pos.x + w * (i % x) as f32;
            r.pos.y = self.base.pos.y + h * (i / x) as f32;
            self.i += 1;
            Some(r)
        }
        else {
            None
        }
    }
}

pub fn horizontal_flip() -> Transform {
    Transform::from_array(
        [[-1f32, 0f32, 0f32],
         [0f32, 1f32, 0f32],
         [0f32, 0f32, 1f32]]
    )
}
