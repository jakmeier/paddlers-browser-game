#[allow(unused_macros)]
#[cfg(target_arch = "wasm32")]
macro_rules! println {
    ($($tt:tt)*) => {{
        let msg = format!($($tt)*);
        js! { console.log(@{ msg }) }
    }}
}
#[allow(unused_macros)]
#[cfg(target_arch = "wasm32")]
macro_rules! error {
    ($($tt:tt)*) => {{
        let msg = format!($($tt)*);
        js! { console.error(@{ msg }) }
    }}
}

#[cfg(target_arch = "wasm32")]
pub fn setup_wasm() {
    std::panic::set_hook(Box::new(|panic_info| {
        error!("PANIC: {}\n", &panic_info);
    }));
    stdweb::initialize();
    // stdweb::event_loop();
}

use crate::stdweb::unstable::TryInto;
pub fn utc_now() -> crate::Timestamp {
    let millis: f64 = js!(
        var date = new Date();
        return date.getTime();
    )
    .try_into()
    .expect("Reading time");
    crate::Timestamp::from_millis(millis as i64)
}

/// Extension trait for stdweb::web::INode
pub trait PadlINode {
    fn remove_all_children(&self);
}

use stdweb::web::INode;
impl<T: INode> PadlINode for T {
    fn remove_all_children(&self) {
        while let Some(child) = self.first_child() {
            self.remove_child(&child).expect("not found");
        }
    }
}
