use paddlers_shared_lib::api::shop::*;
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
    pub ui: UiBox<BuildingType>,
}
impl Default for DefaultShop {
    fn default() -> Self {
        DefaultShop {
            ui : UiBox::new(3, 3, 4.0, 8.0)
        }
    }
}
impl DefaultShop {
    pub fn new() -> Self {
        let mut result = DefaultShop::default();
        result.add_building(BuildingType::BlueFlowers);
        result.add_building(BuildingType::RedFlowers);
        result.add_building(BuildingType::Tree);
        result.add_building(BuildingType::BundlingStation);
        result.add_building(BuildingType::SawMill);
        result
    }

    fn add_building(&mut self, b: BuildingType) {
        self.ui.add_with_background_color_and_cost(b.sprite(), LIGHT_BLUE, b, b.cost());
    }

    pub fn click(&self, mouse: impl Into<Vector>) -> Option<Grabbable> {
        let buy_this = self.ui.click(mouse);
        if let Some(building_type) = buy_this {
            return Some(
                Grabbable::NewBuilding(building_type)
            )
        }
        None
    }
}