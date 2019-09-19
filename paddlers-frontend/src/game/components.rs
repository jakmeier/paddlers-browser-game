use crate::game::units::workers::Worker;
use specs::prelude::*;
use crate::prelude::*;
use crate::gui::{
    animation::AnimationState,
    render::Renderable,
    gui_components::UiBox,
    utils::*,
    input::Clickable,
};
use super::movement::*;
use super::units::attackers::{Attacker};
use super::fight::*;
use super::forestry::*;
use super::map::{VillageMetaInfo, MapPosition};



pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<MapPosition>();
    world.register::<Moving>();
    world.register::<Renderable>();
    world.register::<Clickable>();
    world.register::<Attacker>();
    world.register::<Worker>();
    world.register::<Range>();
    world.register::<Health>();
    world.register::<NetObj>();
    world.register::<AnimationState>();
    world.register::<EntityContainer>();
    world.register::<ForestComponent>();
    world.register::<VillageMetaInfo>();
    world.register::<UiMenu>();
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct NetObj {
    pub id: i64, 
    // Maybe one could add a type field to make it unique. But not sure right now if it is even necessary.
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