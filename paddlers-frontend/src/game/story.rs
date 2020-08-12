pub mod entity_trigger;
pub mod scene;

use crate::game::{player_info::PlayerInfo, story::scene::SceneIndex, Game};
use crate::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;
use scene::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StoryAction {
    OpenScene(SceneIndex, SlideIndex),
    StoryProgress(StoryState),
    TownSelectEntity(Option<specs::Entity>),
}

impl Game<'_, '_> {
    pub fn set_story_state(&self, s: StoryState) {
        self.world.fetch_mut::<PlayerInfo>().set_story_state(s);
    }
    pub fn story_state(&self) -> StoryState {
        self.world.fetch::<PlayerInfo>().story_state()
    }
    pub fn load_story_state(&mut self) -> PadlResult<()> {
        let story_state = self.story_state();
        if let Some((scene, slide)) = select_dialogue_scene(story_state) {
            crate::game::game_event_manager::game_event(GameEvent::StoryActions(vec![
                StoryAction::OpenScene(scene, slide),
            ]));
        }
        self.load_story_triggers(&story_state)?;
        Ok(())
    }
}

pub fn select_dialogue_scene(story_state: StoryState) -> Option<(SceneIndex, SlideIndex)> {
    match story_state {
        StoryState::Initialized
        | StoryState::TempleBuilt
        | StoryState::VisitorArrived
        | StoryState::FirstVisitorWelcomed
        | StoryState::FlowerPlanted
        | StoryState::MoreHappyVisitors
        | StoryState::TreePlanted
        | StoryState::StickGatheringStationBuild
        | StoryState::GatheringSticks => None,
        StoryState::ServantAccepted => Some((SceneIndex::Entrance, 5)),
    }
}
