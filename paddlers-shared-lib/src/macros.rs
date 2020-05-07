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
#[macro_export]
/// for printing that should only happen on the web
macro_rules! webprintln {
    ($($tt:tt)*) => {{
        let msg = format!($($tt)*);
        js! { console.log(@{ msg }) }
    }}
}

#[allow(unused_macros)]
#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
/// for printing that should only happen on the web
macro_rules! webprintln {
    ($($tt:tt)*) => {{
    }}
}