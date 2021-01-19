use crate::gui::sprites::{paths::SPRITE_PATHS, Sprites, *};
use crate::prelude::*;
use div::{doc, DivError};
use paddle::{DisplayArea, Rectangle};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlImageElement};

/// TODO: This component could be a good start to experiment with )Svelte / Mogwai / ...) component integration to paddle
pub struct ResourcesComponent {
    parent: HtmlElement,
    currently_displayed: Vec<(ResourceType, i64)>,
    area: Rectangle,
}

use mogwai::prelude::*;
#[allow(unused_braces)]
pub fn mogwai_res_node(n: impl ToString, r: ResourceType) -> ViewBuilder<HtmlElement> {
    let img = Sprites::new_image_node_builder(r.sprite().default());
    builder!(
        <div class="res">
            <div> { n.to_string() } </div>
            { img }
        </div>
    )
}
#[allow(unused_braces)]
pub fn mogwai_karma_res_node(n: i64) -> ViewBuilder<HtmlElement> {
    let img = Sprites::new_image_node_builder(SpriteIndex::Simple(SingleSprite::Karma));
    builder!(
        <div class="res">
            <div> { n.to_string() } </div>
            { img }
        </div>
    )
}

impl ResourcesComponent {
    pub fn new() -> PadlResult<Self> {
        let window = web_sys::window().ok_or(DivError::MissingWindow)?;
        let doc = window.document().ok_or(DivError::MissingDocument)?;
        let node: HtmlElement = doc
            .create_element("div")?
            .dyn_into()
            .map_err(|_| DivError::JsCastError)?;
        node.set_class_name("pdl-res-comp");
        node.style().set_property("pointer-events", "None")?;
        node.style().set_property("position", "absolute")?;

        Ok(ResourcesComponent {
            parent: node,
            currently_displayed: vec![],
            area: Rectangle::default(),
        })
    }
    // Call this once after initialization
    pub fn attach(&mut self, display: &DisplayArea) {
        display.add_html(self.parent.clone().into());
    }
    pub fn update(&mut self, resis: &[(ResourceType, i64)]) -> PadlResult<()> {
        if self.currently_displayed != resis {
            self.clear();
            for (res, n) in resis {
                let new_node = Self::new_resource_element(*res, *n)?;
                self.parent.append_with_node_1(&new_node)?;
            }
        }
        self.currently_displayed.clear();
        self.currently_displayed.extend_from_slice(resis);
        Ok(())
    }
    pub fn clear(&mut self) {
        self.parent.remove_all_children();
    }
    // Call this for setting size and position
    pub fn draw(&mut self, display: &DisplayArea, max_area: &Rectangle) -> PadlResult<()> {
        let area = display.frame_to_browser_area(*max_area);
        if self.area != area {
            let x = area.x() as u32;
            let y = area.y() as u32;
            let w = area.width() as u32;
            let h = area.height() as u32;
            self.parent
                .style()
                .set_property("top", &(y.to_string() + "px"))?;
            self.parent
                .style()
                .set_property("left", &(x.to_string() + "px"))?;
            self.parent
                .style()
                .set_property("width", &(w.to_string() + "px"))?;
            self.parent
                .style()
                .set_property("height", &(h.to_string() + "px"))?;
            self.area = area;
        }

        Ok(())
    }
    fn new_resource_element(res: ResourceType, n: i64) -> PadlResult<Element> {
        let node = doc().unwrap().create_element("div").unwrap();
        node.set_class_name("pdl-res-comp-el");
        let number = doc().unwrap().create_element("p")?;
        number.set_inner_html(&n.to_string());
        let img = HtmlImageElement::new().unwrap();

        let i = match res.sprite() {
            SpriteSet::Simple(x) => x.index_in_vector(),
            _ => unreachable!(),
        };
        let img_src = SPRITE_PATHS[i];
        img.set_src(img_src);

        node.append_child(&number)?;
        node.append_child(&img)?;
        Ok(node)
    }
}
