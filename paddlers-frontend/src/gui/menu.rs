pub mod buttons;
mod map_menu;
mod town_menu;

pub(crate) use map_menu::MapMenuFrame;
pub(crate) use town_menu::TownMenuFrame;

use crate::game::{
    components::*,
    fight::{Aura, Health},
    forestry::ForestComponent,
    map::VillageMetaInfo,
    player_info::PlayerInfo,
    town::DefaultShop,
    Game,
};
use crate::gui::{
    decoration::*,
    gui_components::*,
    render::Renderable,
    sprites::{SingleSprite, SpriteIndex},
    ui_state::{Now, UiState},
    utils::*,
    z::*,
};
use crate::prelude::*;
use crate::resolution::ScreenResolution;
use quicksilver::prelude::{Col, Rectangle, Transform, Vector, Window};
use specs::prelude::*;

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
    pub fn leaves_border_h(&self) -> f32 {
        match self {
            ScreenResolution::Low => 15.0,
            ScreenResolution::Mid => 40.0,
            ScreenResolution::High => 100.0,
        }
    }
    pub fn leaves_border_w(&self) -> f32 {
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
    pub fn render_menu_box(&mut self, window: &mut Window) -> PadlResult<Rectangle> {
        let resolution = *self.world.read_resource::<ScreenResolution>();
        let button_height = resolution.button_h();

        let data = self.world.read_resource::<UiState>();
        let mut area = data.menu_box_area;
        let y0 = data.menu_box_area.y();
        let h0 = data.menu_box_area.height();
        std::mem::drop(data);

        // Menu Box Background
        window.draw_ex(&area, Col(LIGHT_GREEN), Transform::IDENTITY, Z_MENU_BOX);

        // Leaves border
        let leaf_w = resolution.leaves_border_w();
        let leaf_h = resolution.leaves_border_h();
        draw_leaf_border(window, &mut self.sprites, &area, leaf_w, leaf_h);

        let padding = resolution.menu_padding();
        area.pos.x += leaf_w * 0.25 + padding * 0.5;
        area.pos.y += leaf_h / 2.0;
        area.size.x -= leaf_w + padding;
        area.size.y -= leaf_h;

        // butttons
        let mut y = area.y();
        let button_area = Rectangle::new((area.pos.x, y), (area.width(), button_height));
        self.render_buttons(window, &button_area)?;
        y += button_height;

        // Duck steps
        let h = resolution.duck_steps_h();
        draw_duck_step_line(
            window,
            &mut self.sprites,
            Vector::new(area.x() - leaf_w * 0.5, y),
            area.x() + area.width() + leaf_w * 0.5,
            h,
        );
        y += h + 10.0;
        let h = y0 + h0 - y - leaf_h;
        area.pos.y = y;
        area.size.y = h;

        // Return area that is left for other menus
        Ok(area)
    }

    pub fn render_entity_details(
        &mut self,
        window: &mut Window,
        area: &Rectangle,
        e: Entity,
        text_provider: &mut TableTextProvider,
        hover_res_comp: &mut ResourcesComponent,
    ) -> PadlResult<()> {
        let mut img_bg_area = area.clone();
        img_bg_area.size.y = img_bg_area.height() / 3.0;
        let img_bg_area = img_bg_area
            .fit_square(FitStrategy::Center)
            .shrink_to_center(0.8);
        let text_y = img_bg_area.y() + img_bg_area.height();
        let text_area = Rectangle::new(
            (area.x(), text_y),
            (area.width(), area.y() + area.height() - text_y),
        )
        .padded(20.0);

        self.draw_entity_details_img(window, e, &img_bg_area)?;
        self.draw_entity_details_table(window, e, &text_area, text_provider, hover_res_comp)?;
        Ok(())
    }

    fn draw_entity_details_img(
        &mut self,
        window: &mut Window,
        e: Entity,
        area: &Rectangle,
    ) -> PadlResult<()> {
        let r = self.world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        let inner_area = area.shrink_to_center(0.8);
        if let Some(rd) = r.get(e) {
            match rd.kind {
                RenderVariant::ImgWithImgBackground(main, background) => {
                    draw_static_image(
                        sprites,
                        window,
                        &area,
                        SpriteIndex::Simple(background),
                        Z_MENU_BOX + 1,
                        FitStrategy::Center,
                    )?;
                    draw_static_image(
                        sprites,
                        window,
                        &inner_area,
                        main.default(),
                        Z_MENU_BOX + 2,
                        FitStrategy::Center,
                    )?;
                }
                RenderVariant::ImgWithColBackground(main, col) => {
                    window.draw_ex(area, Col(col), Transform::IDENTITY, Z_MENU_BOX + 1);
                    draw_static_image(
                        sprites,
                        window,
                        &inner_area,
                        main.default(),
                        Z_MENU_BOX + 2,
                        FitStrategy::Center,
                    )?;
                }
                _ => panic!("Not implemented"),
            }
        }
        Ok(())
    }

    fn draw_entity_details_table(
        &mut self,
        window: &mut Window,
        e: Entity,
        area: &Rectangle,
        text_provider: &mut TableTextProvider,
        res_comp: &mut ResourcesComponent,
    ) -> PadlResult<()> {
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
            table.push(TableRow::Text(format!(
                "{}/{} occupied",
                c.count(),
                c.capacity
            )));
        }

        let temple = self.town().temple;
        if let Some(temple) = temple {
            if e == temple {
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
            Self::draw_shop_prices(window, &mut area, &mut ui.ui, res_comp)?;
            table.push(TableRow::InteractiveArea(&mut ui.ui));
        }
        draw_table(
            window,
            &mut self.sprites,
            &mut table,
            &area,
            text_provider,
            40.0,
            Z_MENU_TEXT,
            self.world.read_resource::<Now>().0,
        )?;
        Ok(())
    }

    fn render_default_shop(
        &mut self,
        window: &mut Window,
        area: &Rectangle,
        text_provider: &mut TableTextProvider,
        res_comp: &mut ResourcesComponent,
    ) -> PadlResult<()> {
        let mut table = vec![];
        let mut area = *area;
        // table.push(faith_details(self.town().faith));
        table.push(forest_details(
            self.town().forest_size(),
            self.town().forest_usage(),
        ));
        table.push(total_aura_details(self.town().ambience()));
        let shop = &mut self.world.write_resource::<DefaultShop>();
        Self::draw_shop_prices(window, &mut area, &mut shop.ui, res_comp)?;

        table.push(TableRow::InteractiveArea(&mut shop.ui));

        draw_table(
            window,
            &mut self.sprites,
            &mut table,
            &area,
            text_provider,
            60.0,
            Z_MENU_TEXT,
            self.world.read_resource::<Now>().0,
        )
    }
    fn draw_shop_prices(
        window: &mut Window,
        area: &mut Rectangle,
        ui: &mut UiBox,
        res_comp: &mut ResourcesComponent,
    ) -> PadlResult<()> {
        let price_tag_h = 50.0;
        let (shop_area, price_tag_area) = area.cut_horizontal(area.height() - price_tag_h);
        *area = shop_area;
        ui.draw_hover_info(window, res_comp, &price_tag_area)?;
        Ok(())
    }
}

