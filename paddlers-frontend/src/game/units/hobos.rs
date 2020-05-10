use crate::game::{components::NetObj, Game};
use crate::net::graphql::query_types::{HobosQueryResponse, HobosQueryUnitColor};
use crate::prelude::*;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Hobo;

impl Game<'_, '_> {
    pub fn insert_hobos(&mut self, hobos: HobosQueryResponse) -> PadlResult<()> {
        // Insert idle prophets
        self.town_mut().idle_prophets = hobos
            .iter()
            .filter(|h| h.color == Some(HobosQueryUnitColor::PROPHET))
            .filter(|h| h.idle)
            .map(|h| new_hobo(&mut self.world, &h.id))
            .collect::<Result<Vec<_>, _>>()?;
        // Ignore all other hobos (for now)
        Ok(())
    }
    pub fn hobo_key(&self, e: Entity) -> PadlResult<HoboKey> {
        let net = self.world.read_storage::<NetObj>();
        if let Some(obj) = net.get(e) {
            Ok(HoboKey(obj.id))
        } else {
            PadlErrorCode::MissingComponent("Prophet Hobo").dev()
        }
    }
}

fn new_hobo(world: &mut World, id: &str) -> PadlResult<Entity> {
    let id = id
        .parse()
        .map_err(|_| PadlError::dev_err(PadlErrorCode::InvalidGraphQLData("HoboId")))?;
    let entity = world
        .create_entity()
        .with(NetObj::hobo(id))
        .with(Hobo)
        .build();
    Ok(entity)
}
