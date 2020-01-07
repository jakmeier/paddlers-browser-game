use crate::game::Game;
use crate::view::FrameManager;
use crate::game::map::MapFrame;
use crate::gui::input::UiView;
use crate::gui::menu::{MapMenuFrame,TownMenuFrame};
use quicksilver::prelude::*;
use crate::Framer;
use crate::prelude::*;

pub (crate) fn load_viewer(game: &mut Game<'static,'static>, ep: EventPool) -> Framer {
    let mut viewer: FrameManager<UiView,Game<'static,'static>,Window,Event,PadlError> = Default::default();

    /* Town */

    let menu = TownMenuFrame::new(game, ep.clone()).expect("Town menu loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Town],
        (0,0), // TODO
        (0,0), // TODO
    );

    /* Map */

    let menu = MapFrame::new();
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Map],
        (0,0), // TODO
        (0,0), // TODO
    );

    let menu = MapMenuFrame::new(game, ep).expect("Map menu loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Map],
        (0,0), // TODO
        (0,0), // TODO
    );

    viewer
}