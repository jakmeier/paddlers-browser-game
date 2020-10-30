//! The game event manager handles events that are generated by SPECS systems.

use crate::{net::game_master_api::HttpNotifyVisitorSatisfied, game::{
    components::*, player_info::PlayerInfo, story::StoryAction, units::attackers::Visitor,
    units::attackers::*,
}};
use crate::gui::input::UiView;
use crate::gui::ui_state::Now;
use crate::gui::ui_state::UiState;
use crate::net::game_master_api::RestApiState;
use crate::net::request_foreign_town;
use crate::prelude::*;
use paddle::{Domain, NutsCheck};
use paddlers_shared_lib::api::story::StoryStateTransition;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

pub struct EventManager;

/// Coordinates of a village in world map
pub type VillageCoordinate = (i32, i32);

/// Send a GameEvent to the game event manager (replaces endpoints that were copied everywhere before)
pub fn game_event(ev: GameEvent) {
    paddle::nuts::publish(ev);
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameEvent {
    HoboSatisfied(Entity),
    HttpBuyProphet,
    LoadHomeVillage,
    LoadVillage(VillageKey),
    SendProphetAttack(VillageCoordinate),
    StoryActions(Vec<StoryAction>),
    SwitchToView(UiView),
    DisplayConfirmation(TextKey),
}

pub fn load_game_event_manager() {
    let event_manager_activity = nuts::new_domained_activity(EventManager, &Domain::Frame);
    event_manager_activity.subscribe_domained(|_, domain, event: &GameEvent| {
        let game: &mut Game = domain.try_get_mut().expect("Forgot to insert game?");
        game.try_handle_event(event.clone()).nuts_check(); // FIXME: Clone seems expensive
    });
}

impl Game {
    fn try_handle_event(&mut self, evt: GameEvent) -> PadlResult<()> {
        match evt {
            GameEvent::HoboSatisfied(id) => {
                let now = *self.world.fetch::<Now>();
                let resolution = *self.world.fetch::<ScreenResolution>();
                let town_world = self.town_world_mut();
                let mut rend_store = town_world.write_storage::<Renderable>();
                if let Some(mut rend) = rend_store.get_mut(id) {
                    change_duck_sprite_to_happy(&mut rend);
                }
                std::mem::drop(rend_store);
                let hobo_store = town_world.read_storage::<Visitor>();
                if let Some(hobo) = hobo_store.get(id) {
                    if !hobo.hurried {
                        let mut v_store = town_world.write_storage::<Moving>();
                        if v_store.get(id).is_none() {
                            // hobo currently stopped (in frontend)
                            // => Set it moving again, assuming it has been released by the game-master
                            let moving = release_and_move_visitor(hobo, resolution, now);
                            v_store.insert(id, moving)?;
                        }
                        // Tell backend that release might be required
                        let net_store = town_world.read_storage::<NetObj>();
                        let net_id = net_store.get(id).ok_or(PadlError::dev_err(
                            PadlErrorCode::MissingComponent("NetObj"),
                        ))?;
                        nuts::publish(HttpNotifyVisitorSatisfied {
                            hobo: HoboKey(net_id.id),
                        });
                    }
                }
            }
            GameEvent::HttpBuyProphet => {
                let player: PlayerInfo = *self.player().clone();
                crate::game::town::purchase_prophet(&player)?;
            }
            GameEvent::SendProphetAttack((x, y)) => {
                self.send_prophet_attack((x, y))?;
                // TODO: Only confirm if HTTP OK is returned
                // (Probably do this after cleaning pu network and promise handling)
                self.confirm_to_user("attack-sent".into())?;
            }
            GameEvent::SwitchToView(view) => {
                self.switch_view(view);
            }
            GameEvent::StoryActions(actions) => {
                for a in actions {
                    self.try_handle_story_action(a)?;
                }
            }
            GameEvent::LoadVillage(vid) => {
                self.town_context.load_foreign(vid);
                self.switch_view(UiView::Town);
                request_foreign_town(vid);
            }
            GameEvent::LoadHomeVillage => {
                self.town_context.reset_to_home();
            }
            GameEvent::DisplayConfirmation(t) => {
                self.confirm_to_user(t)?;
            }
        }
        Ok(())
    }
    fn try_handle_story_action(&mut self, action: StoryAction) -> PadlResult<()> {
        match action {
            StoryAction::OpenScene(scene, slide) => {
                paddle::share(crate::game::dialogue::LoadNewDialogueScene::new(
                    scene, slide,
                ));
                self.switch_view(UiView::Dialogue);
            }
            StoryAction::StoryProgress(new_story_state) => {
                let t = StoryStateTransition {
                    before: self.story_state(),
                    after: new_story_state,
                };
                nuts::publish(t);
                paddle::share(crate::game::dialogue::NewStoryState { new_story_state });
            }
            StoryAction::TownSelectEntity(e) => {
                let world = self.town_context.world();
                world.write_resource::<UiState>().selected_entity = e;
            }
        }
        Ok(())
    }
}
