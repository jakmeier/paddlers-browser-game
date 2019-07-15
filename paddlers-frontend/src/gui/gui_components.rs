//! High-level GUI components that may be related to game logic as far as necessary

use paddlers_api_lib::types::ResourceType;
use quicksilver::prelude::*; 
use crate::gui::{
    sprites::{SpriteIndex, Sprites, WithSprite},
    utils::*,
    z::*,
};

pub enum TableRow {
    TextWithImage(String, SpriteIndex),
}

pub fn draw_table(
    window: &mut Window, 
    sprites: &mut Asset<Sprites>, 
    table: &[TableRow], 
    max_area: &Rectangle, 
    font: &mut Asset<Font>, 
    max_row_height: f32, 
    z: i32
) -> Result<()> 
{
    let row_height = max_row_height.min( max_area.height() / table.len() as f32);
    let font_h = row_height* 0.9;
    let img_s = row_height;
    let margin = 10.0;

    let mut line = Rectangle::new(max_area.pos, (max_area.width(), row_height));
    for row in table {
        match row {
            TableRow::TextWithImage(text, img) => {
                let mut text_area = line.clone();
                text_area.size.x -= img_s - margin;
                text_area.size.y = font_h;
                text_area.pos.y += 2.0*(row_height - font_h); // something is fishy here, should be /2.0 but right now looks better with *2.0
                write_text(font, window, &text_area, z, FitStrategy::Center, text)?;
                let symbol = Rectangle::new(line.pos + (text_area.width(), 0.0).into(), (img_s, img_s));
                draw_static_image(sprites, window, &symbol, *img, z, FitStrategy::Center)?;
            }
        }
        line.pos.y += row_height;
    }
    Ok(())
}

pub fn draw_resources(
    window: &mut Window, 
    sprites: &mut Asset<Sprites>, 
    resis: &[(ResourceType, i64)], 
    max_area: &Rectangle, 
    font: &mut Asset<Font>, 
    z: i32
) -> Result<()> 
{
    // XXX This is quite specific. If this is used more flexible, consider refactoring.
    let cols = 3;
    let rows = (2 + resis.len()) / cols;
    let grid = max_area.grid(cols, rows);
    let max_img_area = Rectangle::new_sized((50,50));
    for ((rt, n), res_area) in resis.iter().zip(grid) {
        let mut img_area = max_img_area.fit_into(&res_area, FitStrategy::TopLeft);
        img_area.size.y = res_area.height();
        img_area.pos.x = img_area.pos.x + res_area.width() - img_area.width();
        let text_h = res_area.height().min(36.0);
        let text_area = Rectangle::new(  
            (res_area.pos.x, res_area.pos.y + (res_area.height() - text_h)/2.0),
            (res_area.size.x - img_area.width(), text_h)
        );
        draw_static_image(sprites, window, &img_area, rt.sprite(), z, FitStrategy::Center)?;
        write_text(font, window, &text_area, z+1, FitStrategy::Center, &n.to_string())?;
    }
    Ok(())
}


#[derive(Clone)]
struct UiElement<T: Clone> {
    display: RenderVariant,
    hover_display: Option<Vec<(ResourceType, i64)>>, 
    on_click: T,
}
#[derive(Clone)]
pub struct UiBox<T: Clone> {
    area: Rectangle,
    elements: Vec<UiElement<T>>,
    columns: usize,
    rows: usize,
    padding: f32,
    margin: f32,
}

impl<T: Clone> UiBox<T> {
    pub fn new(area: Rectangle, columns: usize, rows: usize) -> Self {
        UiBox {
            area: area,
            elements: vec![],
            columns: columns,
            rows: rows,
            padding: 5.0,
            margin: 10.0,
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, i: SpriteIndex, on_click: T) {
        self.add_el(
            UiElement { 
                display: RenderVariant::Img(i), 
                hover_display: None,
                on_click: on_click,
            }    
        );
    }

    #[allow(dead_code)]
    pub fn add_with_background_color(&mut self, i: SpriteIndex, col: Color, on_click: T) {
        self.add_el(
            UiElement { 
                display: RenderVariant::ImgWithColBackground(i, col),
                hover_display: None,
                on_click: on_click,
            }    
        );
    }

    pub fn add_with_background_color_and_cost(&mut self, i: SpriteIndex, col: Color, on_click: T, cost: Vec<(ResourceType, i64)>) {
        self.add_el(
            UiElement { 
                display: RenderVariant::ImgWithColBackground(i, col),
                hover_display: Some(cost),
                on_click: on_click,
            }    
        );
    }

    fn add_el(&mut self, el: UiElement<T>) {
        self.elements.push(el);
        if self.columns * self.rows < self.elements.len() {
            println!("Warning: Not all elements of the UI Area will be visible")
        }
    }


    pub fn draw(&mut self, window: &mut Window, sprites: &mut Asset<Sprites>, area: &Rectangle) -> Result<()> {
        self.area = *area;
        let grid = area.grid(self.columns, self.rows);

        for (el, draw_area) in self.elements.iter().zip(grid) {
            let img = 
            match el.display {
                RenderVariant::Img(img) => {
                    img
                },
                RenderVariant::ImgWithColBackground(img, col) => {
                    window.draw_ex(
                        &draw_area.padded(self.margin),
                        Col(col),
                        Transform::IDENTITY, 
                        Z_MENU_BOX_BUTTONS-1,
                    );
                    img
                },
                RenderVariant::ImgWithImgBackground(img, bkg) => {
                    draw_static_image(
                        sprites, 
                        window, 
                        &draw_area, 
                        bkg, 
                        Z_MENU_BOX_BUTTONS-1, 
                        FitStrategy::Center
                    )?;
                    img
                }
            };
            draw_static_image(
                sprites, 
                window, 
                &draw_area.padded(self.padding + self.margin), 
                img, 
                Z_MENU_BOX_BUTTONS, 
                FitStrategy::Center
            )?;
        }

        Ok(())
    }

    fn find_element_under_mouse(&self, mouse: impl Into<Vector>) -> Option<&UiElement<T>> {
              let dx = self.area.width() / self.columns as f32;
        let dy = self.area.height() / self.rows as f32;
        let pos = mouse.into() - self.area.pos;
        if pos.y < 0.0 || pos.x < 0.0 {
            return None;
        }
        let i = (pos.y / dy) as usize * self.columns + (pos.x / dx) as usize;
        return self.elements.get(i);
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> Option<T> {
        if let Some(el) = self.find_element_under_mouse(mouse) {
            Some(el.on_click.clone())
        }
        else {
            None
        }
    }

    pub fn draw_hover(&mut self, window: &mut Window, sprites: &mut Asset<Sprites>, font: &mut Asset<Font>, area: &Rectangle) -> Result<()> {
        let mouse = window.mouse().pos();
        if let Some(el) = self.find_element_under_mouse(mouse) {
            if let Some(cost) = &el.hover_display {
                draw_resources(window, sprites, &cost, area, font, Z_MENU_RESOURCES)?;
            }
        }
        Ok(())
    }
}