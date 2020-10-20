use crate::game::{
    components::*, fight::*, forestry::ForestrySystem, mana::extend_transformed,
    movement::MoveSystem, story::entity_trigger::EntityTriggerSystem, town::Town,
    units::worker_system::WorkerSystem, units::workers::Worker, Game,
};
use crate::gui::{
    input::{left_click::TownLeftClickSystem, MouseState},
    ui_state::*,
};
use crate::init::quicksilver_integration::Signal;
use crate::prelude::*;
use paddle::quicksilver_compat::graphics::Mesh;
use paddle::quicksilver_compat::{MouseButton, Shape, Vector};
use paddle::{Frame, Window};
use specs::prelude::*;
use std::ops::Deref;

pub(crate) struct TownFrame<'a, 'b> {
    left_click_dispatcher: Dispatcher<'a, 'b>,
    town_dispatcher: Dispatcher<'a, 'b>,
    // Graphics optimization
    pub background_cache: Option<Mesh>,
}

impl<'a, 'b> Frame for TownFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;

    fn update(&mut self, state: &mut Self::State) -> Result<(), Self::Error> {
        state.prepare_town_resources();
        let world = state.town_world_mut();
        world.maintain();
        self.town_dispatcher.dispatch(world);

        Ok(())
    }
    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        {
            // FIXME: This should not be necessary if resources are defined properly
            state.prepare_town_resources();

            let ul = state.world.fetch::<ScreenResolution>().unit_length();
            let tick = state.world.read_resource::<ClockTick>().0;
            let asset = &mut state.sprites;
            let town = state.town_context.town_mut();
            if self.background_cache.is_none() {
                self.background_cache = Some(Mesh::new());
                town.render_background(self.background_cache.as_mut().unwrap(), asset, ul)?;
            }
            self.background_cache.as_ref().unwrap().vertices.len();
            extend_transformed(
                window.mesh(),
                self.background_cache.as_ref().unwrap(),
                Default::default(),
            );
            town.render(window, asset, tick, ul)?;
        }
        state.draw_town_main(window)?;
        Ok(())
    }

    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) -> Result<(), Self::Error> {
        let town_world = state.town_world();
        let ui_state = town_world.fetch_mut::<ViewState>();

        // This can be removed once the frame positions are checked properly before right_click is called
        let mouse_pos: Vector = pos.into();
        let in_main_area = mouse_pos.overlaps_rectangle(&(*ui_state).main_area);
        if !in_main_area {
            return Ok(());
        }
        std::mem::drop(ui_state);

        let ms = MouseState(pos.into(), Some(MouseButton::Left));
        state.town_world_mut().insert(ms);
        self.left_click_dispatcher.dispatch(state.town_world());
        Ok(())
    }
    fn right_click(&mut self, state: &mut Self::State, pos: (i32, i32)) -> Result<(), Self::Error> {
        let town_world = state.town_world();
        let view_state = town_world.fetch_mut::<ViewState>();

        // This can be removed once the frame positions are checked properly before right_click is called
        let mouse_pos: Vector = pos.into();
        let in_main_area = mouse_pos.overlaps_rectangle(&(*view_state).main_area);
        if !in_main_area {
            return Ok(());
        }

        // Right click cancels grabbed item (take removes from option)
        let mut ui_state = town_world.fetch_mut::<UiState>();
        if ui_state.take_grabbed_item().is_some() {
            return Ok(());
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
                    let start = town
                        .resolution
                        .next_tile_in_direction(from.area.pos, movement.momentum);
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

        Ok(())
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
    pub fn signal(
        &mut self,
        state: &mut Game<'static, 'static>,
        msg: &Signal,
    ) -> Result<(), PadlError> {
        match msg {
            Signal::PlayerInfoUpdated => {
                state.update_temple()?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl<'a, 'b> Game<'a, 'b> {
    /// Copy over Resources from global world to town world
    // Note: This is ugly but how else to share resources?
    //       The best solution I could think of would be to call all systems directly, instead of using a dispatcher.
    pub(crate) fn prepare_town_resources(&mut self) {
        self.copy_res::<Now>();
        self.copy_res::<ClockTick>();
        self.copy_res::<ScreenResolution>();
        self.copy_res::<ViewState>();
        self.copy_res::<UiView>();
    }
    fn copy_res<T: Clone + 'static>(&mut self) {
        let res: T = self.world.read_resource::<T>().deref().clone();
        self.town_context.world_mut().insert::<T>(res);
    }
}
