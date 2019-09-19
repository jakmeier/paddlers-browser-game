//! High-level GUI components that may be related to game logic as far as necessary

mod ui_box;
pub use ui_box::*;

use crate::game::abilities::Ability;
use crate::gui::{sprites::*, utils::*, menu::buttons::MenuButtonAction};
use crate::prelude::*;
use quicksilver::prelude::*;

pub enum TableRow<'a> {
    Text(String),
    TextWithImage(String, SpriteIndex),
    InteractiveArea(&'a mut dyn InteractiveTableArea),
}

/// An area that is part of the graphical user interface.
pub trait InteractiveTableArea {
    /// Defines how many table rows it takes to draw the area
    fn rows(&self) -> usize;
    /// Draw the area on a specified area
    fn draw(&mut self, window: &mut Window, sprites: &mut Sprites, area: &Rectangle) -> Result<()>;
    /// Check if the mouse hits somthing on the area
    fn click(&self, mouse: Vector) -> Option<ClickOutput>;
    /// Remove one of the clickable options
    fn remove(&mut self, output: ClickOutput);
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Elements than can be produces by a lick in a interactive area
pub enum ClickOutput {
    Entity(specs::Entity),
    BuildingType(BuildingType),
    Ability(Ability),
    MenuButtonAction(MenuButtonAction),
}

pub fn draw_table(
    window: &mut Window,
    sprites: &mut Sprites,
    table: &mut [TableRow],
    max_area: &Rectangle,
    font: &mut Asset<Font>,
    max_row_height: f32,
    z: i32,
) -> Result<()> {
    let total_rows = row_count(table);
    let row_height = max_row_height.min(max_area.height() / total_rows as f32);
    let font_h = row_height * 0.9;
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
            TableRow::InteractiveArea(ia) => {
                let mut area = line.clone();
                area.size.y = area.size.y * ia.rows() as f32;
                ia.draw(window, sprites, &area)?;
                line.pos.y += area.size.y;
            }
        }
    }
    Ok(())
}

fn row_count(table: &[TableRow]) -> usize {
    table.iter().fold(0, |acc, row| {
        acc + match row {
            TableRow::Text(_) => 1,
            TableRow::TextWithImage(_, _) => 1,
            TableRow::InteractiveArea(ia) => ia.rows(),
        }
    })
}

pub fn draw_resources(
    window: &mut Window,
    sprites: &mut Sprites,
    resis: &[(ResourceType, i64)],
    max_area: &Rectangle,
    font: &mut Asset<Font>,
    z: i32,
) -> Result<()> {
    // XXX This is quite specific. If this is used more flexible, consider refactoring.
    let cols = 3;
    let rows = (2 + resis.len()) / cols;
    let grid = max_area.grid(cols, rows);
    let max_img_area = Rectangle::new_sized((50, 50));
    for ((rt, n), res_area) in resis.iter().zip(grid) {
        let mut img_area = max_img_area.shrink_and_fit_into(&res_area, FitStrategy::TopLeft);
        img_area.size.y = res_area.height();
        img_area.pos.x = img_area.pos.x + res_area.width() - img_area.width();
        let text_h = res_area.height().min(36.0);
        let text_area = Rectangle::new(
            (
                res_area.pos.x,
                res_area.pos.y + (res_area.height() - text_h) / 2.0,
            ),
            (res_area.size.x - img_area.width(), text_h),
        );
        draw_static_image(
            sprites,
            window,
            &img_area.padded(10.0),
            rt.sprite().default(),
            z,
            FitStrategy::Center,
        )?;
        write_text(
            font,
            window,
            &text_area,
            z + 1,
            FitStrategy::Center,
            &n.to_string(),
        )?;
    }
    Ok(())
}

impl From<specs::Entity> for ClickOutput {
    fn from(e: specs::Entity) -> Self {
        ClickOutput::Entity(e)
    }
}
impl From<BuildingType> for ClickOutput {
    fn from(bt: BuildingType) -> Self {
        ClickOutput::BuildingType(bt)
    }
}
impl From<Ability> for ClickOutput {
    fn from(a: Ability) -> Self {
        ClickOutput::Ability(a)
    }
}
impl From<MenuButtonAction> for ClickOutput {
    fn from(a: MenuButtonAction) -> Self {
        ClickOutput::MenuButtonAction(a)
    }
}