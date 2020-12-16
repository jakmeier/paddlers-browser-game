use crate::game::player_info::PlayerInfo;
use crate::gui::{gui_components::*, utils::*};
use crate::gui::{input::Grabbable, sprites::WithSprite};
use paddle::quicksilver_compat::*;
use paddlers_shared_lib::api::shop::*;
use paddlers_shared_lib::prelude::*;

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
            .filter(|b| b.player_can_build(karma, story_state))
        {
            result.add_building(*b);
        }
        result
    }
    pub fn reload(world: &mut specs::World) {
        use specs::WorldExt;
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
