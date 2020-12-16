use crate::{game::leaderboard::LeaderboardFrame, resolution::{MENU_AREA_X, MENU_AREA_Y}};
use crate::game::map::MapFrame;
use crate::game::town::TownFrame;
use crate::game::visits::{
    attacks::VisitorFrame, reports::ReportFrame, visitor_menu::VisitorMenuFrame,
};
use crate::game::{dialogue::DialogueFrame, town::town_summary::TownSummaryFrame};
use crate::gui::menu::{MapMenuFrame, MenuBackgroundFrame, TownMenuFrame};
use crate::prelude::*;
use paddle::ViewManager;

pub(crate) fn load_viewer(view: UiView) -> ViewManager<UiView> {
    let mut viewer = ViewManager::new(view);

    /* Town */

    let menu = TownFrame::new();
    let town_handler = viewer.add_frame(menu, &[UiView::Town, UiView::TownHelp], (0, 0));
    town_handler.listen(TownFrame::signal);

    let menu = TownMenuFrame::new().expect("Town menu loading");
    let town_menu_handle = viewer.add_frame(menu, &[UiView::Town, UiView::TownHelp], (0, 0));
    town_menu_handle.listen(TownMenuFrame::new_story_state);
    town_menu_handle.listen(TownMenuFrame::signal);

    /* Menu background and buttons */
    // Somehow, the town rendering gets messed up if TownFrame is added after this frame...
    let menu = MenuBackgroundFrame::new();
    let menu_bg_handler = viewer.add_frame(
        menu,
        &[
            UiView::Town,
            UiView::Leaderboard,
            UiView::Map,
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
            UiView::TownHelp,
        ],
        (0, 0),
    );
    menu_bg_handler.listen(MenuBackgroundFrame::network_message);
    menu_bg_handler.listen(MenuBackgroundFrame::signal);

    /* Map */

    let menu = MapFrame::new();
    viewer.add_frame(menu, &[UiView::Map], (0, 0));

    let menu = MapMenuFrame::new().expect("Map menu loading");
    viewer.add_frame(menu, &[UiView::Map], (MENU_AREA_X, MENU_AREA_Y));

    /* Visitors */

    let menu = VisitorMenuFrame::new();
    viewer.add_frame(
        menu,
        &[
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
        ],
        (MENU_AREA_X, MENU_AREA_Y),
    );

    let menu = VisitorFrame::new(0.0, 0.0).expect("Attacks loading");
    viewer.add_frame(
        menu,
        &[UiView::Visitors(VisitorViewTab::IncomingAttacks)],
        (0, 0),
    );

    let frame = ReportFrame::new().expect("Report frame loading");
    let report_handler =
        viewer.add_frame(frame, &[UiView::Visitors(VisitorViewTab::Letters)], (0, 0));
    report_handler.listen(ReportFrame::network_message);

    /* Leaderboard */

    let menu = LeaderboardFrame::new().expect("Leaderboard loading");
    let leaderboard_handler = viewer.add_frame(menu, &[UiView::Leaderboard], (0, 0));
    leaderboard_handler.listen(LeaderboardFrame::network_message);

    let summary = TownSummaryFrame::new().expect("Town summary loading");
    viewer.add_frame(summary, &[UiView::TownHelp], (0, 0));

    /* Dialogue box */
    let dialogue = DialogueFrame::new().expect("Dialogue loading");
    let dialogue_handle = viewer.add_frame(dialogue, &[UiView::Dialogue], (0, 0));
    dialogue_handle.listen(DialogueFrame::receive_load_scene);
    dialogue_handle.listen(DialogueFrame::receive_new_story_state);
    viewer
}
