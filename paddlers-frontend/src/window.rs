use crate::prelude::*;
use paddle::{quicksilver_compat::Vector, JsError, WebGLCanvas};
use paddlers_shared_lib::game_mechanics::town::{TOWN_X, TOWN_Y};
use strum::IntoEnumIterator;

pub fn adapt_window_size(window: &mut WebGLCanvas) -> PadlResult<()> {
    window.fit_to_screen(20.0)?;
    let area = window.browser_region();

    let x = area.x();
    let y = area.y();
    let w = area.width();
    let h = area.height();

    div::reposition(x as u32, y as u32)?;
    window.set_size((w, h));
    div::resize(w as u32, h as u32)?;
    Ok(())
}

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
