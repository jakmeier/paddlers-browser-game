use crate::prelude::*;
use quicksilver::prelude::{Vector, Window};
use stdweb::traits::*;
use stdweb::unstable::TryFrom;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::IHtmlElement;
use strum::IntoEnumIterator;

pub fn adapt_window_size(window: &mut Window) -> PadlResult<()> {
    // TODO: (optimization) cache canvas
    let canvas: CanvasElement = stdweb::web::document()
        .get_element_by_id("game-root")
        .expect("Failed to find specified HTML id")
        .child_nodes()
        .iter()
        .map(CanvasElement::try_from)
        .find(|res| res.is_ok())
        .map(|res| res.unwrap())
        .expect("No canvas");

    let w = stdweb::web::window().inner_width();
    let h = stdweb::web::window().inner_height();
    canvas.set_width(w as u32);
    canvas.set_height(h as u32);

    // Check what the actual size is after CSS messes with the canvas
    let size_in_browser = canvas.get_bounding_client_rect();
    let x = size_in_browser.get_width();
    let y = size_in_browser.get_height();
    let size_in_browser = Vector::new(x as f32, y as f32);
    window.set_size(size_in_browser);

    // Update panes
    let Vector { x, y } = window.screen_offset();
    panes::reposition(x as u32, y as u32)?;
    let Vector { x, y } = window.screen_size();
    panes::resize(x as u32, y as u32)?;
    Ok(())
}

/// Determines a resolution which is close to the current window size
pub fn estimate_screen_size() -> ScreenResolution {
    let screen_w = stdweb::web::window().inner_width() as f32;
    let screen_h = stdweb::web::window().inner_height() as f32;
    ScreenResolution::iter()
        .filter(|res| {
            let (w, h) = res.pixels();
            w * 0.875 < screen_w && 0.875 * h < screen_h
        })
        .last()
        .unwrap_or(ScreenResolution::Low)
}
