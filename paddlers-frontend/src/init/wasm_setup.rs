#[allow(unused_macros)]
macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[allow(unused_macros)]
macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

pub fn setup_wasm() {
    #[cfg(feature = "console_error_panic_hook")]
    #[cfg(feature = "debug_assertions")]
    std::panic::set_hook(Box::new(|panic_info| {
        let nuts_info = nuts::panic_info();
        web_sys::console::error_1(&nuts_info.into());
        console_error_panic_hook::hook(panic_info);
    }));
}

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
