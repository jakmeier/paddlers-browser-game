//! High-level GUI components that may be related to game logic as far as necessary

use crate::prelude::*;
use quicksilver::prelude::*; 
use crate::gui::{
    sprites::*,
    utils::*,
    z::*,
};

pub enum TableRow<'a> {
    Text(String),
    TextWithImage(String, SpriteIndex),
    UiBoxWithEntities(&'a mut UiBox<specs::Entity>),
    UiBoxWithBuildings(&'a mut UiBox<BuildingType>),
}

pub fn draw_table(
    window: &mut Window, 
    sprites: &mut Asset<Sprites>, 
    table: &mut [TableRow], 
    max_area: &Rectangle, 
    font: &mut Asset<Font>, 
    max_row_height: f32, 
    z: i32
) -> Result<()> 
{
    let total_rows = row_count(table);
    let row_height = max_row_height.min( max_area.height() / total_rows as f32);
    let font_h = row_height* 0.9;
    let img_s = row_height * 0.95;
    let margin = 10.0;

    let mut line = Rectangle::new(max_area.pos, (max_area.width(), row_height));
    for row in table {
        match row {
            TableRow::Text(text) => {
                let mut text_area = line.clone();
                text_area.size.y = font_h;
                write_text(font, window, &text_area, z, FitStrategy::Center, text)?;
                line.pos.y += row_height;
            }
            TableRow::TextWithImage(text, img) => {
                let symbol = Rectangle::new(line.pos, (img_s, img_s));
                let mut text_area = line.clone();
                let shift_x = img_s + margin;
                text_area.size.x -= shift_x;
                text_area.pos.x += shift_x;
                text_area.size.y = font_h;
                text_area.pos.y += row_height - font_h; // something is fishy here, should be /2.0 but right now looks better without
                write_text(font, window, &text_area, z, FitStrategy::Center, text)?;
                draw_static_image(sprites, window, &symbol, *img, z, FitStrategy::Center)?;
                line.pos.y += row_height;
            }
            TableRow::UiBoxWithEntities(uib) => {
                let mut area = line.clone();
                area.size.y = area.size.y * (uib.rows * 2) as f32;
                uib.draw(window, sprites, &area)?;
                line.pos.y += area.size.y;
            }
            TableRow::UiBoxWithBuildings(uib) => {
                let mut area = line.clone();
                area.size.y = area.size.y * (uib.rows * 2) as f32;
                uib.draw(window, sprites, &area)?;
                line.pos.y += area.size.y;
            }
        }
    }
    Ok(())
}

fn row_count(table: &[TableRow]) -> usize {
    table.iter()
        .fold(
            0, 
            |acc, row| {
                acc + match row {
                    TableRow::Text(_) => 1,
                    TableRow::TextWithImage(_,_) => 1,
                    TableRow::UiBoxWithEntities(uib) => 2 * uib.rows,
                    TableRow::UiBoxWithBuildings(uib) => 2 * uib.rows,
                }
            }
        )
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
        let mut img_area = max_img_area.shrink_and_fit_into(&res_area, FitStrategy::TopLeft);
        img_area.size.y = res_area.height();
        img_area.pos.x = img_area.pos.x + res_area.width() - img_area.width();
        let text_h = res_area.height().min(36.0);
        let text_area = Rectangle::new(  
            (res_area.pos.x, res_area.pos.y + (res_area.height() - text_h)/2.0),
            (res_area.size.x - img_area.width(), text_h)
        );
        draw_static_image(sprites, window, &img_area, rt.sprite().default(), z, FitStrategy::Center)?;
        write_text(font, window, &text_area, z+1, FitStrategy::Center, &n.to_string())?;
    }
    Ok(())
}


#[derive(Clone, Debug)]
struct UiElement<T: Clone + std::fmt::Debug> {
    display: RenderVariant,
    hover_info: Option<Vec<(ResourceType, i64)>>, 
    on_click: Option<T>,
}
#[derive(Clone, Debug)]
pub struct UiBox<T: Clone + std::fmt::Debug> {
    area: Rectangle,
    elements: Vec<UiElement<T>>,
    columns: usize,
    rows: usize,
    padding: f32,
    margin: f32,
}

