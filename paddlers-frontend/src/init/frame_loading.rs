use crate::game::Game;
use crate::game::map::MapFrame;
use crate::gui::input::UiView;
use crate::gui::menu::{MapMenuFrame,TownMenuFrame};
use quicksilver::prelude::*;
use crate::Framer;
use crate::prelude::*;
use crate::game::attacks::AttackFrame;
use crate::game::town::TownFrame;
use crate::game::leaderboard::LeaderboardFrame;

pub (crate) fn load_viewer(game: &mut Game<'static,'static>, ep: EventPool) -> Framer {
    let mut viewer: Framer = Default::default();

    /* Town */

    let menu = TownFrame::new();
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Town],
        (0,0), // TODO
        (0,0), // TODO
    );

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

    /* Attacks */
    
    let (w,h) = game.world.fetch::<ScreenResolution>().main_area();
    let menu = AttackFrame::new(0.0,0.0,w,h).expect("Attacks loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Attacks],
        (0,0),
        (w as i32, h as i32),
    );
    
    /* Leaderboard */

    let rect = Rectangle::new((0.0,0.0),(w,h));
    let menu = LeaderboardFrame::new(&rect).expect("Leaderboard loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Leaderboard],
        (0,0), // TODO
        (0,0), // TODO
    );

    viewer
}