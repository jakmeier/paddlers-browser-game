use crate::gui::{sprites::*, utils::*, z::*};
use crate::prelude::*;
use quicksilver::prelude::*;
use super::*;

#[derive(Clone, Debug)]
pub struct UiElement {
    display: RenderVariant,
    // Extend with enum and more variants once necessary (for overlay and hover info)
    pub overlay: Option<(Timestamp, Timestamp)>,
    condition: Option<Condition>,
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
                        &draw_area.padded(self.margin),
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
    fn click(&self, mouse: Vector) -> PadlResult<Option<(ClickOutput, Option<Condition>)>> {
        if let Some(el) = self.find_element_under_mouse(mouse) {
            el.click()
        } else {
            Ok(None)
        }
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

    pub fn add(&mut self, el: UiElement) {
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
    pub fn find_by_on_click(&mut self, val: ClickOutput) -> Option<&mut UiElement> {
        self.elements.iter_mut().find(|el| el.on_click.is_some() && *el.on_click.as_ref().unwrap() == val)
    }

    pub fn draw_hover_info(
        &mut self,
        window: &mut Window,
        sprites: &mut Sprites,
        floats: &mut[FloatingText;3],
        area: &Rectangle,
    ) -> PadlResult<()> {
        let mouse = window.mouse().pos();
        if let Some(el) = self.find_element_under_mouse(mouse) {
            if let Some(Condition::HasResources(cost)) = &el.condition {
                draw_resources(window, sprites, &cost.0, area, floats, Z_MENU_RESOURCES)?;
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
    fn click(&self) -> PadlResult<Option<(ClickOutput, Option<Condition>)>> {
        self.is_active()?;
        Ok(
            self.on_click.as_ref().map(|c| (c.clone().into(), self.condition.clone()) )
        )
    }
    fn is_active(&self) -> PadlResult<()> {
        if let Some((start, end)) = self.overlay {
            let now = utc_now();
            if start < now && now < end {
                return PadlErrorCode::NotReadyYet.usr();
            }
        }
        Ok(())
    }

    pub fn new<T: Into<ClickOutput> + Clone> (on_click: T) -> Self {
        UiElement {
            display: RenderVariant::Hide,
            overlay: None,
            condition: None,
            on_click: Some(on_click.into()),
        }
    }
    pub fn with_image(mut self, i: SpriteSet) -> Self {
        self.display = RenderVariant::Img(i);
        self
    }
    pub fn with_cooldown(mut self, start: Timestamp, end: Timestamp) -> Self {
        self.overlay = Some((start, end));
        self
    }
    pub fn with_render_variant(mut self, rv: RenderVariant) -> Self {
        self.display = rv;
        self
    }
    pub fn with_background_color(mut self, col: Color) -> Self {
        match self.display {
            RenderVariant::Img(i) => {
                self.display = RenderVariant::ImgWithColBackground(i, col);
            },
            _ => panic!("Not implemented")
        }
        self
    }
    pub fn with_cost(mut self, cost: Price) -> Self {
        self.condition = Some(Condition::HasResources(cost));
        self
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        UiElement {
            display: RenderVariant::Hide,
            overlay: None,
            condition: None,
            on_click: None,
        }
    }
}
