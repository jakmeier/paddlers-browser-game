use crate::game::toplevel::Signal;
use crate::gui::{
    animation::AnimationState, input::Grabbable, sprites::Sprites, sprites::*, ui_state::*,
    utils::colors::*, utils::*, z::*,
};
use crate::{
    game::{
        components::*, fight::*, forestry::ForestrySystem, movement::MoveSystem,
        story::entity_trigger::EntityTrigger, story::entity_trigger::EntityTriggerSystem,
        town::Town, units::worker_system::WorkerSystem, units::workers::Worker, Game,
    },
    gui::input::{left_click::TownLeftClickSystem, MouseState},
    prelude::*,
    resolution::TOWN_TILE_S,
};

use paddle::*;
use paddle::{
    quicksilver_compat::{Color, Shape},
    FitStrategy,
};

use paddlers_shared_lib::story::{story_state::StoryState, story_trigger::StoryTrigger};
use specs::prelude::*;
use std::ops::Deref;

use super::{tiling, town_render::draw_shiny_border, visitor_gate::WatergateQueueSystem};

pub(crate) struct TownFrame<'a, 'b> {
    left_click_dispatcher: Dispatcher<'a, 'b>,
    town_dispatcher: Dispatcher<'a, 'b>,
    mouse: PointerTracker,
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
    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, timestamp: f64) {
        {
            // FIXME: This should not be necessary if resources are defined properly
            state.prepare_town_resources();

            let town = state.town_context.town_mut();
            let water_shader = &state.shaders.water;
            window.update_uniform(
                water_shader.render_pipeline(),
                "Time",
                &UniformValue::F32((timestamp / 1000.0) as f32),
            );
            town.render_water(window, water_shader);
            town.render(window, &mut state.sprites);
        }

        let world = state.town_context.world();
        let ui_state = world.read_resource::<UiState>();
        let hovered_entity = ui_state.hovered_entity;
        let grabbed_item = ui_state.grabbed_item().clone();
        std::mem::drop(ui_state);

        let sprites = &mut state.sprites;
        if let Some(entity) = hovered_entity {
            render_hovering(world, window, sprites, entity);
        }
        if let Some(grabbed) = grabbed_item {
            self.render_grabbed_item(&state.town_context.town(), window, sprites, &grabbed);
        } else {
            // window.set_cursor(MouseCursor::Default);
        }

        render_town_entities(world, window, sprites);
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        match event {
            PointerEvent(PointerEventType::PrimaryClick, pos) => self.left_click(state, pos),
            PointerEvent(PointerEventType::SecondaryClick, pos)
            | PointerEvent(PointerEventType::DoubleClick, pos) => {
                self.right_click(state, pos);
            }
            PointerEvent(PointerEventType::Move, _pos) => self.mouse_move(state),
            _ => { /* NOP */ }
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
            .with(WatergateQueueSystem, "wgq", &[])
            .build();

        TownFrame {
            left_click_dispatcher,
            town_dispatcher,
            mouse: Default::default(),
        }
    }
    pub fn signal(&mut self, state: &mut Game, msg: &Signal) {
        match msg {
            Signal::PlayerInfoUpdated => {
                state.update_temple().nuts_check();
            }
            Signal::BuildingBuilt(bt) => {
                state.home_town_world_mut().maintain();
                state.handle_story_trigger(StoryTrigger::BuildingBuilt(*bt));
                // TODO: Can these be integrated in specs?
                if *bt == BuildingType::Temple {
                    state
                        .load_story_triggers(&StoryState::TempleBuilt)
                        .nuts_check();
                }
                if *bt == BuildingType::Watergate {
                    state.town_mut().refresh_attacker_direction();
                    state.refresh_visitor_gate();
                }
            }
            Signal::BuildingUpgraded(BuildingType::Watergate) => {
                state.refresh_visitor_gate();
            }
            _ => {}
        }
    }
    fn left_click(&mut self, state: &mut Game, pos: Vector) {
        let ms = MouseState(pos);
        state.town_world_mut().insert(ms);
        self.left_click_dispatcher.dispatch(state.town_world());
    }
    fn right_click(&mut self, state: &mut Game, mouse_pos: Vector) {
        let town_world = state.town_world();

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
    fn mouse_move(&mut self, state: &mut Game) {
        let mut ui_state = state.town_world().write_resource::<UiState>();
        (*ui_state).hovered_entity = None;
        if let Some(mouse_pos) = self.mouse.pos() {
            let position = state.town_world().read_storage::<Position>();
            let entities = state.town_world().entities();
            for (e, pos) in (&entities, &position).join() {
                if mouse_pos.overlaps_rectangle(&pos.area) {
                    (*ui_state).hovered_entity = Some(e);
                    break;
                }
            }
        }
    }
    pub fn render_grabbed_item(
        &self,
        town: &Town,
        window: &mut DisplayArea,
        sprites: &mut Sprites,
        item: &Grabbable,
    ) {
        if self.mouse.pos().is_none() {
            return;
        }
        let mouse = self.mouse.pos().unwrap();

        let ul = TOWN_TILE_S as f32;
        let center = mouse - (ul / 2.0, ul / 2.0).into();
        let max_area = Rectangle::new(center, (ul, ul));
        match item {
            Grabbable::NewBuilding(building_type) => {
                let possible_tiles = town.allowed_tiles_for_new_building(*building_type);
                let shadow_col = Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.15,
                };
                Town::shadow_tiles(window, &possible_tiles, shadow_col);

                draw_static_image(
                    sprites,
                    window,
                    &max_area,
                    building_type.sprite().default(),
                    Z_GRABBED_ITEM,
                    FitStrategy::TopLeft,
                )
            }
            Grabbable::Ability(ability) => draw_static_image(
                sprites,
                window,
                &max_area.shrink_to_center(0.375),
                ability.sprite().default(),
                Z_GRABBED_ITEM,
                FitStrategy::TopLeft,
            ),
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

pub fn render_town_entities(world: &World, window: &mut DisplayArea, sprites: &mut Sprites) {
    let pos_store = world.read_storage::<Position>();
    let rend_store = world.read_storage::<Renderable>();
    let animation_store = world.read_storage::<AnimationState>();
    let triggers = world.read_storage::<EntityTrigger>();
    let entities = world.entities();
    let tick = world.read_resource::<ClockTick>();
    for (e, pos, r) in (&entities, &pos_store, &rend_store).join() {
        let mut area = pos.area;
        if r.in_game_transformation.is_normal() {
            area = area.shrink_to_center(r.in_game_transformation);
        }
        match r.kind {
            RenderVariant::Img(i) | RenderVariant::ImgWithImgBackground(i, _) => {
                if let Some(animation) = animation_store.get(e) {
                    draw_animated_sprite(
                        sprites,
                        window,
                        &area,
                        i,
                        pos.z,
                        FitStrategy::TopLeft,
                        animation,
                        tick.0,
                    );
                } else {
                    draw_static_image(
                        sprites,
                        window,
                        &area,
                        i.default(),
                        pos.z,
                        FitStrategy::TopLeft,
                    );
                }
            }
            RenderVariant::ImgCollection(ref c) => {
                draw_image_collection(sprites, window, &area, &c, pos.z, FitStrategy::TopLeft);
            }

            _ => panic!("Not implemented"),
        }
        if triggers.get(e).is_some() {
            draw_shiny_border(window, pos.area, tick.0);
        }
    }
}

pub fn render_hovering(
    world: &World,
    window: &mut DisplayArea,
    sprites: &mut Sprites,
    entity: Entity,
) {
    let position_store = world.read_storage::<Position>();
    let range_store = world.read_storage::<Range>();
    let health_store = world.read_storage::<Health>();

    if let Some((range, p)) = (&range_store, &position_store)
        .join()
        .get(entity, &world.entities())
    {
        range.draw(window, &p.area).nuts_check();
    }

    if let Some((health, p)) = (&health_store, &position_store)
        .join()
        .get(entity, &world.entities())
    {
        render_health(&health, sprites, window, &p.area);
    }
}

fn render_health(
    health: &Health,
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    area: &Rectangle,
) {
    let (max, hp) = (health.max_hp, health.hp);
    let unit_pos = area.pos;
    let w = area.width();
    let h = 10.0;
    let max_area = Rectangle::new((unit_pos.x, unit_pos.y - h), (w, h));

    match hp {
        0 => {
            let h = 20.0;
            let max_area = Rectangle::new((unit_pos.x, unit_pos.y - h), (w, h));
            draw_static_image(
                sprites,
                window,
                &max_area,
                SpriteIndex::Simple(SingleSprite::Heart),
                Z_HP_BAR,
                FitStrategy::Center,
            );
        }
        hp if hp < 10 => {
            let d = w / hp as f32;
            let mut hp_block = max_area.clone();
            hp_block.size.x = d * 0.9;
            for _ in 0..hp as usize {
                draw_rect(window, &hp_block, GREY);
                hp_block.pos.x += d;
            }
        }
        hp if hp < 50 => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max - hp) as f32 / max as f32;
            draw_rect(window, &max_area, GREY);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        }
        _ => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max - hp) as f32 / max as f32;
            draw_rect(window, &max_area, BLACK);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        }
    }
}
#[inline]
fn draw_rect(window: &mut DisplayArea, area: &Rectangle, col: Color) {
    draw_rect_z(window, area, col, 0);
}
#[inline]
fn draw_rect_z(window: &mut DisplayArea, area: &Rectangle, col: Color, z_shift: i16) {
    window.draw_ex(area, &col, Transform::IDENTITY, Z_HP_BAR + z_shift);
}

impl Range {
    fn draw(&self, window: &mut DisplayArea, area: &Rectangle) -> PadlResult<()> {
        // TODO Check if this aligns 100% with server. Also consider changing interface to TileIndex instead of center
        Town::shadow_rectified_circle(window, area.center(), self.range);
        Ok(())
    }
}
