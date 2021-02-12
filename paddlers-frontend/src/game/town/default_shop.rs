use crate::game::player_info::PlayerInfo;
use crate::gui::{gui_components::*, menu::entity_details::*, ui_state::Now, utils::*, z::*};
use crate::gui::{input::Grabbable, sprites::WithSprite};
use crate::prelude::*;
use paddle::DisplayArea;
use paddle::*;
use paddlers_shared_lib::api::shop::*;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

#[derive(Clone)]
pub struct DefaultShop {
    pub ui: UiBox,
}
impl Default for DefaultShop {
    fn default() -> Self {
        DefaultShop {
            ui: UiBox::new(3, 3, 4.0, 8.0),
        }
    }
}
impl DefaultShop {
    pub fn new(player_info: &PlayerInfo) -> Self {
        let mut result = DefaultShop::default();
        let karma = player_info.karma();
        let story_state = player_info.story_state();
        for b in BuildingType::default_shop_buildings()
            .filter(|b| b.player_can_build(karma, story_state, player_info.civilization_perks()))
        {
            result.add_building(*b);
        }
        result
    }
    pub fn reload(world: &mut specs::World) {
        let player_info = (*world.read_resource::<PlayerInfo>()).clone();
        world.insert(DefaultShop::new(&player_info));
    }

    pub fn add_building(&mut self, b: BuildingType) {
        self.ui.add(
            UiElement::new(b)
                .with_image(b.sprite())
                .with_background_color(LIGHT_BLUE)
                .with_cost(b.price()),
        );
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> Option<(Grabbable, Option<Condition>)> {
        if let Some((ClickOutput::BuildingType(building_type), condition)) =
            self.ui.click(mouse.into())
        {
            Some((Grabbable::NewBuilding(building_type), condition))
        } else {
            None
        }
    }
}

impl DefaultShop {
    pub(crate) fn render_default_shop(
        game: &mut Game,
        window: &mut DisplayArea,
        area: &Rectangle,
        text_provider: &mut TableTextProvider,
        res_comp: &mut ResourcesComponent,
        mouse_pos: Option<Vector>,
    ) {
        let mut table = vec![];
        let mut area = *area;

        // <TODO: This stuff should be moved to the town info view>
        // table.push(faith_details(game.town().faith));
        table.push(forest_details(
            game.town().forest_size(),
            game.town().forest_usage(),
        ));
        table.push(total_aura_details(game.town().ambience()));
        // </TODO>

        let shop = &mut game.town_context.world().write_resource::<DefaultShop>();
        Game::draw_shop_prices(window, &mut area, &mut shop.ui, res_comp, mouse_pos).nuts_check();

        table.push(TableRow::InteractiveArea(&mut shop.ui));

        draw_table(
            window,
            &mut game.sprites,
            &mut table,
            &area,
            text_provider,
            60.0,
            Z_UI_MENU,
            game.town_context.world().read_resource::<Now>().0,
            TableVerticalAlignment::Top,
            mouse_pos,
        )
    }
}
impl Game {
    pub fn draw_shop_prices(
        display: &mut DisplayArea,
        area: &mut Rectangle,
        ui: &mut UiBox,
        res_comp: &mut ResourcesComponent,
        mouse_pos: Option<Vector>,
    ) -> PadlResult<()> {
        let price_tag_h = 50.0;
        let (shop_area, price_tag_area) = area.cut_horizontal(area.height() - price_tag_h);
        *area = shop_area;
        if let Some(mouse_pos) = mouse_pos {
            ui.draw_hover_info(display, res_comp, &price_tag_area, mouse_pos)?;
        } else {
            res_comp.clear();
        }
        Ok(())
    }
}
