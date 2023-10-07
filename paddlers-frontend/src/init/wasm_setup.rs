/// Extension trait for stdweb::web::INode
pub trait PadlINode {
    fn remove_all_children(&self);
}

impl PadlINode for web_sys::Node {
    fn remove_all_children(&self) {
        while let Some(child) = self.first_child() {
            self.remove_child(&child).expect("not found");
        }
    }
}
