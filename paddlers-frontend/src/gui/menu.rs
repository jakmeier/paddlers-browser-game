pub mod buttons;

use crate::gui::ui_state::Now;
use crate::game::components::UiMenu;
use quicksilver::prelude::*;
use specs::prelude::*;
use crate::resolution::ScreenResolution;
use crate::game::{
    Game,
    fight::{Health, Aura},
    components::*,
    forestry::ForestComponent,
    town::DefaultShop,
    map::VillageMetaInfo,
    player_info::PlayerInfo,
};
use crate::gui::{
    sprites::{SpriteIndex, SingleSprite, Sprites},
    z::*,
    input::UiView,
    ui_state::UiState,
    utils::*,
    gui_components::*,
    render::Renderable,
    decoration::*,
};

impl ScreenResolution {
    fn button_h(&self) -> f32 {
        match self {
            ScreenResolution::Low => 50.0,
            ScreenResolution::Mid => 80.0,
            ScreenResolution::High => 150.0,
        }
    }
    fn resources_h(&self) -> f32 {
        match self {
            ScreenResolution::Low => 20.0,
            ScreenResolution::Mid => 30.0,
            ScreenResolution::High => 80.0,
        }
    }
    fn leaves_border_h(&self) -> f32 {
        match self {
            ScreenResolution::Low => 15.0,
            ScreenResolution::Mid => 40.0,
            ScreenResolution::High => 100.0,
        }
    }
    fn leaves_border_w(&self) -> f32 {
        match self {
            ScreenResolution::Low => 12.0,
            ScreenResolution::Mid => 30.0,
            ScreenResolution::High => 80.0,
        }
    }
    fn duck_steps_h(&self) -> f32 {
        match self {
            ScreenResolution::Low => 10.0,
            ScreenResolution::Mid => 30.0,
            ScreenResolution::High => 40.0,
        }
    }
    fn menu_padding(&self) -> f32 {
        match self {
            ScreenResolution::Low => 2.0,
            ScreenResolution::Mid => 5.0,
            ScreenResolution::High => 10.0,
        }
    }
}

impl Game<'_, '_> {
    pub fn render_menu_box(&mut self, window: &mut Window) -> Result<()> {
        let resolution = *self.world.read_resource::<ScreenResolution>();
        let button_height = resolution.button_h();
        let resources_height = resolution.resources_h();

        let data = self.world.read_resource::<UiState>();
        let entity = (*data).selected_entity;
        let mut area = data.menu_box_area;
        let y0 = data.menu_box_area.y();
        let h0 = data.menu_box_area.height();
        std::mem::drop(data);

        // Menu Box Background
        window.draw_ex(
            &area,
            Col(LIGHT_GREEN),
            Transform::IDENTITY, 
            Z_MENU_BOX
        );

        let leaf_w = resolution.leaves_border_w();
        let leaf_h = resolution.leaves_border_h();
        draw_leaf_border(window, self.sprites.as_mut().unwrap(), &area, leaf_w, leaf_h);

        let padding = resolution.menu_padding();
        area.pos.x += leaf_w * 0.25 + padding * 0.5;
        area.pos.y += leaf_h / 2.0;
        area.size.x -= leaf_w + padding;
        area.size.y -= leaf_h;

        let mut y = area.y();
        let button_area = Rectangle::new( (area.pos.x, y), (area.width(), button_height) );
        self.render_buttons(window, &button_area)?;
        y += button_height;

        let h = resolution.duck_steps_h();
        draw_duck_step_line(window, self.sprites.as_mut().unwrap(), 
            Vector::new(area.x()-leaf_w*0.5, y),
            area.x() + area.width() + leaf_w*0.5, 
            h
        );
        y += h + 10.0;

        let view = self.world.read_resource::<UiState>().current_view;
        match view {
            UiView::Map => {
                if let Some(e) = entity {
                    let h = y0 + h0 - y - leaf_h;
                    let menu_area = Rectangle::new((area.x(), y),(area.width(), h));
                    self.render_entity_details(window, &menu_area, e)?;
                }
            },
            UiView::Town => {
                let resources_area = Rectangle::new( (area.x(), y), (area.width(), resources_height) );
                self.render_resources(window, &resources_area)?;
                y += resources_area.height();

                let h = y0 + h0 - y - leaf_h;
                let menu_area = Rectangle::new((area.x(), y),(area.width(), h));

                self.render_town_menu(window, entity, &menu_area)?;
            },
            UiView::Attacks => {
                // NOP
            },
            UiView::Leaderboard => {
                // NOP
            },
        }

        Ok(())
    }

