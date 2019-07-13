use quicksilver::prelude::*; 
use crate::gui::{
    sprites::{SpriteIndex, Sprites},
    z::*
};

#[derive(Copy, Clone, Debug)]
pub enum FitStrategy {
    TopLeft,
    Center
}

pub fn draw_static_image(asset: &mut Asset<Sprites>, window: &mut Window, max_area: &Rectangle, i: SpriteIndex, z: i32, fit_strat: FitStrategy) -> Result<()> {
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
            Transform::IDENTITY, 
            z
        );
        Ok(())
    })
}


pub trait JmrRectangle {
    fn padded(&self, padding_factor: f32) -> Rectangle;
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle;
    fn grid(&self, w: usize, h: usize) -> Grid ;
}

impl JmrRectangle for Rectangle{
    fn padded(&self, padding_factor: f32) -> Rectangle {
        Rectangle::new_sized(self.size() * padding_factor)
            .with_center(self.center())
    }
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
    fn grid(&self, w: usize, h: usize) -> Grid {
        let dx = self.width() / w as f32;
        let dy = self.height() / h as f32;
        Grid {
            base: Rectangle::new(self.pos, (dx,dy)),
            i: 0,
            x: w,
            y: h,
        }
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
            r.pos.x = self.base.pos.x + w * (i % y) as f32;
            r.pos.y = self.base.pos.y + h * (i / x) as f32;
            self.i += 1;
            Some(r)
        }
        else {
            None
        }
    }
}





#[derive(Clone)]
struct UiElement<T: Clone> {
    sprite: SpriteIndex,
    on_click: T,
}
#[derive(Clone)]
pub struct UiBox<T: Clone> {
    area: Rectangle,
    elements: Vec<UiElement<T>>,
    columns: usize,
    rows: usize,
}

impl<T: Clone> UiBox<T> {
    pub fn new(area: Rectangle, columns: usize, rows: usize) -> Self {
        UiBox {
            area: area,
            elements: vec![],
            columns: columns,
            rows: rows,
        }
    }

    pub fn add(&mut self, i: SpriteIndex, on_click: T) {
        self.elements.push(
            UiElement { sprite: i, on_click: on_click }    
        );
        if self.columns * self.rows < self.elements.len() {
            println!("Warning: Not all elements of the UI Area will be visible")
        }
    }


    pub fn draw(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {

        let grid = self.area.grid(self.columns, self.rows);

        for (el, draw_area) in self.elements.iter().zip(grid) {
            draw_static_image(
                sprites, 
                window, 
                &draw_area, 
                el.sprite, 
                Z_MENU_BOX_BUTTONS, 
                FitStrategy::Center
            )?;
        }

        Ok(())
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> Option<T> {
        let dx = self.area.width() / self.columns as f32;
        let dy = self.area.height() / self.rows as f32;
        let pos = mouse.into() - self.area.pos;
        if pos.y < 0.0 || pos.x < 0.0 {
            return None;
        }
        let i = (pos.y / dy) as usize * self.columns + (pos.x / dx) as usize;
        self.elements.get(i).map(|el| el.on_click.clone())
    }
}