pub (crate) mod init;
pub (crate) mod specs_resources;
pub (crate) mod buildings;
pub (crate) mod units;
pub (crate) mod movement;
pub (crate) mod map;
pub (crate) mod town;
pub (crate) mod town_resources;
pub (crate) mod fight;
pub (crate) mod components;
pub (crate) mod forestry;
pub (crate) mod abilities;
pub (crate) mod net_receiver;
pub (crate) mod status_effects;

use crate::prelude::*;
use crate::game::{
    components::*,
    };
use crate::gui::{
    input::{self, UiView},
    sprites::*,
};
use crate::net::{
    NetMsg, 
    game_master_api::RestApiState,
};
use crate::logging::{
    ErrorQueue,
    text_to_user::TextBoard,
    statistics::Statistician,
};
pub use specs_resources::*;

use input::{UiState, pointer::PointerManager};
use movement::*;
use quicksilver::prelude::*;
use specs::prelude::*;
use town::{Town, town_shop::DefaultShop};
use fight::*;
use forestry::*;
use std::sync::mpsc::{Receiver, channel};
use town_resources::TownResources;
use units::worker_system::WorkerSystem;
use map::{GlobalMap, GlobalMapPrivateState};


pub(crate) struct Game<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    pointer_manager: PointerManager<'a, 'b>,
    pub world: World,
    pub sprites: Option<Sprites>,
    pub preload: Option<loading::Preloading>,
    pub font: Asset<Font>,
    pub bold_font: Asset<Font>,
    pub unit_len: Option<f32>,
    pub resources: TownResources,
    net: Option<Receiver<NetMsg>>,
    time_zero: Timestamp,
    total_updates: u64,
    async_err_receiver: Receiver<PadlError>,
    stats: Statistician,
    map: Option<GlobalMapPrivateState>,
    #[cfg(feature="dev_view")]
    palette: bool,
}

impl Game<'static, 'static> {
    fn with_town(mut self, town: Town) -> Self {
        self.world.insert(town);
        self
    }
    fn with_unit_length(mut self, ul: f32) -> Self {
        self.unit_len = Some(ul);
        self.world.insert(UnitLength(ul));
        self
    }
    fn with_ui_division(mut self, main_area: Rectangle, menu_area: Rectangle) -> Self {
        {
            self.world.insert(DefaultShop::new());
            let mut data = self.world.write_resource::<UiState>();
            (*data).main_area = main_area;
            (*data).menu_box_area = menu_area;
        } 
        self
    }
    fn with_network_chan(mut self, net_receiver: Receiver<NetMsg>) -> Self {
        self.net = Some(net_receiver);
        self
    }
    fn init_map(mut self) -> Self {
        let main_area = self.world.read_resource::<UiState>().main_area;
        let (private, shared) = GlobalMap::new(main_area.size());
        self.map = Some(private);
        self.world.insert(shared);
        self
    }
}

