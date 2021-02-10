use crate::game::{
    dialogue::DialogueFrame, religion_frame::ReligionFrame, town::town_summary::TownSummaryFrame,
};
use crate::game::{
    quests::QuestsFrame,
    visits::{attacks::VisitorFrame, reports::ReportFrame, visitor_menu::VisitorMenuFrame},
};
use crate::gui::menu::{MapMenuFrame, MenuBackgroundFrame, TownMenuFrame};
use crate::prelude::*;
use crate::{
    game::leaderboard::LeaderboardFrame,
    gui::menu::{INNER_MENU_AREA_X, INNER_MENU_AREA_Y},
};
use crate::{
    game::map::MapFrame,
    resolution::{OUTER_MENU_AREA_X, OUTER_MENU_AREA_Y},
};
use crate::{game::town::TownFrame, gui::z::*};
use paddle::ViewManager;

pub(crate) fn load_viewer(view: UiView) -> ViewManager<UiView> {
    let mut viewer = ViewManager::new(view);

    /* Town */

    let menu = TownFrame::new();
    let town_handler = viewer.add_frame(menu, &[UiView::Town, UiView::TownHelp], (0, 0));
    town_handler.listen(TownFrame::signal);

    let menu = TownMenuFrame::new().expect("Town menu loading");
    let town_menu_handle = viewer.add_frame(
        menu,
        &[UiView::Town, UiView::TownHelp],
        (INNER_MENU_AREA_X as u32, INNER_MENU_AREA_Y as u32),
    );
    town_menu_handle.listen(TownMenuFrame::new_story_state);
    town_menu_handle.listen(TownMenuFrame::signal);
    town_menu_handle.set_z(MENU_Z_LAYER);

    /* Menu background and buttons */
    let menu = MenuBackgroundFrame::new();
    let menu_bg_handler = viewer.add_frame(
        menu,
        &[
            UiView::Town,
            UiView::Leaderboard,
            UiView::Map,
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
            UiView::Visitors(VisitorViewTab::Quests),
            UiView::TownHelp,
        ],
        (OUTER_MENU_AREA_X, OUTER_MENU_AREA_Y),
    );
    menu_bg_handler.set_z(MENU_BG_Z_LAYER);
    menu_bg_handler.listen(MenuBackgroundFrame::network_message);
    menu_bg_handler.listen(MenuBackgroundFrame::signal);

    /* Map */

    let menu = MapFrame::new();
    viewer.add_frame(menu, &[UiView::Map], (0, 0));

    let menu = MapMenuFrame::new().expect("Map menu loading");
    let menu_handler = viewer.add_frame(
        menu,
        &[UiView::Map],
        (INNER_MENU_AREA_X as u32, INNER_MENU_AREA_Y as u32),
    );
    menu_handler.set_z(MENU_Z_LAYER);

    /* Visitors */

    let menu = VisitorMenuFrame::new();
    let menu_handler = viewer.add_frame(
        menu,
        &[
            UiView::Visitors(VisitorViewTab::IncomingAttacks),
            UiView::Visitors(VisitorViewTab::Letters),
            UiView::Visitors(VisitorViewTab::Quests),
        ],
        (INNER_MENU_AREA_X as u32, INNER_MENU_AREA_Y as u32),
    );
    menu_handler.set_z(MENU_Z_LAYER);

    let menu = VisitorFrame::new(0.0, 0.0).expect("Attacks loading");
    viewer.add_frame(
        menu,
        &[UiView::Visitors(VisitorViewTab::IncomingAttacks)],
        (0, 0),
    );

    let frame = ReportFrame::new().expect("Report frame loading");
    let report_handler =
        viewer.add_frame(frame, &[UiView::Visitors(VisitorViewTab::Letters)], (0, 0));
    ReportFrame::init_listeners(report_handler);

    let frame = QuestsFrame::new();
    let quests_handler =
        viewer.add_frame(frame, &[UiView::Visitors(VisitorViewTab::Quests)], (0, 0));
    QuestsFrame::init_listeners(quests_handler);

    /* Leaderboard */

    let menu = LeaderboardFrame::new().expect("Leaderboard loading");
    let leaderboard_handler = viewer.add_frame(menu, &[UiView::Leaderboard], (0, 0));
    leaderboard_handler.listen(LeaderboardFrame::network_message);

    let summary = TownSummaryFrame::new().expect("Town summary loading");
    viewer.add_frame(summary, &[UiView::TownHelp], (0, 0));

    /* Dialogue box */
    let dialogue = DialogueFrame::new().expect("Dialogue loading");
    let dialogue_handle = viewer.add_frame(dialogue, &[UiView::Dialogue], (0, 0));
    DialogueFrame::init_listeners(dialogue_handle);

    /* Civ / Religion view */
    let religion_frame = ReligionFrame::new();
    let religion_handle = viewer.add_frame(religion_frame, &[UiView::Religion], (0, 0));
    religion_handle.listen(ReligionFrame::signal);

    viewer
}
