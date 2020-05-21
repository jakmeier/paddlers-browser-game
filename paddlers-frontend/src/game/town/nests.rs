use crate::game::components::NetObj;
use crate::game::town::*;
use crate::net::game_master_api::RestApiState;
use crate::net::state::current_village;
use paddlers_shared_lib::api::attacks::InvitationDescriptor;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
/// A nest can provide space for some hobos and those can be invited to other towns
pub struct Nest {
    hobos: Vec<Entity>,
    capacity: usize,
}

impl Nest {
    pub fn new(capacity: usize) -> Self {
        Self {
            hobos: vec![],
            capacity,
        }
    }
    pub fn add_hobo(&mut self, hobo: Entity) {
        self.hobos.push(hobo);
    }
    pub fn clear_hobos(&mut self, lazy: &LazyUpdate) -> PadlResult<()> {
        if self.hobos.len() == 0 {
            PadlErrorCode::NestEmpty.usr()
        } else {
            let hobos = std::mem::replace(&mut self.hobos, vec![]);
            lazy.exec_mut(move |world| world.delete_entities(&hobos).expect("Delete failed"));
            Ok(())
        }
    }
}
impl Town {
    pub fn nest_invitation<'a>(
        &mut self,
        nest: &mut Nest,
        netids: ReadStorage<'a, NetObj>,
        e: Entity,
        lazy: &LazyUpdate,
        ep: &EventPool,
    ) -> PadlResult<()> {
        nest.clear_hobos(lazy)?;
        let netid = netids
            .get(e)
            .ok_or(PadlError::dev_err(PadlErrorCode::MissingComponent(
                "NetObj",
            )))?;
        let msg = InvitationDescriptor {
            nest: netid.as_building().expect("building"),
            to: current_village(),
        };
        RestApiState::get().http_invite(msg)?;
        ep.send(GameEvent::DisplayConfirmation("invitation-sent".into()))?;
        Ok(())
    }
}
