use crate::game::dialogue::DialogueFrame;
use crate::game::leaderboard::LeaderboardFrame;
use crate::game::map::MapFrame;
use crate::game::town::TownFrame;
use crate::game::visits::{
    attacks::VisitorFrame, reports::ReportFrame, visitor_menu::VisitorMenuFrame,
};
use crate::game::Game;
use crate::gui::menu::{MapMenuFrame, MenuBackgroundFrame, TownMenuFrame};
use crate::prelude::*;
use crate::Framer;
use quicksilver::prelude::*;
use specs::WorldExt;

pub(crate) fn load_viewer(game: &mut Game<'static, 'static>, ep: EventPool) -> Framer {
    let view = game.entry_view();
    let mut viewer = Framer::new(view);
    let resolution = *game.world.read_resource();

    /* Town */

    let menu = TownFrame::new();
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Town],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    let menu = TownMenuFrame::new(game, ep.clone()).expect("Town menu loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Town],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Menu background and buttons */
    // Somehow, the town rendering gets messed up if TownFrame is added after this frame...
    let menu = MenuBackgroundFrame::new();
    viewer.add_frame(
        Box::new(menu),
        &[
            UiView::Town,
            UiView::Leaderboard,
            UiView::Map,
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
        ],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Map */

    let menu = MapFrame::new();
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Map],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    let menu = MapMenuFrame::new(game, ep).expect("Map menu loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Map],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Visitors */

    let menu = VisitorMenuFrame::new();
    viewer.add_frame(
        Box::new(menu),
        &[
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
        ],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    let (w, h) = game.world.fetch::<ScreenResolution>().main_area();
    let menu = VisitorFrame::new(0.0, 0.0, w, h).expect("Attacks loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Visitors(VisitorViewTab::IncomingAttacks)],
        (0, 0),
        (w as i32, h as i32),
    );

    let rect = Rectangle::new((0.0, 0.0), (w, h));
    let frame = ReportFrame::new(rect, resolution).expect("Report frame loading");
    viewer.add_frame(
        Box::new(frame),
        &[UiView::Visitors(VisitorViewTab::Letters)],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Leaderboard */

    let menu = LeaderboardFrame::new(&rect).expect("Leaderboard loading");
    viewer.add_frame(
        Box::new(menu),
        &[UiView::Leaderboard],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Dialogue box */

    let (w1, _h1) = game.world.fetch::<ScreenResolution>().menu_area();
    let w = w + w1;
    let rect = Rectangle::new((0.0, 0.0), (w, h));
    let dialogue = DialogueFrame::new(&rect).expect("Dialogue loading");
    viewer.add_frame(
        Box::new(dialogue),
        &[UiView::Dialogue],
        (0, 0),
        (w as i32, h as i32),
    );
    viewer.reload(game).expect("Initial View loading");
    viewer
}
