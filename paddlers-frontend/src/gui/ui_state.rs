use specs::prelude::*;
use quicksilver::prelude::*;
use crate::prelude::*;
use crate::gui::input::*;

#[derive(Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub main_area: Rectangle,
    pub menu_box_area: Rectangle,
    pub current_view: UiView,
}


#[derive(Default)]
/// Global animation ticker
pub struct ClockTick(pub u32);
#[derive(Default)]
/// Used for UI scaling. To be removed in favour of better options.
pub struct UnitLength(pub f32);
#[derive(Default)]
/// Real-time timestamp of frame rendering
pub struct Now(pub Timestamp);