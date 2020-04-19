use paddlers_shared_lib::api::shop::*;
use crate::prelude::*;
use crate::gui::{
    utils::*,
    gui_components::*,
};
use quicksilver::prelude::*;
use crate::gui::{
    input::Grabbable,
    sprites::WithSprite
};
use paddlers_shared_lib::prelude::*;

#[derive(Clone)]
pub struct DefaultShop {
    pub ui: UiBox,
}
impl Default for DefaultShop {
    fn default() -> Self {
        DefaultShop {
            ui : UiBox::new(3, 3, 4.0, 8.0)
        }
    }
}
impl DefaultShop {
    pub fn new(karma: i64) -> Self {
        let mut result = DefaultShop::default();
        for b in BuildingType::default_shop_buildings()
            .filter(|b| b.player_can_build(karma))
        {
            result.add_building(*b);
        }
        result
    }

    pub fn add_building(&mut self, b: BuildingType) {
        self.ui.add(
            UiElement::new(b)
                .with_image(b.sprite())
                .with_background_color(LIGHT_BLUE)
                .with_cost(b.price())
        );
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> PadlResult<Option<(Grabbable, Option<Condition>)>> {
        self.ui.click(mouse.into())
            .map(|buy_this| {
                if let Some((ClickOutput::BuildingType(building_type), condition)) = buy_this {
                    Some((
                        Grabbable::NewBuilding(building_type),
                        condition
                    ))
                } else {
                    None
                }
            })
    }
}