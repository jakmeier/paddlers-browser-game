pub mod entity_trigger;

use crate::{
    game::{player_info::PlayerInfo, Game},
    net::graphql::{ForceRequest, PeriodicalSyncRequest},
};
use crate::{net::graphql::ScheduledRequest, prelude::*};
use paddle::NutsCheck;
use paddlers_shared_lib::story::{story_action::StoryAction, story_state::StoryState};
use paddlers_shared_lib::{specification_types::*, story::story_trigger::StoryTrigger};

impl Game {
    pub fn set_story_state(&self, s: StoryState) {
        self.world.fetch_mut::<PlayerInfo>().set_story_state(s);
    }
    pub fn story_state(&self) -> StoryState {
        self.world.fetch::<PlayerInfo>().story_state()
    }
    pub fn load_story_state(&mut self) -> PadlResult<()> {
        let story_state = self.story_state();
        if let Some((scene, slide)) = select_dialogue_scene(story_state) {
            crate::game::game_event_manager::game_event(GameEvent::DialogueActions(vec![
                DialogueAction::OpenScene(scene, slide),
            ]));
        }
        self.load_story_triggers(&story_state)?;
        Ok(())
    }
    // TODO: This should be called everywhere in the frontend where a story state changing action happens. And then the code should be changed to not do the full const computation every time-
    pub fn handle_story_trigger(&mut self, trigger: StoryTrigger) {
        let story_state = self.story_state();
        if let Some(t) = story_state.transition(&trigger) {
            if t.next_state != story_state {
                self.set_story_state(t.next_state);
                self.load_story_state().nuts_check();
                nuts::publish(ForceRequest::SyncAsap(PeriodicalSyncRequest::PlayerInfo));
            }
            let mut mana_changed = false;
            let mut visitors_sent = false;
            let mut new_quest = false;
            for action in t.actions.into_iter() {
                if let StoryAction::AddMana(_) = action {
                    mana_changed = true;
                }
                if let StoryAction::SendHobo(_) = action {
                    visitors_sent = true;
                }
                if let StoryAction::StartQuest(_) = action {
                    new_quest = true;
                }
            }
            if mana_changed {
                // one could go and add Mana manually. In favour of a more widely applicable solution, I want to trigger a reload instead (for now).
                nuts::publish(ForceRequest::Extra(ScheduledRequest::Workers, 3));
            }
            if visitors_sent {
                nuts::publish(ForceRequest::Extra(ScheduledRequest::Visitors, 3));
            }
            if new_quest {
                nuts::publish(ForceRequest::Extra(ScheduledRequest::Quests, 3));
            }
        }
    }
}

pub fn select_dialogue_scene(story_state: StoryState) -> Option<(SceneIndex, SlideIndex)> {
    match story_state {
        StoryState::Initialized
        | StoryState::VisitorArrived
        | StoryState::FirstVisitorWelcomed
        | StoryState::BuildingWatergate
        | StoryState::WatergateBuilt
        | StoryState::PickingPrimaryCivBonus
        | StoryState::SolvingPrimaryCivQuestPartA
        | StoryState::SolvingPrimaryCivQuestPartB
        | StoryState::SolvingSecondaryQuestA
        | StoryState::SolvingSecondaryQuestB
        | StoryState::AllDone
        | StoryState::VisitorQueued
        | StoryState::WelcomeVisitorQuestStarted
        | StoryState::UnlockingInvitationPathA
        | StoryState::UnlockingInvitationPathB
        | StoryState::TempleBuilt => None,
        StoryState::ServantAccepted => Some((SceneIndex::Entrance, 5)),
        StoryState::DialogueBalanceA => None,
        StoryState::DialogueBalanceB => None,
    }
}
