use specs::prelude::*;
use quicksilver::geom::*;
use crate::gui::render::Renderable;
use crate::gui::gui_components::UiBox;
use crate::gui::utils::*;
use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct NetObj {
    pub id: i64, 
    // Maybe one could add a type field to make it unique. But not sure right now if it is even necessary.
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
/// Entitiy that can contain other entities (E.g. House has units inside)
pub struct EntityContainer {
    pub children: Vec<Entity>,
    pub ui: UiBox<Entity>,
    pub capacity: usize,
    pub task: TaskType,
}

impl EntityContainer {
    pub fn new(capacity: usize, task: TaskType) -> Self {
        EntityContainer {
            children: vec![],
            ui: UiBox::new(3,3, 0.0, 1.0),
            capacity: capacity,
            task: task,
        }
    }
    pub fn can_add_entity(&self) -> bool {
        self.children.len() < self.capacity
    }
    pub fn add_entity_unchecked(&mut self, e: Entity, rend: &Renderable) {
        self.children.push(e);
        let style = match rend.kind {
            RenderVariant::ImgWithImgBackground(img, _) 
            | RenderVariant::ImgWithColBackground(img, _)
            | RenderVariant::Img(img)
            => {
                RenderVariant::ImgWithColBackground(img, GREY)
            },
        };
        self.ui.add_with_render_variant(style, e);
    }
    pub fn worker_to_release<'a>(&mut self, mouse: &Vector) -> Option<Entity> {
        if let Some(entity_to_release) = self.ui.click_and_remove(*mouse) {
            self.children.remove_item(&entity_to_release);
            return Some(entity_to_release);
        }
        None
    }
    pub fn count(&self) -> usize {
        self.children.len()
    }
}