use crate::game::story::scene::SceneIndex;
use crate::game::story::StoryAction;
use crate::game::units::workers::Worker;
use crate::gui::ui_state::UiState;
use crate::logging::ErrorQueue;
use crate::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;
use specs::prelude::*;
use specs::storage::HashMapStorage;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
/// If attached to an entity, will be triggered when the entity is selected.
pub struct EntityTrigger {
    pub actions: Vec<StoryAction>,
}

impl Game<'_, '_> {
    pub fn load_story_triggers(&mut self, story_state: &StoryState) -> PadlResult<()> {
        match story_state {
            StoryState::Initialized => {
                self.add_trigger_to_hero(EntityTrigger {
                    actions: vec![StoryAction::OpenScene(SceneIndex::Entrance, 0)],
                })?;
            }
            StoryState::TempleBuilt => {
                self.add_trigger_to_hero(EntityTrigger {
                    actions: vec![StoryAction::OpenScene(SceneIndex::TempleBuilt, 0)],
                })?;
            }
            StoryState::VisitorArrived
            | StoryState::FirstVisitorWelcomed
            | StoryState::FlowerPlanted
            | StoryState::MoreHappyVisitors
            | StoryState::TreePlanted
            | StoryState::StickGatheringStationBuild
            | StoryState::GatheringSticks => {}
            StoryState::ServantAccepted => {}
        }
        Ok(())
    }
    fn add_trigger_to_hero(&mut self, trigger: EntityTrigger) -> PadlResult<()> {
        let world = self.town_world();
        let (workers, entities) = world.system_data();
        let hero_id = Worker::find_hero(workers, entities)?;
        let mut triggers: WriteStorage<'_, EntityTrigger> = world.write_storage();
        triggers.insert(hero_id, trigger)?;
        Ok(())
    }
}

/// Triggers event on entity selection
pub struct EntityTriggerSystem {
    event_pool: EventPool,
}
impl EntityTriggerSystem {
    pub fn new(event_pool: EventPool) -> Self {
        EntityTriggerSystem { event_pool }
    }
    fn trigger(&mut self, trigger: EntityTrigger) -> PadlResult<()> {
        self.event_pool
            .send(GameEvent::StoryActions(trigger.actions))?;
        Ok(())
    }
}
impl<'a> System<'a> for EntityTriggerSystem {
    type SystemData = (
        WriteStorage<'a, EntityTrigger>,
        WriteExpect<'a, ErrorQueue>,
        ReadExpect<'a, UiState>,
    );
    fn run(&mut self, (mut triggers, mut errors, ui): Self::SystemData) {
        if let Some(e) = ui.selected_entity {
            if let Some(trigger) = triggers.remove(e) {
                let err = self.trigger(trigger);
                if let Err(e) = err {
                    errors.push(e);
                }
            }
        }
    }
}
