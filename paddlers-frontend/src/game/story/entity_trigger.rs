use crate::game::units::workers::Worker;
use crate::game::{story::DialogueAction, town::Town};
use crate::gui::ui_state::UiState;
use crate::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;
use specs::prelude::*;
use specs::storage::HashMapStorage;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
/// If attached to an entity, will be triggered when the entity is selected.
pub struct EntityTrigger {
    pub actions: Vec<DialogueAction>,
}

impl Game {
    pub fn load_story_triggers(&mut self, story_state: &StoryState) -> PadlResult<()> {
        match story_state {
            StoryState::Initialized => {
                self.add_trigger_to_hero(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(SceneIndex::Entrance, 0)],
                })?;
            }
            StoryState::TempleBuilt => {
                self.add_trigger_to_hero(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(SceneIndex::BuildWatergate, 0)],
                })?;
            }
            StoryState::WatergateBuilt => {
                self.add_trigger_to_hero(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(SceneIndex::ExplainWatergate, 0)],
                })?;
            }
            StoryState::VisitorArrived => {
                self.add_trigger_to_hero(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(SceneIndex::WelcomeVisitor, 0)],
                })?;
            }
            StoryState::UnlockingInvitationPathA | StoryState::UnlockingInvitationPathB => {
                self.add_trigger_to_temple(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(
                        SceneIndex::UnlockingInvitation,
                        0,
                    )],
                })?;
            }
            StoryState::DialogueBalanceA => {
                self.add_trigger_to_temple(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(SceneIndex::VisitorBalanceTown, 0)],
                })?;
            }
            StoryState::DialogueBalanceB => {
                self.add_trigger_to_temple(EntityTrigger {
                    actions: vec![DialogueAction::OpenScene(SceneIndex::TownBalanceVisitor, 0)],
                })?;
            }
            StoryState::VisitorQueued
            | StoryState::FirstVisitorWelcomed
            | StoryState::ServantAccepted
            | StoryState::BuildingWatergate
            | StoryState::PickingPrimaryCivBonus
            | StoryState::SolvingPrimaryCivQuestPartA
            | StoryState::SolvingPrimaryCivQuestPartB
            | StoryState::SolvingSecondaryQuestA
            | StoryState::SolvingSecondaryQuestB
            | StoryState::WelcomeVisitorQuestStarted
            | StoryState::AllDone => {}
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
    fn add_trigger_to_temple(&mut self, trigger: EntityTrigger) -> PadlResult<()> {
        let world = self.home_town_world();
        if let Some(e) = Town::find_building_entity(world, BuildingType::Temple) {
            let mut triggers: WriteStorage<'_, EntityTrigger> = world.write_storage();
            triggers.insert(e, trigger)?;
        }
        Ok(())
    }
}

/// Triggers event on entity selection
pub struct EntityTriggerSystem;
impl EntityTriggerSystem {
    pub fn new() -> Self {
        EntityTriggerSystem
    }
    fn trigger(&mut self, trigger: EntityTrigger) -> PadlResult<()> {
        crate::game::game_event_manager::game_event(GameEvent::DialogueActions(trigger.actions));
        Ok(())
    }
}
impl<'a> System<'a> for EntityTriggerSystem {
    type SystemData = (WriteStorage<'a, EntityTrigger>, ReadExpect<'a, UiState>);
    fn run(&mut self, (mut triggers, ui): Self::SystemData) {
        if let Some(e) = ui.selected_entity {
            if let Some(trigger) = triggers.remove(e) {
                let err = self.trigger(trigger);
                if let Err(e) = err {
                    nuts::publish(e);
                }
            }
        }
    }
}