fn aura_details(aura: &Aura) -> TableRow {
    let text = format!("+{}", aura.effect);
    TableRow::TextWithImage(text, SpriteIndex::Simple(SingleSprite::Ambience))
}
fn health_details(health: &Health) -> TableRow {
    let health_text = format!("Well-being {}/{}", health.max_hp - health.hp, health.max_hp);
    TableRow::TextWithImage(health_text, SpriteIndex::Simple(SingleSprite::Heart))
}
fn tree_details(forest: &ForestComponent) -> TableRow {
    let text = format!("+{}", forest.score);
    TableRow::TextWithImage(text, SpriteIndex::Simple(SingleSprite::Tree))
}
fn forest_details<'a>(forest_size: usize, forest_usage: usize) -> TableRow<'a> {
    let text = format!("{} (using {})", forest_size, forest_usage);
    TableRow::TextWithImage(text, SpriteIndex::Simple(SingleSprite::Tree))
}
fn total_aura_details<'a>(aura_size: i64) -> TableRow<'a> {
    let text = format!("Ambience: {}", aura_size);
    TableRow::TextWithImage(text, SpriteIndex::Simple(SingleSprite::Ambience))
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
// fn faith_details<'a>(faith: u8) -> TableRow<'a> {
//     let text = format!("{}% faith", faith);
//     TableRow::Text(text)
// }
