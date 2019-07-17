pub (crate) mod attackers;
pub (crate) mod defenders;
pub (crate) mod movement;
pub (crate) mod town;
pub (crate) mod town_resources;
pub (crate) mod fight;

use crate::gui::input;
use crate::gui::render::*;
use crate::gui::sprites::*;
use crate::net::{NetMsg};

use input::{MouseState, MouseSystem, UiState, Clickable, DefaultShop, Grabbable};
use movement::*;
use quicksilver::prelude::*;
use specs::prelude::*;
use town::{Town, TOWN_RATIO};
use attackers::{Attacker};
use fight::*;
use std::sync::mpsc::{Receiver};
use town_resources::TownResources;

const MENU_BOX_WIDTH: f32 = 300.0;
const CYCLE_SECS: u32 = 10;

mod resources {
    #[derive(Default)]
    pub struct ClockTick(pub u32);
    #[derive(Default)]
    pub struct UnitLength(pub f32);
    #[derive(Default)]
    pub struct Dt(pub f64);
}
use resources::*;

pub(crate) struct Game<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    mouse_dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
    pub town: Town,
    pub sprites: Asset<Sprites>,
    pub font: Asset<Font>,
    pub bold_font: Asset<Font>,
    pub unit_len: Option<f32>,
    pub resources: TownResources,
    net: Option<Receiver<NetMsg>>,
    time_zero: f64,
    total_updates: u64,
}

impl Game<'static, 'static> {
    fn with_unit_length(mut self, ul: f32) -> Self {
        self.unit_len = Some(ul);
        self.world.insert(UnitLength(ul));
        self.town.update_ul(ul);
        self
    }
    fn with_menu_box_area(mut self, area: Rectangle) -> Self {
        {
            self.world.insert(DefaultShop::new(area));
            let mut data = self.world.write_resource::<UiState>();
            (*data).menu_box_area = area;
        } 
        self
    }
    fn with_network_chan(mut self, net_receiver: Receiver<NetMsg>) -> Self {
        self.net = Some(net_receiver);
        self
    }
}