impl<T: Clone + std::fmt::Debug> UiBox<T> {
    pub fn new(columns: usize, rows: usize, padding: f32, margin: f32) -> Self {
        UiBox {
            area: Rectangle::default(),
            elements: vec![],
            columns: columns,
            rows: rows,
            padding: padding,
            margin: margin,
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, i: SpriteSet, on_click: T) {
        self.add_el(
            UiElement { 
                display: RenderVariant::Img(i), 
                hover_info: None,
                on_click: Some(on_click),
            }    
        );
    }

    pub fn add_with_render_variant(&mut self, rv: RenderVariant, on_click: T) {
        self.add_el(
            UiElement { 
                display: rv,
                hover_info: None,
                on_click: Some(on_click),
            }    
        );
    }

    pub fn add_with_background_color_and_cost(&mut self, i: SpriteSet, col: Color, on_click: T, cost: Vec<(ResourceType, i64)>) {
        self.add_el(
            UiElement { 
                display: RenderVariant::ImgWithColBackground(i, col),
                hover_info: Some(cost),
                on_click: Some(on_click),
            }    
        );
    }

    pub fn add_empty(&mut self) {
        self.add_el(
            UiElement { 
                display: RenderVariant::Hide, 
                hover_info: None,
                on_click: None,
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
                    Some(img)
                },
                RenderVariant::ImgWithColBackground(img, col) => {
                    window.draw_ex(
                        &draw_area.padded(self.margin),
                        Col(col),
                        Transform::IDENTITY, 
                        Z_MENU_BOX_BUTTONS-1,
                    );
                    Some(img)
                },
                RenderVariant::ImgWithImgBackground(img, bkg) => {
                    draw_static_image(
                        sprites, 
                        window, 
                        &draw_area, 
                        SpriteIndex::Simple(bkg), 
                        Z_MENU_BOX_BUTTONS-1, 
                        FitStrategy::Center
                    )?;
                    Some(img)
                },
                RenderVariant::ImgWithHoverAlternative(img, hov) => {
                    if window.mouse().pos().overlaps_rectangle(&draw_area) {
                        Some(hov)
                    } else {
                        Some(img)
                    } 
                },
                RenderVariant::Hide => {
                    None
                }
            };
            if let Some(img) = img {
                draw_static_image(
                    sprites, 
                    window, 
                    &draw_area.padded(self.padding + self.margin), 
                    img.default(), 
                    Z_MENU_BOX_BUTTONS, 
                    FitStrategy::Center
                )?;
            }
        }

        Ok(())
    }

    fn element_index_under_mouse(&self, mouse: impl Into<Vector>) -> Option<usize> {
        let dx = self.area.width() / self.columns as f32;
        let dy = self.area.height() / self.rows as f32;
        let pos = mouse.into() - self.area.pos;
        if pos.y < 0.0 || pos.x < 0.0 {
            return None;
        }
        let i = (pos.y / dy) as usize * self.columns + (pos.x / dx) as usize;
        Some(i)
    }
    fn find_element_under_mouse(&self, mouse: impl Into<Vector>) -> Option<UiElement<T>> {
        self.element_index_under_mouse(mouse)
            .and_then(|i| self.elements.get(i)) 
            .map(UiElement::clone)
    }
    fn remove_element_under_mouse(&mut self, mouse: impl Into<Vector>) -> Option<UiElement<T>> {
        self.element_index_under_mouse(mouse)
            .map(|i| self.elements.remove(i)) 
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> Option<T> {
        if let Some(el) = self.find_element_under_mouse(mouse) {
            el.on_click.clone()
        }
        else {
            None
        }
    }
    pub fn click_and_remove(&mut self, mouse: impl Into<Vector>) -> Option<T> {
        if let Some(el) = self.remove_element_under_mouse(mouse) {
            el.on_click.clone()
        }
        else {
            None
        }
    }

    pub fn draw_hover_info(&mut self, window: &mut Window, sprites: &mut Asset<Sprites>, font: &mut Asset<Font>, area: &Rectangle) -> Result<()> {
        let mouse = window.mouse().pos();
        if let Some(el) = self.find_element_under_mouse(mouse) {
            if let Some(cost) = &el.hover_info {
                draw_resources(window, sprites, &cost, area, font, Z_MENU_RESOURCES)?;
            }
        }
        Ok(())
    }
}