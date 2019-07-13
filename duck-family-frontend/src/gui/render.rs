use quicksilver::prelude::*;
use quicksilver::graphics::Color;
use specs::prelude::*;
use specs::world::Index;
use crate::game::{
    Game,
    movement::Position,
    fight::{Health, Range},
    town::Town,
};
use crate::gui::{
    sprites::{SpriteIndex, WithSprite},
    z::*,
    input::{UiState, DefaultShop, Grabbable},
    utils::*
};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderType,
}
#[derive(Debug)]
pub enum RenderType {
    StaticImage(SpriteIndex, SpriteIndex), // main, background
}  

pub const BLACK: Color =    Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
pub const GREEN: Color =    Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 };
pub const GREY: Color =    Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };

impl Game<'_, '_> {
    pub fn render_entities(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        let pos_store = world.read_storage::<Position>();
        let rend_store = world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        for (pos, r) in (&pos_store, &rend_store).join() {
            match r.kind {
                RenderType::StaticImage(i, _) => {
                    draw_static_image(sprites, window, &pos.area, i, pos.z, FitStrategy::TopLeft)?;
                },
            }
        }
        Ok(())
    }
    pub fn render_menu_box(&mut self, window: &mut Window) -> Result<()> {
        let resources_height = 100.0;
        let data = self.world.read_resource::<UiState>();
        let entity = (*data).selected_entity;
        let area = data.menu_box_area;
        std::mem::drop(data);

        // Menu Box Background
        window.draw_ex(
            &area,
            Col(GREY),
            Transform::IDENTITY, 
            Z_MENU_BOX
        );

        let resources_area = Rectangle::new( (area.pos.x, area.y()) , (area.width(), resources_height) );
        self.render_resources(window, &resources_area)?;

        let menu_area = area.translate((0,resources_height));
        match entity {
            Some(id) => {
                self.render_entity_details(window, &menu_area, id)?;
            },
            None => {
                self.render_shop(window, &menu_area)?;
            },
        }
        Ok(())
    }

    pub fn render_hovering(&mut self, window: &mut Window, id: specs::world::Index) -> Result<()> {
        let entity = self.world.entities().entity(id);

        let position_store = self.world.read_storage::<Position>();
        let range_store = self.world.read_storage::<Range>();
        let health_store = self.world.read_storage::<Health>();

        if let Some((range,p)) = (&range_store, &position_store).join().get(entity, &self.world.entities()) {
            let ul = self.unit_len.unwrap();
            range.draw(window, &self.town, &p.area, ul)?;
        }

        if let Some((health,p)) = (&health_store, &position_store).join().get(entity, &self.world.entities()) {
            health.draw(window, &p.area)?;
        }
        Ok(())
    }

    pub fn render_entity_details(&mut self, window: &mut Window, area: &Rectangle, id: Index) -> Result<()> {
        let mut img_bg_area = area.clone();
        img_bg_area.size.y = img_bg_area.height() / 3.0;
        let img_bg_area = img_bg_area.fit_square(FitStrategy::Center).shrink_to_center(0.8);
        let img_area = img_bg_area.shrink_to_center(0.8);

        let e = self.world.entities().entity(id);
        let r = self.world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        let rd = r.get(e).expect("Selected item should have Renderable component");
        match rd.kind {
            RenderType::StaticImage(main, background) => {
                draw_static_image(sprites, window, &img_bg_area, background, Z_MENU_BOX + 1, FitStrategy::Center)?;
                draw_static_image(sprites, window, &img_area, main, Z_MENU_BOX + 2, FitStrategy::Center)?;
            },
        }
        Ok(())
    }

    pub fn render_shop(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let mut shop = self.world.write_resource::<DefaultShop>();
        let sprites = &mut self.sprites;
        (*shop).ui.draw(window, sprites, area)
    }

    pub fn render_grabbed_item(&mut self, window: &mut Window, item: &Grabbable) -> Result<()> {
        let mouse = window.mouse().pos();
        let ul = self.unit_len.unwrap();
        let center = mouse - (ul / 2.0, ul / 2.0).into();
        let max_area = Rectangle::new(center, (ul, ul));
        match item {
            Grabbable::NewBuilding(building_type) => {
                draw_static_image(&mut self.sprites, window, &max_area, building_type.sprite(), Z_GRABBED_ITEM, FitStrategy::TopLeft)?
            }
        }
        Ok(())
    }

    pub fn render_resources(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let sprites = &mut self.sprites;
        let resis = self.resources.non_zero_resources();
        let cols = 3;
        let rows = (2 + resis.len()) / cols;
        let grid = area.grid(cols, rows);
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
            draw_static_image(sprites, window, &img_area, rt.sprite(), Z_MENU_RESOURCES, FitStrategy::Center)?;
            write_text(&mut self.bold_font, window, &text_area, Z_MENU_TEXT, FitStrategy::Center, &n.to_string())?;
        }
        Ok(())
    }

}


trait Draw {
    fn draw(&self, window: &mut Window, area: &Rectangle) -> Result<()>;
}
impl Draw for Health {
    fn draw(&self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let (max, hp) = (self.max_hp, self.hp);
        let unit_pos = area.pos;
        let w = area.width();
        let h = 10.0;
        let max_area = Rectangle::new((unit_pos.x,unit_pos.y-h),(w,h));

        match hp {
            hp if hp < 10 => {
                let d = w / hp as f32;
                let mut hp_block = max_area.clone();
                hp_block.size.x = d * 0.9;
                for _ in 0..hp as usize {
                    draw_rect(window, &hp_block, GREY);
                    hp_block.pos.x += d;
                }
            },
            hp if hp < 50 => {
                let mut lost_hp_area = max_area.clone();
                let hp = max / 2;
                lost_hp_area.size.x *= (max-hp) as f32 / max as f32;
                draw_rect(window, &max_area, GREY);
                draw_rect_z(window, &lost_hp_area, GREEN, 1);
            },
            _ => {
                let mut lost_hp_area = max_area.clone();
                let hp = max / 2;
                lost_hp_area.size.x *= (max-hp) as f32 / max as f32;
                draw_rect(window, &max_area, BLACK);
                draw_rect_z(window, &lost_hp_area, GREEN, 1);
            }
        }

        Ok(())
    }
}
impl Range {
    fn draw(&self, window: &mut Window, town: &Town, area: &Rectangle, ul: f32) -> Result<()> {
        town.shadow_rectified_circle(window, ul, area.center(), self.range);
        Ok(())
    }
}
#[inline]
fn draw_rect(window: &mut Window, area: &Rectangle, col: Color) {
    draw_rect_z(window, area, col, 0);
}
#[inline]
fn draw_rect_z(window: &mut Window, area: &Rectangle, col: Color, z_shift: i32) {
    window.draw_ex(
        area,
        Col(col),
        Transform::IDENTITY, 
        Z_HP_BAR + z_shift,
    );
}