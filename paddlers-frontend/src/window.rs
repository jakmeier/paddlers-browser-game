use crate::prelude::*;
use paddle::{JsError};
use strum::IntoEnumIterator;

/// Determines a resolution which is close to the current window size
pub fn estimate_screen_size() -> PadlResult<ScreenResolution> {
    let screen_w: f32 = web_sys::window()
        .unwrap()
        .inner_width()
        .map(|f| f.as_f64().unwrap() as f32)
        .map_err(JsError::from_js_value)?;
    let screen_h: f32 = web_sys::window()
        .unwrap()
        .inner_height()
        .map(|f| f.as_f64().unwrap() as f32)
        .map_err(JsError::from_js_value)?;
    Ok(ScreenResolution::iter()
        .filter(|res| {
            let (w, h) = res.pixels();
            w * 0.75 < screen_w && 0.75 * h < screen_h
        })
        .last()
        .unwrap_or(ScreenResolution::Low))
}
