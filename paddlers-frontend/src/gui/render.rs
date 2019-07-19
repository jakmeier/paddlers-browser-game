use quicksilver::prelude::*;
use quicksilver::graphics::Color;
use specs::prelude::*;
use crate::game::{
    Game,
    movement::Position,
    fight::{Health, Range, Aura},
    town::Town,
};
use crate::gui::{
    sprites::{SpriteIndex, WithSprite, Sprites},
    z::*,
    input::{UiState, DefaultShop, Grabbable},
    utils::*,
    gui_components::*,
};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderVariant,
}

impl Game<'_, '_> {
    pub fn render_entities(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        let pos_store = world.read_storage::<Position>();
        let rend_store = world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        for (pos, r) in (&pos_store, &rend_store).join() {
            match r.kind {
                RenderVariant::ImgWithImgBackground(i, _) => {
                    draw_static_image(sprites, window, &pos.area, i, pos.z, FitStrategy::TopLeft)?;
                },
                _ => { panic!("Not implemented")}
            }
        }
        Ok(())
    }
    pub fn render_menu_box(&mut self, window: &mut Window) -> Result<()> {
        let resources_height = 50.0;
        let data = self.world.read_resource::<UiState>();
        let entity = (*data).selected_entity;
        let area = data.menu_box_area;
        std::mem::drop(data);

        // Menu Box Background
        window.draw_ex(
            &area,
            Col(LIME_GREEN),
            Transform::IDENTITY, 
            Z_MENU_BOX
        );

        let area = area.padded(15.0);
        let resources_area = Rectangle::new( (area.pos.x, area.y()) , (area.width(), resources_height) );
        self.render_resources(window, &resources_area)?;

        let mut menu_area = area.translate((0,resources_height));
        menu_area.size.y -= resources_height;
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

    pub fn render_hovering(&mut self, window: &mut Window, entity: Entity) -> Result<()> {
        let position_store = self.world.read_storage::<Position>();
        let range_store = self.world.read_storage::<Range>();
        let health_store = self.world.read_storage::<Health>();

        if let Some((range,p)) = (&range_store, &position_store).join().get(entity, &self.world.entities()) {
            range.draw(window, &self.town(), &p.area)?;
        }

        if let Some((health,p)) = (&health_store, &position_store).join().get(entity, &self.world.entities()) {
            render_health(&health, &mut self.sprites, window, &p.area)?;
        }
        Ok(())
    }

    pub fn render_entity_details(&mut self, window: &mut Window, area: &Rectangle, e: Entity) -> Result<()> {
        let mut img_bg_area = area.clone();
        img_bg_area.size.y = img_bg_area.height() / 3.0;
        let img_bg_area = img_bg_area.fit_square(FitStrategy::Center).shrink_to_center(0.8);
        let img_area = img_bg_area.shrink_to_center(0.8);
        let text_y = img_bg_area.y() + img_bg_area.height();
        let text_area = Rectangle::new((area.x(), text_y), (area.width(), area.y() + area.height() - text_y) ).padded(20.0);

        let r = self.world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        if let Some(rd) = r.get(e) {
            match rd.kind {
                RenderVariant::ImgWithImgBackground(main, background) => {
                    draw_static_image(sprites, window, &img_bg_area, background, Z_MENU_BOX + 1, FitStrategy::Center)?;
                    draw_static_image(sprites, window, &img_area, main, Z_MENU_BOX + 2, FitStrategy::Center)?;
                },
                _ => { panic!("Not implemented") }
            }
        }

        let mut detail_table = vec![];

        let health = self.world.read_storage::<Health>();
        if let Some(health) = health.get(e) {
            let health_text = format!("Well-being {}/{}", health.max_hp - health.hp, health.max_hp);
            detail_table.push(
                TableRow::TextWithImage(
                    health_text,
                    SpriteIndex::Heart,
                )
            );
        }

        let aura = self.world.read_storage::<Aura>();
        if let Some(aura) = aura.get(e) {
            let text = format!("+{} Ambience", aura.effect);
            detail_table.push(
                TableRow::TextWithImage(
                    text,
                    SpriteIndex::Ambience,
                )
            );
        }

        draw_table(window, &mut self.sprites, &detail_table, &text_area, &mut self.font, 40.0, Z_MENU_TEXT)?;
        Ok(())
    }

    pub fn render_shop(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let mut shop = self.world.write_resource::<DefaultShop>();
        let sprites = &mut self.sprites;
        let price_tag_h = 50.0;
        let (shop_area, price_tag_area) = area.cut_horizontal(area.height() - price_tag_h);
        (*shop).ui.draw(window, sprites, &shop_area)?;
        (*shop).ui.draw_hover(window, sprites, &mut self.bold_font, &price_tag_area)
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
        let font = &mut self.bold_font;
        let resis = self.resources.non_zero_resources();
        draw_resources(window, sprites, &resis, area, font, Z_MENU_RESOURCES)?;
        Ok(())
    }
}

fn render_health(health: &Health, sprites: &mut Asset<Sprites>, window: &mut Window, area: &Rectangle) -> Result<()> {
    let (max, hp) = (health.max_hp, health.hp);
    let unit_pos = area.pos;
    let w = area.width();
    let h = 10.0;
    let max_area = Rectangle::new((unit_pos.x,unit_pos.y-h),(w,h));

    match hp {
        0 => {
            let h = 20.0;
            let max_area = Rectangle::new((unit_pos.x,unit_pos.y-h),(w,h));
            draw_static_image(sprites, window, &max_area, SpriteIndex::Heart, Z_HP_BAR, FitStrategy::Center)?;
        }
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
            lost_hp_area.size.x *= (max-hp) as f32 / max as f32;
            draw_rect(window, &max_area, GREY);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        },
        _ => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max-hp) as f32 / max as f32;
            draw_rect(window, &max_area, BLACK);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        }
    }

    Ok(())
}

impl Range {
    fn draw(&self, window: &mut Window, town: &Town, area: &Rectangle) -> Result<()> {
        // TODO Check if this aligns 100% with server. Also consider changing interface to TileIndex instead of center
        town.shadow_rectified_circle(window, area.center(), self.range);
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