use crate::gui::{sprites::*, utils::*, z::*};
use crate::prelude::*;
use quicksilver::prelude::*;
use super::*;

#[derive(Clone, Debug)]
pub struct UiElement {
    display: RenderVariant,
    // Extend with enum and more variants once necessary (for overlay and hover info)
    pub overlay: Option<(Timestamp, Timestamp)>,
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
        now: Timestamp,
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
            if el.overlay.is_some() {
                el.draw_overlay(window, &draw_area.padded(self.margin), now);
            }
        }

        Ok(())
    }
    fn click(&self, mouse: Vector) -> Option<ClickOutput> {
        self.find_element_under_mouse(mouse)
            .filter(|el| el.is_active())
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
            overlay: None,
            hover_info: None,
            on_click: Some(on_click.into()),
        });
    }

    pub fn add_with_render_variant
    <T: Into<ClickOutput> + Clone>
    (&mut self, rv: RenderVariant, on_click: T) {
        self.add_el(UiElement {
            display: rv,
            overlay: None,
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
            overlay: None,
            hover_info: Some(cost),
            on_click: Some(on_click.into()),
        });
    }

    pub fn add_empty(&mut self) {
        self.add_el(UiElement {
            display: RenderVariant::Hide,
            overlay: None,
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

    pub fn add_with_cooldown
    <T: Into<ClickOutput> + Clone>
    (&mut self, i: SpriteSet, on_click: T, start: Timestamp, end: Timestamp) {
        self.add_el(UiElement {
            display: RenderVariant::Img(i),
            overlay: Some((start, end)),
            hover_info: None,
            on_click: Some(on_click.into()),
        });
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
    pub fn find_by_on_click(&mut self, val: ClickOutput) -> Option<&mut UiElement> {
        self.elements.iter_mut().find(|el| el.on_click.is_some() && *el.on_click.as_ref().unwrap() == val)
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

impl UiElement {
    fn draw_overlay(
        &self,
        window: &mut Window,
        area: &Rectangle,
        now: Timestamp
    ) {
        if let Some((start, end)) = self.overlay {
            if now > start && now < end {
                let progress = (now - start) as f32 / (end - start) as f32;
                let center = area.center();
                let border = [
                    Vector::new(center.x, area.y()),
                    Vector::new(area.x(), area.y()),
                    Vector::new(area.x(), center.y),
                    Vector::new(area.x(), area.y() + area.height()),
                    Vector::new(center.x, area.y() + area.height()),
                    Vector::new(area.x() + area.width(), area.y() + area.height()),
                    Vector::new(area.x() + area.width(), center.y),
                    Vector::new(area.x() + area.width(), area.y()),
                    Vector::new(center.x, area.y()),
                ];
                for i in 0..8 {
                    let segment_len = ((1.0-progress) * 8.0 - i as f32).min(1.0);
                    if segment_len <= 0.0 {
                        break;
                    }
                    let t  = Triangle::new(
                        center,
                        border[i],
                        border[i] + ((border[i+1] - border[i]) * segment_len),
                    );
                    window.draw_ex(
                        &t,
                        Col(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.8 }),
                        Transform::IDENTITY,
                        Z_MENU_BOX_BUTTONS + 1,
                    );
                }
            }
        }
    }
    fn is_active(&self) -> bool {
        if let Some((start, end)) = self.overlay {
            let now = utc_now();
            if start < now && now < end {
                return false;
            }
        }
        true
    }
}
