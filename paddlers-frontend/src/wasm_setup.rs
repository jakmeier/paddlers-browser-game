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
    let millis : f64 = js!(
        var date = new Date();
        return date.getTime();
    ).try_into().unwrap();
    millis as i64 * 1000
}