    fn render_town_menu(&mut self, window: &mut Window, entity: Option<Entity>, area: &Rectangle) -> Result<()> {
        match entity {
            Some(id) => {
                self.render_entity_details(window, area, id)?;
            },
            None => {
                self.render_default_shop(window, area)?;
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
                    draw_static_image(sprites.as_mut().unwrap(), window, &area, SpriteIndex::Simple(background), Z_MENU_BOX + 1, FitStrategy::Center)?;
                    draw_static_image(sprites.as_mut().unwrap(), window, &inner_area, main.default(), Z_MENU_BOX + 2, FitStrategy::Center)?;
                },
                RenderVariant::ImgWithColBackground(main, col) => {
                    window.draw_ex(area, Col(col), Transform::IDENTITY, Z_MENU_BOX + 1);
                    draw_static_image(sprites.as_mut().unwrap(), window, &inner_area, main.default(), Z_MENU_BOX + 2, FitStrategy::Center)?;
                }
                _ => { panic!("Not implemented") }
            }
        }
        Ok(())
    }

    fn draw_entity_details_table(&mut self, window: &mut Window, e: Entity, area: &Rectangle) -> Result<()> {
        let mut area = *area;
        let mut table = vec![];

        let villages = self.world.read_storage::<VillageMetaInfo>();
        if let Some(v) = villages.get(e) {
            for row in v.village_details().into_iter() {
                table.push(row);
            }
        }


        let health = self.world.read_storage::<Health>();
        if let Some(health) = health.get(e) {
            table.push(health_details(health));
        }

        let lvls = self.world.read_storage::<Level>();
        if let Some(level) = lvls.get(e) {
            table.extend(level.menu_table_infos());
        }

        let mana = self.world.read_storage::<Mana>();
        if let Some(m) = mana.get(e) {
            table.extend(m.menu_table_infos());
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
            table.push(
                TableRow::Text(format!("{}/{} occupied", c.count(), c.capacity))
            );
        }

        let temple = self.town().temple;
        if let Some(temple) = temple {
            if e  == temple {
                let player_info = self.player();
                table.extend(temple_details(&player_info));
            }
        }
        
        let effects = self.world.read_storage::<StatusEffects>();
        if let Some(ef) = effects.get(e) {
            let list = ef.menu_table_infos();
            if list.len() > 0 {
                TableRow::Text("Status effects".to_owned());
                table.extend(list);
            }
        }
        
        let mut ui_area = self.world.write_storage::<UiMenu>();
        if let Some(ui) = ui_area.get_mut(e) {
            Self::draw_shop_prices(window, &mut area, &mut ui.ui, self.sprites.as_mut().unwrap(), &mut self.bold_font)?;
            table.push(
                TableRow::InteractiveArea(&mut ui.ui)
            );
        }
        
        draw_table(window, self.sprites.as_mut().unwrap(), &mut table, 
        &area, &mut self.font, 40.0, Z_MENU_TEXT, self.world.read_resource::<Now>().0)?;
        Ok(())
    }
    
    fn render_default_shop(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let mut table = vec![];
        let mut area = *area;
        table.push(faith_details(self.town().faith));
        table.push(forest_details(self.town().forest_size(), self.town().forest_usage()));
        table.push(total_aura_details(self.town().ambience()));
        
        let shop = &mut self.world.write_resource::<DefaultShop>();
        Self::draw_shop_prices(window, &mut area, &mut shop.ui, self.sprites.as_mut().unwrap(), &mut self.bold_font)?;

        table.push(
            TableRow::InteractiveArea(&mut shop.ui)
        );

        draw_table(window, self.sprites.as_mut().unwrap(), &mut table, 
            &area, &mut self.font, 60.0, Z_MENU_TEXT, self.world.read_resource::<Now>().0)
    }
    
    fn draw_shop_prices(window: &mut Window, area: &mut Rectangle, ui: &mut UiBox,
        sprites: &mut Sprites, font: &mut Asset<Font>,
    ) -> Result<()> {
        let price_tag_h = 50.0;
        let (shop_area, price_tag_area) = area.cut_horizontal(area.height() - price_tag_h);
        *area = shop_area;
        ui.draw_hover_info(window, sprites, font, &price_tag_area)
    }

    pub fn render_resources(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let sprites = &mut self.sprites;
        let font = &mut self.bold_font;
        let resis = self.resources.non_zero_resources();
        draw_resources(window, sprites.as_mut().unwrap(), &resis, &area, font, Z_MENU_RESOURCES)?;
        Ok(())
    }
}

fn aura_details(aura: &Aura) -> TableRow {
    let text = format!("+{} Ambience", aura.effect);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Simple(SingleSprite::Ambience),
    )
}
fn health_details(health: &Health) -> TableRow {
    let health_text = format!("Well-being {}/{}", health.max_hp - health.hp, health.max_hp);
    TableRow::TextWithImage(
        health_text,
        SpriteIndex::Simple(SingleSprite::Heart),
    )
}
fn tree_details(forest: &ForestComponent) -> TableRow {
    let text = format!("+{} Forest flora", forest.score);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Simple(SingleSprite::Tree),
    )
}
fn forest_details<'a>(forest_size: usize, forest_usage: usize) -> TableRow<'a> {
    let text = format!("Forest flora: {} (used {})", forest_size, forest_usage);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Simple(SingleSprite::Tree),
    )
}
fn total_aura_details<'a>(aura_size: i64,) -> TableRow<'a> {
    let text = format!("Ambience: {}", aura_size);
    TableRow::TextWithImage(
        text,
        SpriteIndex::Simple(SingleSprite::Ambience),
    )
}
fn temple_details<'a>(player: &PlayerInfo) -> Vec<TableRow<'a>> {
    let karma = player.karma();
    let row1 = TableRow::TextWithImage(
        format!("{} Karma", karma),
        SpriteIndex::Simple(SingleSprite::Karma),
    );
    let prophets = player.prophets_available();
    let max_prophets = player.prophets_limit();
    
    let row2 = TableRow::TextWithImage(
        format!("{} / {}", prophets, max_prophets),
        SpriteIndex::Simple(SingleSprite::Prophet),
    );
    vec![row1, row2]
}
fn faith_details<'a>(faith: u8) -> TableRow<'a> {
    let text = format!("{}% faith of Paddlers", faith);
    TableRow::Text(text)
}