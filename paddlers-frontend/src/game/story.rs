pub mod entity_trigger;
pub mod scene;

use crate::game::{player_info::PlayerInfo, story::scene::SceneIndex, Game};
use crate::prelude::*;
use paddlers_shared_lib::story::{story_state::StoryState, story_trigger::StoryChoice};
use scene::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogueAction {
    OpenScene(SceneIndex, SlideIndex),
    StoryProgress(StoryState, Option<StoryChoice>),
    TownSelectEntity(Option<specs::Entity>),
    SettleHobo,
}

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
        | StoryState::TempleBuilt => None,
        StoryState::ServantAccepted => Some((SceneIndex::Entrance, 5)),
        // TODO
        StoryState::DialogueBalanceA => None,
        StoryState::DialogueBalanceB => None,
    }
}
