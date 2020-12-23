use crate::game::toplevel::Signal;
use crate::prelude::*;
use crate::{
    game::{
        components::*, fight::*, forestry::ForestrySystem, movement::MoveSystem,
        story::entity_trigger::EntityTriggerSystem, town::Town, units::worker_system::WorkerSystem,
        units::workers::Worker, Game,
    },
    gui::input::MouseButton,
};
use crate::{
    gui::{
        input::{left_click::TownLeftClickSystem, MouseState},
        ui_state::*,
    },
    resolution::TOWN_TILE_S,
};
use paddle::quicksilver_compat::graphics::Mesh;
use paddle::quicksilver_compat::{Shape, Vector};
use paddle::{DisplayArea, Frame, NutsCheck};
use specs::prelude::*;
use std::ops::Deref;

use super::tiling;

pub(crate) struct TownFrame<'a, 'b> {
    left_click_dispatcher: Dispatcher<'a, 'b>,
    town_dispatcher: Dispatcher<'a, 'b>,
    // Graphics optimization
    pub background_cache: Option<Mesh>,
}

impl<'a, 'b> Frame for TownFrame<'a, 'b> {
    type State = Game;
    const WIDTH: u32 = crate::resolution::MAIN_AREA_W;
    const HEIGHT: u32 = crate::resolution::MAIN_AREA_H;

    fn update(&mut self, state: &mut Self::State) {
        state.prepare_town_resources();
        let world = state.town_world_mut();
        world.maintain();
        self.town_dispatcher.dispatch(world);
    }
    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        {
            // FIXME: This should not be necessary if resources are defined properly
            state.prepare_town_resources();

            let ul = TOWN_TILE_S as f32;
            let tick = state.world.read_resource::<ClockTick>().0;
            let asset = &mut state.sprites;
            let town = state.town_context.town_mut();
            if self.background_cache.is_none() {
                self.background_cache = Some(Mesh::new());
                town.render_background(self.background_cache.as_mut().unwrap(), asset, ul)
                    .nuts_check();
            }
            self.background_cache.as_ref().unwrap().vertices.len();
            window.draw_triangles(self.background_cache.as_ref().unwrap());
            town.render(window, asset, tick, ul).nuts_check();
        }
        state.draw_town_main(window);
    }

    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        let mouse_pos: Vector = pos.into();
        println!("DEBUG_XXX Left click at town frame {:?}", mouse_pos);

        let ms = MouseState(pos.into(), Some(MouseButton::Left));
        state.town_world_mut().insert(ms);
        self.left_click_dispatcher.dispatch(state.town_world());
    }
    fn right_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        let town_world = state.town_world();
        let mouse_pos: Vector = pos.into();

        // Right click cancels grabbed item (take removes from option)
        let mut ui_state = town_world.fetch_mut::<UiState>();
        if ui_state.take_grabbed_item().is_some() {
            return;
        }

        let entities = town_world.entities();
        let town = town_world.fetch::<Town>();
        let mut worker = town_world.write_component::<Worker>();
        let mut containers = town_world.write_component::<EntityContainer>();
        let position = town_world.read_component::<Position>();
        let moving = town_world.read_component::<Moving>();
        let clickable = town_world.read_component::<Clickable>();
        let net_ids = town_world.read_component::<NetObj>();
        let mana = town_world.read_component::<Mana>();

        let maybe_top_hit = Town::clickable_lookup(&entities, mouse_pos, &position, &clickable);

        if let Some(e) = (*ui_state).selected_entity {
            if let Some(worker) = worker.get_mut(e) {
                let maybe_job = worker.task_on_right_click(&mouse_pos, &town);
                if let Some((job, destination)) = maybe_job {
                    let target = maybe_top_hit.and_then(|e| net_ids.get(e)).map(|n| n.id);
                    let (from, movement) = (&position, &moving).join().get(e, &entities).unwrap();
                    let start = tiling::next_tile_in_direction(from.area.pos, movement.momentum);
                    let new_job = (job, target);
                    worker.new_order(
                        e,
                        start,
                        new_job,
                        destination,
                        &*town,
                        &mut containers,
                        &mana,
                    );
                }
            }
        }
    }
    fn mouse_move(&mut self, state: &mut Self::State, mouse_pos: (i32, i32)) {
        let mouse_pos = Vector::from(mouse_pos);
        let mut ui_state = state.world.write_resource::<UiState>();
        let position = state.world.read_storage::<Position>();
        let entities = state.world.entities();
        (*ui_state).hovered_entity = None;
        for (e, pos) in (&entities, &position).join() {
            if mouse_pos.overlaps_rectangle(&pos.area) {
                (*ui_state).hovered_entity = Some(e);
                break;
            }
        }
    }
}

impl<'a, 'b> TownFrame<'a, 'b> {
    pub fn new() -> Self {
        let left_click_dispatcher = DispatcherBuilder::new()
            .with(TownLeftClickSystem::new(), "", &[])
            .build();

        let town_dispatcher = DispatcherBuilder::new()
            .with(WorkerSystem::new(), "work", &[])
            .with(MoveSystem, "move", &["work"])
            .with(FightSystem::new(), "fight", &["move"])
            .with(ForestrySystem, "forest", &[])
            .with(EntityTriggerSystem::new(), "ets", &[])
            .build();

        TownFrame {
            left_click_dispatcher,
            background_cache: None,
            town_dispatcher,
        }
    }
    pub fn signal(&mut self, state: &mut Game, msg: &Signal) {
        match msg {
            Signal::PlayerInfoUpdated => {
                state.update_temple().nuts_check();
            }
            _ => {}
        }
    }
}
impl Game {
    /// Copy over Resources from global world to town world
    // Note: This is ugly but how else to share resources?
    //       The best solution I could think of would be to call all systems directly, instead of using a dispatcher.
    pub(crate) fn prepare_town_resources(&mut self) {
        self.copy_res::<Now>();
        self.copy_res::<ClockTick>();
        self.copy_res::<UiView>();
    }
    fn copy_res<T: Clone + 'static>(&mut self) {
        let res: T = self.world.read_resource::<T>().deref().clone();
        self.town_context.world_mut().insert::<T>(res);
    }
}
