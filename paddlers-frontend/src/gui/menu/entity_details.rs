use crate::game::{components::*, map::VillageMetaInfo};
use crate::{
    game::{
        components::{EntityContainer, ForestComponent, Level, StatusEffects, UiMenu},
        fight::{Aura, Health},
        mana::Mana,
        player_info::PlayerInfo,
    },
    gui::{gui_components::*, menu::*, sprites::*, ui_state::Now, utils::*, z::*},
};
use paddle::*;
use paddle::{quicksilver_compat::*, DisplayArea, FitStrategy};
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

use crate::{
    game::{
        level::SpriteIndex,
        mana::{RenderVariant, TableTextProvider},
    },
    gui::{sprites::Sprites, z::Z_MENU_BOX},
};

pub fn draw_entity_img(
    world: &World,
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    e: Entity,
    area: &Rectangle,
) {
    let r = world.read_storage::<Renderable>();
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
                );
                draw_static_image(
                    sprites,
                    window,
                    &inner_area,
                    main.default(),
                    Z_MENU_BOX + 2,
                    FitStrategy::Center,
                );
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
                );
            }
            _ => panic!("Not implemented"),
        }
    }
}

pub fn draw_map_entity_details_table(
    world: &World,
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    e: Entity,
    area: &Rectangle,
    text_provider: &mut TableTextProvider,
    mouse_pos: Vector,
) {
    let mut table = vec![];
    {
        let villages = world.read_storage::<VillageMetaInfo>();
        if let Some(v) = villages.get(e) {
            for row in v.village_details().into_iter() {
                table.push(row);
            }
        }
    }
    let mut ui_area = world.write_storage::<UiMenu>();
    if let Some(ui) = ui_area.get_mut(e) {
        table.push(TableRow::InteractiveArea(&mut ui.ui));
    }

    draw_table(
        window,
        sprites,
        &mut table,
        area,
        text_provider,
        40.0,
        Z_MENU_TEXT,
        world.read_resource::<Now>().0,
        TableVerticalAlignment::Top,
        mouse_pos,
    );
}
pub fn draw_town_entity_details_table(
    world: &World,
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    e: Entity,
    area: &Rectangle,
    text_provider: &mut TableTextProvider,
    res_comp: &mut ResourcesComponent,
    mouse_pos: Vector,
) {
    let mut area = *area;
    let mut table = vec![];

    let health = world.read_storage::<Health>();
    if let Some(health) = health.get(e) {
        table.push(health_details(health));
    }

    let lvls = world.read_storage::<Level>();
    if let Some(level) = lvls.get(e) {
        table.extend(level.menu_table_infos());
    }

    let mana = world.read_storage::<Mana>();
    if let Some(m) = mana.get(e) {
        table.extend(m.menu_table_infos());
    }

    let aura = world.read_storage::<Aura>();
    if let Some(aura) = aura.get(e) {
        table.push(aura_details(aura));
    }
    let forest = world.read_storage::<ForestComponent>();
    if let Some(forest) = forest.get(e) {
        table.push(tree_details(forest));
    }

    let mut container = world.write_storage::<EntityContainer>();
    if let Some(c) = container.get_mut(e) {
        table.push(TableRow::Text(format!(
            "{}/{} occupied",
            c.count(),
            c.capacity
        )));
    }

    let buildings = world.write_storage::<Building>();
    let mut ui_menu = world.write_storage::<UiMenu>();
    if let Some(b) = buildings.get(e) {
        if b.bt == BuildingType::Temple && ui_menu.get(e).is_some() {
            let player_info = world.read_resource::<PlayerInfo>();
            table.extend(temple_details(&player_info));
        }
    }
    let effects = world.read_storage::<StatusEffects>();
    if let Some(ef) = effects.get(e) {
        let list = ef.menu_table_infos();
        if list.len() > 0 {
            TableRow::Text("Status effects".to_owned());
            table.extend(list);
        }
    }

    if let Some(ui) = ui_menu.get_mut(e) {
        Game::draw_shop_prices(window, &mut area, &mut ui.ui, res_comp, mouse_pos).nuts_check();
        table.push(TableRow::InteractiveArea(&mut ui.ui));
    }

    draw_table(
        window,
        sprites,
        &mut table,
        &area,
        text_provider,
        40.0,
        Z_MENU_TEXT,
        world.read_resource::<Now>().0,
        TableVerticalAlignment::Top,
        mouse_pos,
    );
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
pub fn forest_details<'a>(forest_size: usize, forest_usage: usize) -> TableRow<'a> {
    let text = format!("{} (using {})", forest_size, forest_usage);
    TableRow::TextWithImage(text, SpriteIndex::Simple(SingleSprite::Tree))
}
pub fn total_aura_details<'a>(aura_size: i64) -> TableRow<'a> {
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