impl State for Game<'static, 'static> {
    fn new() -> Result<Self> {
        // Start loading fonts asap
        let font = Asset::new(Font::load("fonts/Manjari-Regular.ttf"));
        let bold_font = Asset::new(Font::load("fonts/Manjari-Bold.ttf"));

        let (err_send, err_recv) = channel();
        let mut world = init::init_world(err_send);
        
        let mut dispatcher = DispatcherBuilder::new()
            .with(WorkerSystem, "work", &[])
            .with(MoveSystem, "move", &["work"])
            .with(FightSystem::default(), "fight", &["move"])
            .with(ForestrySystem, "forest", &[])
            .build();
        dispatcher.setup(&mut world);

        let pm = PointerManager::init(&mut world);
        let now = utc_now();

        

        Ok(Game {
            dispatcher: dispatcher,
            pointer_manager: pm,
            world: world,
            sprites: None,
            preload: Some(loading::Preloading::new()),
            font: font,
            bold_font: bold_font,
            unit_len: None,
            net: None,
            time_zero: now,
            resources: TownResources::default(),
            total_updates: 0,
            async_err_receiver: err_recv,
            stats: Statistician::new(now),
            map: None,
            #[cfg(feature="dev_view")]
            palette: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.total_updates += 1;
        window.set_draw_rate(33.3); // 33ms delay between frames  => 30 fps
        window.set_max_updates(1); // 1 update per frame is enough
        // window.set_fullscreen(true);
        self.update_time_reference();
        let now = self.world.read_resource::<Now>().0;
        {
            self.pointer_manager.run(&mut self.world, now)
        }

        {
            let mut tick = self.world.write_resource::<ClockTick>();
            let us_draw_rate = 1_000_000/ 60;
            *tick = ClockTick((now / us_draw_rate) as u32);
        }
        {
            let mut q = self.world.write_resource::<ErrorQueue>();
            let mut t = self.world.write_resource::<TextBoard>();
            q.pull_async(&mut self.async_err_receiver, &mut t);
            q.run(&mut t);
        }
        if self.sprites.is_none() {
            return self.update_preloading(window);
        }

        {
            let mut res = self.world.write_resource::<TownResources>();
            *res = self.resources;
        }
        if let Err(net_err) = self.update_net() {
            let mut q = self.world.write_resource::<ErrorQueue>();
            q.push(net_err);
        }
        {
            self.map_mut().update();
        }
        self.update_time_reference();
        self.dispatcher.dispatch(&mut self.world);
        if self.total_updates % 300 == 15 {
            self.reaper(&Rectangle::new_sized(window.screen_size()));
        }
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let tick = self.world.read_resource::<ClockTick>().0;
        let now = utc_now();
        {
            let mut rest = self.world.write_resource::<RestApiState>();
            let err = self.stats.run(&mut *rest, now);
            self.check(err);
        }
        if self.sprites.is_none() {
            return self.draw_preloading(window);
        }

        let ui_state = self.world.read_resource::<UiState>();
        let hovered_entity = ui_state.hovered_entity;
        let grabbed_item = ui_state.grabbed_item.clone();
        let view = ui_state.current_view;
        let main_area = Rectangle::new(
            (0,0), 
            (ui_state.menu_box_area.x(), window.screen_size().y)
        );
        std::mem::drop(ui_state);
        window.clear(Color::WHITE)?;
        match view {
            UiView::Town => {
                {
                    let (asset, town, ul) = (&mut self.sprites, &self.world.read_resource::<Town>(), self.unit_len.unwrap());
                    // asset.execute(|sprites| town.render(window, sprites, tick, ul))?;
                    town.render(window, asset.as_mut().unwrap(), tick, ul)?;
                }
                self.render_entities(window)?;
            },
            UiView::Map => {
                let (sprites, mut map) = (
                    &mut self.sprites, 
                    GlobalMap::combined(
                        self.map.as_mut().unwrap(),
                        self.world.write_resource()
                    )
                );
                map.render(window, &mut sprites.as_mut().unwrap(), &main_area)?;
            }
        }
        
        self.render_menu_box(window)?;
        self.render_text_messages(window)?;

        if let Some(entity) = hovered_entity {
            self.render_hovering(window, entity)?;
        }
        if let Some(grabbed) = grabbed_item {
            self.render_grabbed_item(window, &grabbed)?;
        }
        #[cfg(feature="dev_view")]
        self.draw_dev_view(window);
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        // println!("Event: {:?}", event);
        // {
        //     let mut t = self.world.write_resource::<TextBoard>();
        //     t.display_debug_message(format!("{:?}", event));
        // }
        match event {
            Event::MouseMoved(pos) => {
                self.pointer_manager.move_pointer(&mut self.world, &pos);
            },
            Event::MouseButton(button, state)
            => {
                let now = self.world.read_resource::<Now>().0;
                let pos = &window.mouse().pos();
                self.pointer_manager.button_event(now, pos, *button, *state);  
            }
            Event::Key(key, state) 
                if *key == Key::Escape && *state == ButtonState::Pressed =>
                {
                    let mut ui_state = self.world.write_resource::<UiState>();
                    if (*ui_state).grabbed_item.is_some(){
                        (*ui_state).grabbed_item = None;
                    } else {
                        (*ui_state).selected_entity = None;
                    }
                },
            Event::Key(key, state) 
                if *key == Key::Delete && *state == ButtonState::Pressed =>
                {
                    let mut ui_state = self.world.write_resource::<UiState>();
                    if let Some(e) = ui_state.selected_entity {
                        (*ui_state).selected_entity = None;
                        std::mem::drop(ui_state);

                        let pos_store = self.world.read_storage::<Position>();
                        let pos = pos_store.get(e).unwrap();
                        let tile_index = self.town().tile(pos.area.center());
                        std::mem::drop(pos_store);

                        let r = self.rest().http_delete_building(tile_index);
                        self.check(r);

                        // Account for changes in aura total
                        let aura_store = self.world.read_storage::<Aura>();
                        let aura = aura_store.get(e).map(|a| a.effect);
                        let range_store = self.world.read_storage::<Range>();
                        let range = range_store.get(e).map(|r| r.range);
                        std::mem::drop(aura_store);
                        std::mem::drop(range_store);
                        if let Some(aura) = aura {
                            if let Some(range) = range {
                                if range > self.town().distance_to_lane(tile_index) {
                                    self.town_mut().total_ambience -= aura;
                                }
                            }
                        }

                        self.town_mut().remove_building(tile_index);
                        self.world.delete_entity(e)
                            .unwrap_or_else(
                                |_|
                                self.check(
                                    PadlErrorCode::DevMsg("Tried to delete wrong Generation").dev()
                                ).unwrap()
                            );
                    }
                },
            Event::Key(key, state) 
                if *key == Key::Tab && *state == ButtonState::Pressed =>
                {
                    let mut ui_state = self.world.write_resource::<UiState>();
                    ui_state.toggle_view();
                },
            _evt => {
                // println!("Event: {:#?}", _evt)
            }
        };
        #[cfg(feature="dev_view")]
        self.dev_view_event(event);
        self.world.maintain();
        Ok(())
    }
}

impl Game<'_,'_> {
    pub fn town(&self) -> specs::shred::Fetch<Town> {
        self.world.read_resource()
    }
    pub fn town_mut(&mut self) -> specs::shred::FetchMut<Town> {
        self.world.write_resource()
    }
    pub fn map_mut(&mut self) -> GlobalMap {
        GlobalMap::combined(
            self.map.as_mut().unwrap(),
            self.world.write_resource(),
        )
    }
    pub fn rest(&mut self) -> specs::shred::FetchMut<RestApiState> {
        self.world.write_resource()
    }
    fn update_time_reference(&mut self) {
        if self.time_zero != 0 {
            let t = utc_now();
            let mut ts = self.world.write_resource::<Now>();
            *ts = Now(t);
        }
    }
    /// Removes entities outside the map
    fn reaper(&mut self, map: &Rectangle) {
        let p = self.world.read_storage::<Position>();
        let mut dead = vec![];
        for (entity, position) in (&self.world.entities(), &p).join() {
            if !position.area.overlaps_rectangle(map)  {
                dead.push(entity);
            }
        }
        std::mem::drop(p);
        self.world.delete_entities(&dead).expect("Something bad happened when deleting dead entities");
    }
    fn worker_entity_by_net_id(&self, net_id: i64) -> PadlResult<Entity> {
        // TODO: Efficient NetId lookup
        let net = self.world.read_storage::<NetObj>();
        let ent = self.world.entities();
        NetObj::lookup_worker(net_id, &net, &ent)
    }
    fn check<R>(&self, res: PadlResult<R>) -> Option<R> {
        if let Err(e) = res {
            let mut q = self.world.write_resource::<ErrorQueue>();
            q.push(e);
            None
        }
        else {
            Some(res.unwrap())
        }
    }
    #[cfg(feature="dev_view")]
    fn draw_dev_view(&self, window: &mut Window) {
        if self.palette {
            let area = Rectangle::new((0,0),window.screen_size()).padded(100.0);
            crate::gui::utils::colors::palette::draw_color_palette(window, area);
        }
    }
    #[cfg(feature="dev_view")]
    fn dev_view_event(&mut self, event: &Event) {
        match event {
            Event::Key(key, state) 
            if *key == Key::Space && *state == ButtonState::Pressed => {
                self.palette = !self.palette;
            },
            _ => {},
        }
    }
}
