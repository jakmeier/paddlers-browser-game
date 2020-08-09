use crate::game::dialogue::DialogueFrame;
use crate::game::leaderboard::LeaderboardFrame;
use crate::game::map::MapFrame;
use crate::game::town::TownFrame;
use crate::game::visits::{
    attacks::VisitorFrame, reports::ReportFrame, visitor_menu::VisitorMenuFrame,
};
use crate::gui::menu::{MapMenuFrame, MenuBackgroundFrame, TownMenuFrame};
use crate::prelude::*;
use crate::view::new_frame::ViewManager;
use quicksilver::prelude::*;

pub(crate) fn load_viewer(view: UiView, resolution: ScreenResolution) -> ViewManager<UiView> {
    let mut viewer = ViewManager::new(view);

    /* Town */

    let menu = TownFrame::new();
    viewer.add_frame(
        menu,
        &[UiView::Town],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    let menu = TownMenuFrame::new().expect("Town menu loading");
    viewer.add_frame(
        menu,
        &[UiView::Town],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Menu background and buttons */
    // Somehow, the town rendering gets messed up if TownFrame is added after this frame...
    let menu = MenuBackgroundFrame::new();
    viewer.add_frame(
        menu,
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
        menu,
        &[UiView::Map],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    let menu = MapMenuFrame::new().expect("Map menu loading");
    viewer.add_frame(
        menu,
        &[UiView::Map],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Visitors */

    let menu = VisitorMenuFrame::new();
    viewer.add_frame(
        menu,
        &[
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
        ],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    let (w, h) = resolution.main_area();
    let menu = VisitorFrame::new(0.0, 0.0, w, h).expect("Attacks loading");
    viewer.add_frame(
        menu,
        &[UiView::Visitors(VisitorViewTab::IncomingAttacks)],
        (0, 0),
        (w as i32, h as i32),
    );

    let rect = Rectangle::new((0.0, 0.0), (w, h));
    let frame = ReportFrame::new(rect, resolution).expect("Report frame loading");
    viewer.add_frame(
        frame,
        &[UiView::Visitors(VisitorViewTab::Letters)],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Leaderboard */

    let menu = LeaderboardFrame::new(&rect).expect("Leaderboard loading");
    viewer.add_frame(
        menu,
        &[UiView::Leaderboard],
        (0, 0), // TODO
        (0, 0), // TODO
    );

    /* Dialogue box */

    let (w1, _h1) = resolution.menu_area();
    let w = w + w1;
    let rect = Rectangle::new((0.0, 0.0), (w, h));
    let dialogue = DialogueFrame::new(&rect).expect("Dialogue loading");
    viewer.add_frame(dialogue, &[UiView::Dialogue], (0, 0), (w as i32, h as i32));
    viewer
}
