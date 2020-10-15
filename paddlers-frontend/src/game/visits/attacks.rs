//! For incoming visits (attacks)

use crate::gui::utils::colors::LIGHT_BLUE;
use crate::gui::z::*;
use crate::net::game_master_api::RestApiState;
use crate::net::state::current_village;
use crate::prelude::*;
use chrono::NaiveDateTime;
use paddle::{utc_now, Frame, TextNode};
use paddlers_shared_lib::api::attacks::*;
use div::new_pane;
use quicksilver::prelude::{Col, Rectangle, Transform, Window};
use specs::prelude::*;
use web_sys::{HtmlElement, Node};
use wasm_bindgen::JsCast;

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Attack {
    pub arrival: NaiveDateTime,
    size: u32,
    description: String,
    dom_node: Option<TextNode>,
}

impl Game<'_, '_> {
    pub fn send_prophet_attack(&mut self, target: (i32, i32)) -> PadlResult<()> {
        let maybe_prophet = self.town_mut().idle_prophets.pop();
        if let Some(prophet) = maybe_prophet {
            let hobo = self.hobo_key(prophet)?;
            let atk = AttackDescriptor {
                from: current_village(),
                to: target,
                units: vec![hobo],
            };
            RestApiState::get().http_send_attack(atk)?;
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
    fn to_html(&self) -> String {
        format!(
            "<div>{}</div><div>{}</div><div>{}</div>",
            self.description,
            self.size,
            self.arrival()
        )
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
    fn delete(self) {
        if let Some(node) = self.dom_node {
            node.delete().unwrap();
        }
    }
}

pub(crate) struct VisitorFrame<'a, 'b> {
    incoming_attacks_table: HtmlElement,
    update_dispatcher: Dispatcher<'a, 'b>,
    pane: div::PaneHandle,
}

impl<'a, 'b> VisitorFrame<'a, 'b> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> PadlResult<Self> {
        let pane = new_pane(
            x as u32,
            y as u32,
            (w / 2.0) as u32,
            h as u32,
            r#"<div class="attack-table"></div>"#,
        )
        .expect("Pane not set up properly");
        let table = pane
            .first_inner_node()
            .map_err(|_| PadlError::dev_err(PadlErrorCode::InvalidDom("No table in pane")))?;

        let update_dispatcher = DispatcherBuilder::new()
            .with(UpdateAttackViewSystem::new(), "update_atk", &[])
            .build();

        let mut attack = VisitorFrame {
            incoming_attacks_table: table.dyn_into().unwrap(),
            update_dispatcher,
            pane,
        };
        attack.add_row("<h2>Incoming Visitors</h2>")?;
        attack.pane.hide()?;

        Ok(attack)
    }
    pub fn add_row(&mut self, html: &str) -> PadlResult<Node> {
        self.incoming_attacks_table
            .append_with_str_1(&html)
            .map_err(|_e| PadlError::dev_err(PadlErrorCode::InvalidDom("Inserting HTML failed")))?;
        self.incoming_attacks_table
            .last_child()
            .ok_or(PadlError::dev_err(PadlErrorCode::InvalidDom(
                "Child lookup failed",
            )))
    }
}

impl<'a, 'b> Frame for VisitorFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;

    fn update(&mut self, state: &mut Self::State) -> Result<(), Self::Error> {
        self.update_dispatcher.dispatch(&mut state.world);
        let mut attack = state.world.write_storage::<Attack>();
        for a in (&mut attack).join() {
            if a.dom_node.is_none() {
                let html = a.to_html();
                match self.add_row(&html) {
                    Ok(node) => {
                        if let Some(arrival_node) = node.last_child() {
                            let text_node = TextNode::new(arrival_node.dyn_into().unwrap(), a.arrival());
                            a.dom_node = Some(text_node);
                        } else {
                            nuts::publish(PadlError::dev_err(PadlErrorCode::InvalidDom(
                                "Child lookup failed",
                            )));
                        }
                    }
                    Err(e) => nuts::publish(e),
                }
            }
        }
        Ok(())
    }
    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        let ui_state = state.world.read_resource::<ViewState>();
        let main_area = Rectangle::new(
            (0, 0),
            (
                ui_state.menu_box_area.x(),
                (window.project() * window.screen_size()).y,
            ),
        );
        std::mem::drop(ui_state);
        window.draw_ex(&main_area, Col(LIGHT_BLUE), Transform::IDENTITY, Z_TEXTURE);
        Ok(())
    }
    fn enter(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.pane.show()?;
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.pane.hide()?;
        Ok(())
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
