use quicksilver::prelude::*;
use specs::prelude::*;
use crate::game::{
    Game,
    fight::{Health, Aura},
    components::EntityContainer,
    forestry::ForestComponent,
};
use crate::gui::{
    sprites::{SpriteIndex, WithSprite},
    z::*,
    input::{UiState, DefaultShop, Grabbable},
    utils::*,
    gui_components::*,
    render::Renderable,
};


impl Game<'_, '_> {
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

    pub fn render_entity_details(&mut self, window: &mut Window, area: &Rectangle, e: Entity) -> Result<()> {
        let mut img_bg_area = area.clone();
        img_bg_area.size.y = img_bg_area.height() / 3.0;
        let img_bg_area = img_bg_area.fit_square(FitStrategy::Center).shrink_to_center(0.8);
        let text_y = img_bg_area.y() + img_bg_area.height();
        let text_area = Rectangle::new((area.x(), text_y), (area.width(), area.y() + area.height() - text_y) ).padded(20.0);

        self.draw_entity_details_img(window, e, &img_bg_area)?;
        self.draw_entity_details_table(window, e, &text_area)?;
        Ok(())
    }

    fn draw_entity_details_img(&mut self, window: &mut Window, e: Entity, area: &Rectangle,) -> Result<()> {
        let r = self.world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        let inner_area = area.shrink_to_center(0.8);
        if let Some(rd) = r.get(e) {
            match rd.kind {
                RenderVariant::ImgWithImgBackground(main, background) => {
                    draw_static_image(sprites, window, &area, background, Z_MENU_BOX + 1, FitStrategy::Center)?;
                    draw_static_image(sprites, window, &inner_area, main, Z_MENU_BOX + 2, FitStrategy::Center)?;
                },
                _ => { panic!("Not implemented") }
            }
        }
        Ok(())
    }

    fn draw_entity_details_table(&mut self, window: &mut Window, e: Entity, area: &Rectangle) -> Result<()> {
        let mut table = vec![];

        let health = self.world.read_storage::<Health>();
        if let Some(health) = health.get(e) {
            table.push(health_details(health));
        }

        let aura = self.world.read_storage::<Aura>();
        if let Some(aura) = aura.get(e) {
            table.push(aura_details(aura));
        }
        
        let forest = self.world.read_storage::<ForestComponent>();
        if let Some(forest) = forest.get(e) {
            table.push(tree_details(forest));
        }

        let mut container = self.world.write_storage::<EntityContainer>();
        if let Some(c) = container.get_mut(e) {
            table.push(forest_details(self.town().forest_size()));
            table.push(
                TableRow::Text(format!("{}/{} occupied", c.count(), c.capacity))
            );
            table.push(
                TableRow::UiBoxWithEntities(&mut c.ui)
            );
        }
        draw_table(window, &mut self.sprites, &mut table, &area, &mut self.font, 40.0, Z_MENU_TEXT)?;
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

fn aura_details(aura: &Aura) -> TableRow {
    let text = format!("+{} Ambience", aura.effect);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Ambience,
    )
}
fn health_details(health: &Health) -> TableRow {
    let health_text = format!("Well-being {}/{}", health.max_hp - health.hp, health.max_hp);
    TableRow::TextWithImage(
        health_text,
        SpriteIndex::Heart,
    )
}
fn tree_details(forest: &ForestComponent) -> TableRow {
    let text = format!("+{} Forest flora", forest.score);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Tree,
    )
}
fn forest_details<'a>(forest_size: usize) -> TableRow<'a> {
    let text = format!("{} Forest flora", forest_size);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Tree,
    )
}