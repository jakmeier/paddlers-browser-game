mod attackers;
mod defenders;
mod input;
mod movement;
mod render;
mod sprites;
mod town;

use crate::game::input::Clickable;
use input::{Click, ClickSystem, MenuBoxData};
use movement::*;
use quicksilver::prelude::*;
use render::*;
use specs::prelude::*;
use sprites::*;
use town::{Town, TOWN_RATIO};
use attackers::{delete_all_attackers, Attacker};
use std::sync::mpsc::{Receiver};
use crate::net::NetMsg;

const MENU_BOX_WIDTH: f32 = 300.0;
const CYCLE_SECS: u32 = 10;

mod resources {
    pub struct ClockTick(pub u32);
    // pub struct UnitLength(pub f32);
    #[derive(Default)]
    pub struct Dt(pub f64);
}
use resources::*;

struct Game<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    click_dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
    town: Town,
    pub sprites: Asset<Sprites>,
    unit_len: Option<f32>,
    net: Option<Receiver<NetMsg>>,
    cycle_begin: f64,
}

impl Game<'static, 'static> {
    fn with_unit_length(mut self, ul: f32) -> Self {
        self.unit_len = Some(ul);
        self
    }
    fn with_menu_box_area(self, area: Rectangle) -> Self {
        {
            let mut data = self.world.write_resource::<MenuBoxData>();
            (*data).area = area;
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
        world.insert(MenuBoxData::default());
        world.insert(Dt);
        world.insert(Click::default());

        let mut dispatcher = DispatcherBuilder::new()
            .with(MoveSystem, "update_atk_pos", &[])
            .build();
        dispatcher.setup(&mut world);

        let mut click_dispatcher = DispatcherBuilder::new()
            .with(ClickSystem, "click", &[])
            .build();
        click_dispatcher.setup(&mut world);

        let town = Town::new();

        // defenders::insert_flowers(&mut world, (100,100), 50.0);

        Ok(Game {
            dispatcher: dispatcher,
            click_dispatcher: click_dispatcher,
            world: world,
            town: town,
            sprites: Sprites::new(),
            unit_len: None,
            net: None,
            cycle_begin: 0.0,
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        {
            let mut tick = self.world.write_resource::<ClockTick>();
            *tick = ClockTick(tick.0 + 1);
        }
        {
            use std::sync::mpsc::TryRecvError;
            match self.net.as_ref().unwrap().try_recv() {
                Ok(msg) => {
                    // println!("Received Network data!");
                    match msg {
                        NetMsg::Attacks(response) => {
                            self.new_cycle();
                            delete_all_attackers(&mut self.world);
                            if let Some(data) = response.data {
                                for atk in data.attacks {
                                    atk.create_entities(&mut self.world, self.unit_len.unwrap());
                                }
                            }
                            else {
                                println!("No data returned");
                            }
                        }
                        NetMsg::Buildings(response) => {
                            if let Some(data) = response.data {
                                data.create_entities(&mut self.world, self.unit_len.unwrap());
                            }
                            else {
                                println!("No buildings available");
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
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let tick = self.world.read_resource::<ClockTick>().0;
        // double borrow
        let (asset, town, ul) = (&mut self.sprites, &self.town, self.unit_len.unwrap());
        asset.execute(|sprites| town.render(window, sprites, tick, ul))?;
        self.render_entities(window)?;
        self.render_menu_box(window)?;
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        // println!("Event: {:?}", event);
        match event {
            Event::MouseMoved(_position) => {
                // TODO: Hoverable stuff
            }
            Event::MouseButton(button, state)
                if *button == MouseButton::Left && *state == ButtonState::Pressed =>
            {
                {
                    let mut c = self.world.write_resource::<Click>();
                    *c = Click(window.mouse().pos());
                }
                self.click_dispatcher.dispatch(&mut self.world);
            }
            _ => {}
        };

        Ok(())
    }
}

impl Game<'_,'_> {
    fn new_cycle(&mut self) {
        self.cycle_begin = stdweb::web::Date::now();
    }
    fn update_dt(&mut self) {
        if self.cycle_begin != 0.0 {
            let t = stdweb::web::Date::now();
            let mut dt = self.world.write_resource::<Dt>();
            *dt = Dt(t - self.cycle_begin);
        }
    }
}

fn init_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();
    world.register::<Clickable>();
    world.register::<Attacker>();

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
