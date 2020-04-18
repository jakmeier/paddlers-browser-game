pub mod entity_trigger;
pub mod scene;

use crate::game::player_info::PlayerInfo;
use crate::game::Game;
use crate::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;

impl Game<'_, '_> {
    fn story_state(&self) -> StoryState {
        self.world.fetch::<PlayerInfo>().story_state
    }
    pub fn load_story_state(&mut self) -> PadlResult<()> {
        let story_state = self.story_state();
        if let Some(scene) = scene::SceneIndex::from_story_state(&story_state) {
            self.event_pool.send(GameEvent::LoadScene(scene))?;
        }
        self.load_story_triggers(&story_state)?;
        Ok(())
    }
}
