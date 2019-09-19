use specs::prelude::*;
use crate::prelude::*;

use crate::game::{
    town_resources::TownResources,
};
use crate::gui::{
    input::UiState,
    menu::buttons::MenuButtons,
};
use crate::net::{
    game_master_api::RestApiState,
};
use crate::logging::{
    ErrorQueue,
    AsyncErr,
    text_to_user::TextBoard,
};


#[derive(Default)]
/// Global animation ticker
pub struct ClockTick(pub u32);
#[derive(Default)]
/// Used for UI scaling. To be removed in favour of better options.
pub struct UnitLength(pub f32);
#[derive(Default)]
/// Timestamp of frame rendering
pub struct Now(pub Timestamp);

pub (super) fn insert_resources(world: &mut World, err_send: std::sync::mpsc::Sender<PadlError>) {
    let err_send_clone = err_send.clone();
    world.insert(ClockTick(0));
    world.insert(UiState::default());
    world.insert(Now);
    world.insert(ErrorQueue::default());
    world.insert(AsyncErr::new(err_send));
    world.insert(TownResources::default());
    world.insert(RestApiState::new(err_send_clone));
    world.insert(TextBoard::default());
    world.insert(MenuButtons::new());
}
