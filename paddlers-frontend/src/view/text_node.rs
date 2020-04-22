use stdweb::web::{INode, Node};

#[derive(Debug)]
pub struct TextNode {
    dom_node: Node,
    text: String,
    dirty: bool,
}

impl TextNode {
    pub fn new(dom_node: Node, text: String) -> Self {
        TextNode {
            text,
            dom_node,
            dirty: true,
        }
    }
    /// Update the inner text (without redrawing it)
    /// Performs string comparison and also a string copy when necessary
    pub fn update(&mut self, text: &str) {
        if self.dirty || text != self.text {
            self.text.clear();
            self.text.push_str(text);
            self.dirty = true;
        }
    }
    /// Same as `update` but takes ownership of string and avoids copying the string content
    pub fn update_owned(&mut self, text: String) {
        if self.dirty || text != self.text {
            self.text = text;
            self.dirty = true;
        }
    }
    pub fn draw(&mut self) {
        if self.dirty {
            self.dom_node.set_text_content(&self.text);
            self.dirty = false;
        }
    }
    pub fn delete(&self) -> Result<(), &'static str> {
        if let Some(parent) = self.dom_node.parent_node() {
            return parent
                .remove_child(&self.dom_node)
                .map(|_| ())
                .map_err(|_| "Child vanished");
        }
        Ok(())
    }
}
