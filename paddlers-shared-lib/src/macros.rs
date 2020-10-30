#[allow(unused_macros)]
#[cfg(target_arch = "wasm32")]
macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[allow(unused_macros)]
#[cfg(target_arch = "wasm32")]
#[macro_export]
/// for printing that should only happen on the web
macro_rules! webprintln {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[allow(unused_macros)]
#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
/// for printing that should only happen on the web
macro_rules! webprintln {
    ($($tt:tt)*) => {{}};
}
