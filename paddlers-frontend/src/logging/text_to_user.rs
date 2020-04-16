use crate::prelude::*;
use crate::view::FloatingText;
use quicksilver::prelude::*;
use crate::gui::{
    utils::colors::color_string,
    utils::{GREY, BLUE},
};

const ERROR_COLOR: Color = Color { r: 0.8, g: 0.2, b: 0.2, a: 0.75 };

struct TextMessage {
    float: FloatingText,
    show_until: Timestamp,
}

#[derive(Default)]
pub struct TextBoard {
    messages: Vec<TextMessage>,
}

impl TextBoard {
    pub fn display_error_message(&mut self, msg: String) -> PadlResult<()> {
        self.display_message(msg, ERROR_COLOR, 3_000_000)
    }
    #[allow(dead_code)]
    pub fn display_debug_message(&mut self, msg: String) -> PadlResult<()> {
        self.display_message(msg, GREY, 8_000_000)
    }
    pub fn display_confirmation(&mut self, msg: String) -> PadlResult<()> {
        self.display_message(msg, BLUE, 3_000_000)
    }
    fn display_message(&mut self, msg: String, col: Color, time_us: i64) -> PadlResult<()> {
        let show_until = utc_now() + time_us;
        let float = Self::new_float(msg, col)?;
        self.messages.push(
            TextMessage { float, show_until }
        );
        Ok(())
    }
    pub fn draw(&mut self, max_area: &Rectangle) -> PadlResult<()> {
        self.remove_old_messages();
        let mut area = max_area.clone(); 
        for msg in self.messages.iter_mut() {
            let (line, rest) = area.cut_horizontal(60.0);
            let (_padding, rest) = rest.cut_horizontal(15.0);
            area = rest;
            msg.float.update_position(&line)?;
            msg.float.draw();
        }
        Ok(())
    }
    fn remove_old_messages(&mut self) {
        let now = utc_now();
        self.messages.retain(|msg| msg.show_until > now );
    }
    fn new_float(s: String, col: Color) -> PadlResult<FloatingText> {
        let col_str = color_string(&col);
        FloatingText::new_styled(
            &Rectangle::default(),
            s,
            &[
                ("background-color",&col_str),
                ("color","white"),
                ("padding","5px"),
                ("text-align","center"),
            ],
            &[],
        )
    }
}