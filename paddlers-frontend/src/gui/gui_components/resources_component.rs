use crate::gui::sprites::paths::SPRITE_PATHS;
use crate::gui::sprites::{SpriteSet, WithSprite};
use crate::prelude::*;
use paddle::quicksilver_compat::Rectangle;
use stdweb::web::html_element::ImageElement;
use stdweb::web::*;

pub struct ResourcesComponent {
    pane: panes::PaneHandle,
    parent: HtmlElement,
}

impl ResourcesComponent {
    pub fn new() -> PadlResult<Self> {
        let pane = panes::new_styled_pane(0, 0, 0, 0, "", &["pdl-res-comp"], &[("", "")])?;
        let parent = pane.parent_element()?;
        Ok(ResourcesComponent { pane, parent })
    }
    pub fn hide(&self) -> PadlResult<()> {
        self.pane.hide()?;
        Ok(())
    }
    pub fn show(&self) -> PadlResult<()> {
        self.pane.show()?;
        Ok(())
    }
    pub fn draw(&mut self, max_area: &Rectangle, resis: &[(ResourceType, i64)]) -> PadlResult<()> {
        self.complete_redraw(max_area, resis)?;
        self.pane.show()?;
        Ok(())
    }
    fn complete_redraw(
        &mut self,
        max_area: &Rectangle,
        resis: &[(ResourceType, i64)],
    ) -> PadlResult<()> {
        // Brute-force delete and redraw everything
        self.pane.reposition_and_resize(
            max_area.x() as u32,
            max_area.y() as u32,
            max_area.width() as u32,
            max_area.height() as u32,
        )?;
        self.parent.remove_all_children();
        for (res, n) in resis {
            let new_node = Self::new_resource_element(*res, *n);
            self.parent.append_child(&new_node);
        }
        Ok(())
    }
    fn new_resource_element(res: ResourceType, n: i64) -> Element {
        let node = document().create_element("span").unwrap();
        let number = document().create_text_node(&n.to_string());
        let img = ImageElement::new();

        let i = match res.sprite() {
            SpriteSet::Simple(x) => x.index_in_vector(),
            _ => unreachable!(),
        };
        let img_src = SPRITE_PATHS[i];
        img.set_src(img_src);

        node.append_child(&number);
        node.append_child(&img);
        node
    }
}
