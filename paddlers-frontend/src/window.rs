use crate::prelude::*;
use paddle::{quicksilver_compat::Vector, JsError, Window};
use strum::IntoEnumIterator;

pub fn adapt_window_size(window: &mut Window) -> PadlResult<()> {
    let canvas = window.html_element();

    let web_window = web_sys::window().unwrap();
    let w = web_window.inner_width()?.as_f64().unwrap();
    let h = web_window.inner_height()?.as_f64().unwrap();
    canvas.set_width(w as u32);
    canvas.set_height(h as u32);

    // Check what the actual size is after CSS messes with the canvas
    let size_in_browser = canvas.get_bounding_client_rect();
    let x = size_in_browser.width();
    let y = size_in_browser.height();
    let size_in_browser = Vector::new(x as f32, y as f32);
    window.set_size(size_in_browser);

    // Update panes
    let Vector { x, y } = window.screen_offset();
    div::reposition(x as u32, y as u32)?;
    let Vector { x, y } = window.screen_size();
    div::resize(x as u32, y as u32)?;
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
