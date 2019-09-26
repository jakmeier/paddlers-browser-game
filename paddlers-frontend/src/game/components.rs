use crate::game::units::workers::Worker;
use specs::prelude::*;
use crate::prelude::*;
use crate::gui::{
    utils::*,
    gui_components::UiBox,
};
pub use crate::gui::{
    animation::AnimationState,
    render::Renderable,
    input::Clickable,
};
pub use super::movement::{Moving, Position};
pub use super::fight::{Health, Range};
pub use super::forestry::ForestComponent;
pub use super::map::{VillageMetaInfo, MapPosition};
pub use super::status_effects::{StatusEffects};



pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<MapPosition>();
    world.register::<Moving>();
    world.register::<Renderable>();
    world.register::<Clickable>();
    world.register::<Worker>();
    world.register::<Range>();
    world.register::<Health>();
    world.register::<NetObj>();
    world.register::<AnimationState>();
    world.register::<EntityContainer>();
    world.register::<ForestComponent>();
    world.register::<VillageMetaInfo>();
    world.register::<UiMenu>();
    world.register::<StatusEffects>();
}

/// Required to give NetObj values a context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NetObjType {
    Hobo,
    Worker,
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
/// Identifies an entity across views (frontend/backend(s))
pub struct NetObj {
    pub id: i64,
    typ: NetObjType,
}

impl NetObj {
    pub fn hobo(id: i64) -> Self {
        NetObj {
            id,
            typ: NetObjType::Hobo,
        }
    }
    pub fn worker(id: i64) -> Self {
        NetObj {
            id,
            typ: NetObjType::Worker,
        }
    }
    pub fn lookup_hobo(net_id: i64, net_ids: &ReadStorage<NetObj>, entities: &Entities) -> PadlResult<Entity> {
        Self::lookup_entity(net_id, NetObjType::Hobo, net_ids, entities)
    }
    pub fn lookup_worker(net_id: i64, net_ids: &ReadStorage<NetObj>, entities: &Entities) -> PadlResult<Entity> {
        Self::lookup_entity(net_id, NetObjType::Worker, net_ids, entities)
    }
    fn lookup_entity(net_id: i64, net_type: NetObjType, net_ids: &ReadStorage<NetObj>, entities: &Entities) -> PadlResult<Entity> {
        // TODO: Efficient NetId lookup
        for (e, n) in (entities, net_ids).join() {
            if n.id == net_id && n.typ == net_type {
                return Ok(e);
            }
        }
        PadlErrorCode::UnknownNetObj(NetObj{ id: net_id, typ: net_type }).dev()
    }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
/// Clickable menu that pop up when entity is selected
pub struct UiMenu {
    pub ui: UiBox,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
/// Entity that can contain other entities (E.g. House has units inside)
pub struct EntityContainer {
    pub children: Vec<Entity>,
    pub capacity: usize,
    pub task: TaskType,
}

impl EntityContainer {
    pub fn new(capacity: usize, task: TaskType) -> Self {
        EntityContainer {
            children: vec![],
            capacity: capacity,
            task: task,
        }
    }
    pub fn can_add_entity(&self) -> bool {
        self.children.len() < self.capacity
    }
    pub fn add_entity_unchecked(&mut self, e: Entity, rend: &Renderable, ui: &mut UiMenu) {
        self.children.push(e);
        let style = match rend.kind {
            RenderVariant::ImgWithImgBackground(img, _) 
            | RenderVariant::ImgWithColBackground(img, _)
            | RenderVariant::Img(img)
            | RenderVariant::ImgWithHoverAlternative(img,_)
            => {
                RenderVariant::ImgWithColBackground(img, GREY)
            },
            RenderVariant::Hide => {
                RenderVariant::Hide
            }
        };
        ui.ui.add_with_render_variant(style, e);
    }
    pub fn remove_entity<'a>(&mut self, e: Entity) {
        self.children.remove_item(&e);
    }
    pub fn count(&self) -> usize {
        self.children.len()
    }
}

impl UiMenu {
    pub fn new_entity_container() -> Self {
        UiMenu {
            ui: UiBox::new(3,3, 0.0, 1.0)
        }
    }
}