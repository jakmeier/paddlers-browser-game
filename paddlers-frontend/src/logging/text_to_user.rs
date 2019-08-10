use crate::prelude::*;
use quicksilver::prelude::*;
use crate::gui::{
    utils::{write_text_col, FitStrategy, GREY},
    z::*,
};

const ERROR_COLOR: Color =    Color { r: 0.8, g: 0.2, b: 0.2, a: 0.75 };

struct TextMessage {
    text: String,
    show_until: Timestamp,
    color: Color,
}

#[derive(Default)]
pub struct TextBoard {
    messages: Vec<TextMessage>,
}

impl TextBoard {
    pub fn display_error_message(&mut self, msg: String) {
        self.display_message(msg, ERROR_COLOR, 3_000_000);
    }
    pub fn display_debug_message(&mut self, msg: String) {
        self.display_message(msg, GREY, 8_000_000);
    }
    fn display_message(&mut self, msg: String, col: Color, time_us: i64) {
        let t = utc_now() + time_us;
        self.messages.push(
            TextMessage {
                text: msg,
                show_until: t,
                color: col,
            }
        );
    }
    pub fn draw(&mut self, asset: &mut Asset<Font>, window: &mut Window, max_area: &Rectangle) {
        let now = utc_now();
        let mut area = max_area.clone(); 
        self.messages.retain(|msg| {
            if msg.show_until < now {
                return false;
            }
            // TODO: multi line text
            let (line, rest) = area.cut_horizontal(50.0);
            area = rest;
            window.draw_ex(&line, Col(msg.color), Transform::IDENTITY, Z_TEXT_MESSAGE-1);
            write_text_col(asset, window, &line.padded(10.0), Z_TEXT_MESSAGE, FitStrategy::Center, &msg.text, Color::WHITE).unwrap();
            true
        });
    }
}