use crate::prelude::*;
use crate::view::TextNode;
use panes::PaneHandle;
use quicksilver::geom::Rectangle;

#[derive(Debug)]
pub struct FloatingText {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    node: TextNode,
    pane: PaneHandle,
}

impl FloatingText {
    pub fn new(area: &Rectangle, text: String) -> PadlResult<Self> {
        Self::new_styled(area, text,&[])
    }
    pub fn new_styled(area: &Rectangle, text: String, styles: &[(&str,&str)]) -> PadlResult<Self> {
        let x = area.x() as u32;
        let y = area.y() as u32;
        let w = area.width() as u32;
        let h = area.height() as u32;

        let html = &text;
        let classes: [&str;0] = [];
        let pane = panes::new_styled_pane(x,y,w,h,html,&classes,styles)?;

        let text_node = pane.parent_element()?.into();
        let node = TextNode::new(text_node, text);

        let float = 
        FloatingText {
            x,y,w,h,
            node,
            pane,
        };
        Ok(float)
    }
    pub fn new_triplet() -> PadlResult<[Self;3]> {
        Ok([
            FloatingText::try_default()?,
            FloatingText::try_default()?,
            FloatingText::try_default()?,
        ])
    }
    pub fn update_position(&mut self, area: &Rectangle) -> Result<(), panes::PanesError> {
        let (x,y,w,h) = (
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
        );

        self.pane.reposition_and_resize(x,y,w,h)
    }
    pub fn update_text(&mut self, text: &str) {
        self.node.update(text);
    }
    pub fn draw(&mut self) {
        self.node.draw();
    }
    pub fn hide(&self) -> Result<(), panes::PanesError> {
        self.pane.hide()
    }
    pub fn try_default() -> PadlResult<Self> {
        Self::new(&Rectangle::default(), "".to_owned())
    }
}
impl Drop for FloatingText {
    fn drop(&mut self) {
        let result = self.node.delete();
        if let Err(e) = result {
            println!("Error while deleting a FloatingText: {}", e);
        }
        let result = self.pane.delete();
        if let Err(e) = result {
            println!("Error while deleting a FloatingText: {}", e);
        }
    }
}