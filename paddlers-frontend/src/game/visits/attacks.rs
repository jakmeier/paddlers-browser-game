//! For incoming visits (attacks)

use crate::net::state::current_village;
use crate::prelude::*;
use crate::{gui::utils::colors::LIGHT_BLUE, net::game_master_api::RestApiState};
use crate::{
    gui::z::*,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use chrono::NaiveDateTime;
use mogwai::prelude::*;
use paddle::NutsCheck;
use paddle::*;
use paddlers_shared_lib::api::attacks::*;
use specs::prelude::Component;
use specs::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Attack {
    pub arrival: NaiveDateTime,
    size: u32,
    description: String,
    dom_node: Option<TextNode>,
    view: Option<Vec<View<HtmlElement>>>,
}

impl Game {
    pub fn send_prophet_attack(&mut self, target: (i32, i32)) -> PadlResult<()> {
        let maybe_prophet = self.town_mut().idle_prophets.pop();
        if let Some(prophet) = maybe_prophet {
            let hobo = self.hobo_key(prophet)?;
            let atk = AttackDescriptor {
                from: current_village(),
                to: target,
                units: vec![hobo],
            };
            nuts::send_to::<RestApiState, _>(atk);
            Ok(())
        } else {
            PadlErrorCode::NotEnoughUnits.usr()
        }
    }
}

impl Attack {
    pub fn new(arrival: NaiveDateTime, description: String, size: u32) -> Self {
        Attack {
            arrival,
            dom_node: None,
            description,
            size,
            view: None,
        }
    }
    fn arrival(&self) -> String {
        let t = (self.arrival - utc_now()).num_seconds();
        if t > 0 {
            t.to_string() + "s"
        } else {
            "Arrived".to_owned()
        }
    }
    fn update_dom(&mut self) -> PadlResult<()> {
        if self.dom_node.is_some() {
            let text = self.arrival();
            self.dom_node.as_mut().unwrap().update_owned(text);
            self.dom_node.as_mut().unwrap().draw();
            return Ok(());
        }
        PadlErrorCode::InvalidDom("Not initialized").dev()
    }
    // TODO: remove after 60s
    fn delete(mut self) {
        if let Some(node) = self.dom_node {
            node.delete().unwrap();
        }
        self.view.take();
    }
}

pub(crate) struct VisitorFrame<'a, 'b> {
    incoming_attacks_table: HtmlElement,
    update_dispatcher: Dispatcher<'a, 'b>,
    pane: div::DivHandle,
}

impl<'a, 'b> VisitorFrame<'a, 'b> {
    pub fn new(x: f32, y: f32) -> PadlResult<Self> {
        let pane = div::new(
            x as i32,
            y as i32,
            Self::WIDTH / 2,
            Self::HEIGHT,
            r#"<div class="attack-table"><h2>Incoming Visitors</h2><div>ORIGIN</div><div>PADDLERS</div><div>ARRIVAL</div></div>"#,
        )
        .expect("Pane not set up properly");
        let table = pane
            .first_inner_node()
            .map_err(|_| PadlError::dev_err(PadlErrorCode::InvalidDom("No table in pane")))?;

        let update_dispatcher = DispatcherBuilder::new()
            .with(UpdateAttackViewSystem::new(), "update_atk", &[])
            .build();

        let attack = VisitorFrame {
            incoming_attacks_table: table.dyn_into().unwrap(),
            update_dispatcher,
            pane,
        };
        attack.pane.hide()?;
        Ok(attack)
    }
    #[allow(unused_braces)]
    pub fn add_row(&mut self, a: &Attack) -> Vec<View<HtmlElement>> {
        let view = view! (
            <div>{&a.description}</div>
            <div>{a.size.to_string()}</div>
        );

        for node in &view {
            self.incoming_attacks_table
                .append_child(&node.dom_ref())
                .unwrap();
        }
        view
    }
}

impl<'a, 'b> Frame for VisitorFrame<'a, 'b> {
    type State = Game;
    const WIDTH: u32 = crate::resolution::MAIN_AREA_W;
    const HEIGHT: u32 = crate::resolution::MAIN_AREA_H;

    fn update(&mut self, state: &mut Self::State) {
        self.update_dispatcher.dispatch(&mut state.world);
        let mut attack = state.world.write_storage::<Attack>();
        for a in (&mut attack).join() {
            if a.dom_node.is_none() {
                let view = self.add_row(&a);
                let arrival_node = div::doc().unwrap().create_element("div").unwrap();
                self.incoming_attacks_table
                    .append_child(&arrival_node)
                    .unwrap();
                let text_node = TextNode::new(arrival_node.dyn_into().unwrap(), a.arrival());
                a.dom_node = Some(text_node);
                a.view = Some(view);
            }
        }
    }
    fn draw(&mut self, _state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        let main_area = Rectangle::new_sized((MAIN_AREA_W, MAIN_AREA_H));
        window.draw_ex(&main_area, &LIGHT_BLUE, Transform::IDENTITY, Z_TEXTURE);
    }
    fn enter(&mut self, _state: &mut Self::State) {
        self.pane.show().nuts_check();
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.pane.hide().nuts_check();
    }
}

pub struct UpdateAttackViewSystem {
    last_update: NaiveDateTime,
}
impl UpdateAttackViewSystem {
    pub fn new() -> Self {
        UpdateAttackViewSystem {
            last_update: utc_now(),
        }
    }
}

impl<'a> System<'a> for UpdateAttackViewSystem {
    type SystemData = (WriteStorage<'a, Attack>,);

    fn run(&mut self, (mut attack,): Self::SystemData) {
        let now = utc_now();
        if (now - self.last_update).num_microseconds().unwrap() < 1_000_000 {
            return;
        }
        self.last_update = now;
        for a in (&mut attack).join() {
            if a.dom_node.is_some() {
                a.update_dom().unwrap_or_else(nuts::publish);
            }
        }
    }
}
