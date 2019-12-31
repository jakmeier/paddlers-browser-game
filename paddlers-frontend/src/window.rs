use quicksilver::prelude::{Window, Vector};
use stdweb::unstable::TryFrom;
use crate::stdweb::web::IHtmlElement;
use stdweb::web::html_element::CanvasElement;
use stdweb::traits::*;

pub fn adapt_window_size(window: &mut Window) {

    // TODO: (optimization) cache canvas
    let canvas: CanvasElement =
    stdweb::web::document().get_element_by_id("game-root")
        .expect("Failed to find specified HTML id").child_nodes().iter()
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
    let x = size_in_browser.get_width() as f32;
    let y = size_in_browser.get_height() as f32;
    let size_in_browser = Vector::new(x,y);
    window.set_size(size_in_browser);
}