impl State for Game<'static, 'static> {
    fn new() -> Result<Self> {
        let mut world = init_world();
        world.insert(ClockTick(0));
        world.insert(UiState::default());
        world.insert(Dt);
        world.insert(MouseState::default());
        world.insert(TownResources::default());

        let mut dispatcher = DispatcherBuilder::new()
            .with(MoveSystem, "m", &[])
            .with(FightSystem::default(), "f", &["m"])
            .build();
        dispatcher.setup(&mut world);

        let mut mouse_dispatcher = DispatcherBuilder::new()
            .with(MouseSystem, "click", &[])
            .build();
        mouse_dispatcher.setup(&mut world);

        let town = Town::new(1.0);

        Ok(Game {
            dispatcher: dispatcher,
            mouse_dispatcher: mouse_dispatcher,
            world: world,
            town: town,
            sprites: Sprites::new(),
            font: Asset::new(Font::load("fonts/Manjari-Regular.ttf")),
            bold_font: Asset::new(Font::load("fonts/Manjari-Bold.ttf")),
            unit_len: None,
            net: None,
            time_zero: crate::wasm_setup::local_now(),
            resources: TownResources::default(),
            total_updates: 0,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.total_updates += 1;
        window.set_draw_rate(33.3); // 33ms delay between frames  => 30 fps
        window.set_max_updates(1); // 1 update per frame is enough
        {
            let mut tick = self.world.write_resource::<ClockTick>();
            *tick = ClockTick(tick.0 + 1);
        }
        {
            let mut res = self.world.write_resource::<TownResources>();
            *res = self.resources;
        }
        {
            use std::sync::mpsc::TryRecvError;
            match self.net.as_ref().unwrap().try_recv() {
                Ok(msg) => {
                    // println!("Received Network data!");
                    match msg {
                        NetMsg::Attacks(response) => {
                            if let Some(data) = response.data {
                                for atk in data.attacks {
                                    atk.create_entities(&mut self.world, self.unit_len.unwrap(), self.time_zero);
                                }
                            }
                            else {
                                println!("No data returned");
                            }
                        }
                        NetMsg::Buildings(response) => {
                            if let Some(data) = response.data {
                                data.create_entities(self);
                            }
                            else {
                                println!("No buildings available");
                            }
                        }
                        NetMsg::Resources(response) => {
                            if let Some(data) = response.data {
                                self.resources.update(data);
                            }
                            else {
                                println!("No resources available");
                            }
                        }
                    }
                },
                Err(TryRecvError::Disconnected) => { println!("Network connection is dead.")},
                Err(TryRecvError::Empty) => {},
            }
        }
        self.update_dt();
        self.dispatcher.dispatch(&mut self.world);
        if self.total_updates % 300 == 15 {
            self.reaper(&Rectangle::new_sized(window.screen_size()));
        }
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let tick = self.world.read_resource::<ClockTick>().0;
        // multi borrow
        let (asset, town, ul) = (&mut self.sprites, &self.town, self.unit_len.unwrap());
        asset.execute(|sprites| town.render(window, sprites, tick, ul))?;
        self.render_entities(window)?;
        self.render_menu_box(window)?;
        
        let ui_state = self.world.read_resource::<UiState>();
        let hovered_entity = ui_state.hovered_entity;
        let grabbed_item = ui_state.grabbed_item.clone();
        std::mem::drop(ui_state);
        if let Some(entity) = hovered_entity {
            self.render_hovering(window, entity)?;
        }
        if let Some(grabbed) = grabbed_item {
            self.render_grabbed_item(window, &grabbed)?;
        }
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        // println!("Event: {:?}", event);
        match event {
            Event::MouseMoved(_position) => {
                {
                    let mut c = self.world.write_resource::<MouseState>();
                    *c = MouseState(window.mouse().pos(), false);
                }
                self.mouse_dispatcher.dispatch(&mut self.world);
            }
            // Left click
            Event::MouseButton(button, state)
                if *button == MouseButton::Left && *state == ButtonState::Pressed =>
            {
                {
                    let mut c = self.world.write_resource::<MouseState>();
                    *c = MouseState(window.mouse().pos(), true);
                }
                {
                    let ui_state = self.world.read_resource::<UiState>();
                    let maybe_grabbed = ui_state.grabbed_item.clone();
                    std::mem::drop(ui_state);
                    if let Some(Grabbable::NewBuilding(item)) = maybe_grabbed {
                        if let Some(pos) = self.town.get_empty_tile(window.mouse().pos(), self.unit_len.unwrap()) {
                            self.purchase_building(item, pos);
                            let mut ui_state = self.world.write_resource::<UiState>();
                            (*ui_state).grabbed_item = None;
                        }
                    }
                }
                self.mouse_dispatcher.dispatch(&mut self.world);
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
                    let ui_state = self.world.read_resource::<UiState>();
                    if let Some(id) = ui_state.selected_entity {
                        let e = self.world.entities().entity(id);
                        std::mem::drop(ui_state);
                        let mut ui_state = self.world.write_resource::<UiState>();
                        (*ui_state).selected_entity = None;
                        std::mem::drop(ui_state);
                        self.delete_building(e);
                    }
                },
            _evt => {
                // println!("Event: {:#?}", _evt)
            }
        };

        Ok(())
    }
}

impl Game<'_,'_> {
    fn update_dt(&mut self) {
        if self.time_zero != 0.0 {
            let t = crate::wasm_setup::local_now();
            let mut dt = self.world.write_resource::<Dt>();
            *dt = Dt(t - self.time_zero);
        }
    }
    /// Removes entites outside the map
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
}

fn init_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();
    world.register::<Clickable>();
    world.register::<Attacker>();
    world.register::<Range>();
    world.register::<Health>();

    world
}

pub fn run(width: f32, height: f32, net_chan: Receiver<NetMsg>) {
    let max_town_width = width - MENU_BOX_WIDTH;
    let (tw, th) = if max_town_width / height <= TOWN_RATIO {
        (max_town_width, max_town_width / TOWN_RATIO)
    } else {
        (TOWN_RATIO * height, height)
    };

    let ul = tw / town::X as f32;
    let menu_box_area = Rectangle::new((tw,0),(MENU_BOX_WIDTH, th));
    quicksilver::lifecycle::run_with::<Game, _>(
        "Happy Town", 
        Vector::new(tw + MENU_BOX_WIDTH, th), 
        Settings::default(), 
        || Ok(
            Game::new().expect("Game initialization")
                .with_unit_length(ul)
                .with_menu_box_area(menu_box_area)
                .with_network_chan(net_chan)
            )
    );
}
