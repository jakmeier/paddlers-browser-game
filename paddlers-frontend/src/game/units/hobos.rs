use crate::game::town::nests::Nest;
use crate::game::town::TownContext;
use crate::game::{components::NetObj, movement::Position, Game};
use crate::gui::{render::Renderable, sprites::*, utils::*, z::Z_UNITS};
use crate::net::graphql::query_types::{
    hobos_query::HobosQueryVillageHobosNest, HobosQueryResponse, HobosQueryUnitColor,
};
use crate::prelude::*;
use paddlers_shared_lib::prelude::*;
use quicksilver::geom::*;
use specs::prelude::*;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Hobo;

impl Game<'_, '_> {
    pub fn hobo_key(&self, e: Entity) -> PadlResult<HoboKey> {
        let net = self.town_world().read_storage::<NetObj>();
        if let Some(obj) = net.get(e) {
            Ok(HoboKey(obj.id))
        } else {
            PadlErrorCode::MissingComponent("Prophet Hobo").dev()
        }
    }
}

pub fn insert_hobos(ctx: &mut TownContext, hobos: HobosQueryResponse) -> PadlResult<()> {
    // Insert idle prophets
    ctx.town_mut().idle_prophets = hobos
        .iter()
        .filter(|h| h.color == Some(HobosQueryUnitColor::PROPHET))
        .filter(|h| h.idle)
        .map(|h| new_prophet(ctx.world_mut(), &h.id))
        .collect::<Result<Vec<_>, _>>()?;
    // Insert sitting hobos
    hobos
        .iter()
        .filter(|h| h.idle)
        .filter(|h| h.nest.is_some())
        .map(|h| new_sitting_hobo(ctx.world_mut(), &h.id, h.nest.as_ref().unwrap()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(())
}

fn new_prophet(world: &mut World, id: &str) -> PadlResult<Entity> {
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

fn new_sitting_hobo(
    world: &mut World,
    id: &str,
    nest: &HobosQueryVillageHobosNest,
) -> PadlResult<Entity> {
    let id = id
        .parse()
        .map_err(|_| PadlError::dev_err(PadlErrorCode::InvalidGraphQLData("HoboId")))?;
    let nest_id = nest
        .id
        .parse()
        .map_err(|_| PadlError::dev_err(PadlErrorCode::InvalidGraphQLData("BuildingId")))?;
    let ul = world.fetch::<ScreenResolution>().unit_length();
    let pos = Vector::new(nest.x as f32 * ul, nest.y as f32 * ul);
    let size = (ul, ul);
    let rend = RenderVariant::Img(SpriteSet::Simple(SingleSprite::SittingYellowDuck));

    let entity = world
        .create_entity()
        .with(NetObj::hobo(id))
        .with(Hobo)
        .with(Renderable::new(rend))
        .with(Position::new(pos, size, Z_UNITS))
        .build();

    let nest_entity = NetObj::lookup_building(nest_id, &world.read_storage(), &world.entities())?;
    let mut nests = world.write_storage::<Nest>();
    let nest =
        nests
            .get_mut(nest_entity)
            .ok_or(PadlError::dev_err(PadlErrorCode::MissingComponent(
                "NetObj",
            )))?;
    nest.add_hobo(entity);
    Ok(entity)
}
