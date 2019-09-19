use crate::gui::{sprites::*, utils::*, z::*};
use crate::prelude::*;
use quicksilver::prelude::*;
use super::*;

#[derive(Clone, Debug)]
struct UiElement {
    display: RenderVariant,
    hover_info: Option<Vec<(ResourceType, i64)>>,
    on_click: Option<ClickOutput>,
}
#[derive(Clone, Debug)]
pub struct UiBox {
    area: Rectangle,
    elements: Vec<UiElement>,
    columns: usize,
    rows: usize,
    padding: f32,
    margin: f32,
}

impl InteractiveTableArea for UiBox {
    fn rows(&self) -> usize {
        2 * self.rows
    }

    fn draw(
        &mut self,
        window: &mut Window,
        sprites: &mut Sprites,
        area: &Rectangle,
    ) -> Result<()> {
        self.area = *area;
        let grid = area.grid(self.columns, self.rows);

        for (el, draw_area) in self.elements.iter().zip(grid) {
            let img = match el.display {
                RenderVariant::Img(img) => Some(img),
                RenderVariant::ImgWithColBackground(img, col) => {
                    window.draw_ex(
                        &draw_area.padded(self.margin),
                        Col(col),
                        Transform::IDENTITY,
                        Z_MENU_BOX_BUTTONS - 1,
                    );
                    Some(img)
                }
                RenderVariant::ImgWithImgBackground(img, bkg) => {
                    draw_static_image(
                        sprites,
                        window,
                        &draw_area,
                        SpriteIndex::Simple(bkg),
                        Z_MENU_BOX_BUTTONS - 1,
                        FitStrategy::Center,
                    )?;
                    Some(img)
                }
                RenderVariant::ImgWithHoverAlternative(img, hov) => {
                    if window.mouse().pos().overlaps_rectangle(&draw_area) {
                        Some(hov)
                    } else {
                        Some(img)
                    }
                }
                RenderVariant::Hide => None,
            };
            if let Some(img) = img {
                draw_static_image(
                    sprites,
                    window,
                    &draw_area.padded(self.padding + self.margin),
                    img.default(),
                    Z_MENU_BOX_BUTTONS,
                    FitStrategy::Center,
                )?;
            }
        }

        Ok(())
    }
    fn click(&self, mouse: Vector) -> Option<ClickOutput> {
        self.find_element_under_mouse(mouse)
            .and_then(|el| el.on_click.clone())
            .map(Into::into)
    }
    fn remove(&mut self, output: ClickOutput) {
        self.remove_with_on_click(output);
    }
}
impl UiBox {
    pub fn new(columns: usize, rows: usize, padding: f32, margin: f32) -> Self {
        UiBox {
            area: Rectangle::default(),
            elements: vec![],
            columns: columns,
            rows: rows,
            padding: padding,
            margin: margin,
        }
    }

    #[allow(dead_code)]
    pub fn add
    <T: Into<ClickOutput> + Clone>
    (&mut self, i: SpriteSet, on_click: T) {
        self.add_el(UiElement {
            display: RenderVariant::Img(i),
            hover_info: None,
            on_click: Some(on_click.into()),
        });
    }

    pub fn add_with_render_variant
    <T: Into<ClickOutput> + Clone>
    (&mut self, rv: RenderVariant, on_click: T) {
        self.add_el(UiElement {
            display: rv,
            hover_info: None,
            on_click: Some(on_click.into()),
        });
    }

    pub fn add_with_background_color_and_cost<T: Into<ClickOutput> + Clone>(
        &mut self,
        i: SpriteSet,
        col: Color,
        on_click: T,
        cost: Vec<(ResourceType, i64)>,
    ) {
        self.add_el(UiElement {
            display: RenderVariant::ImgWithColBackground(i, col),
            hover_info: Some(cost),
            on_click: Some(on_click.into()),
        });
    }

    pub fn add_empty(&mut self) {
        self.add_el(UiElement {
            display: RenderVariant::Hide,
            hover_info: None,
            on_click: None,
        });
    }

    fn add_el(&mut self, el: UiElement) {
        self.elements.push(el);
        if self.columns * self.rows < self.elements.len() {
            println!("Warning: Not all elements of the UI Area will be visible")
        }
    }

    
    fn element_index_under_mouse(&self, mouse: impl Into<Vector>) -> Option<usize> {
        let dx = self.area.width() / self.columns as f32;
        let dy = self.area.height() / self.rows as f32;
        let pos = mouse.into() - self.area.pos;
        if pos.y < 0.0 || pos.x < 0.0 {
            return None;
        }
        let i = (pos.y / dy) as usize * self.columns + (pos.x / dx) as usize;
        Some(i)
    }
    fn find_element_under_mouse(&self, mouse: impl Into<Vector>) -> Option<&UiElement> {
        self.element_index_under_mouse(mouse)
            .and_then(|i| self.elements.get(i))
    }
    fn remove_with_on_click(&mut self, val: ClickOutput) {
        self.elements.retain(|el| el.on_click.is_none() || *el.on_click.as_ref().unwrap() != val);
    }

    pub fn draw_hover_info(
        &mut self,
        window: &mut Window,
        sprites: &mut Sprites,
        font: &mut Asset<Font>,
        area: &Rectangle,
    ) -> Result<()> {
        let mouse = window.mouse().pos();
        if let Some(el) = self.find_element_under_mouse(mouse) {
            if let Some(cost) = &el.hover_info {
                draw_resources(window, sprites, &cost, area, font, Z_MENU_RESOURCES)?;
            }
        }
        Ok(())
    }
}
