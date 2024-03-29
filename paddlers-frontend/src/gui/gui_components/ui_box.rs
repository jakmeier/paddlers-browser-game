use super::*;
use crate::gui::{sprites::*, utils::*};
use crate::prelude::*;
use chrono::NaiveDateTime;
use paddle::quicksilver_compat::*;
use paddle::*;
use paddle::{quicksilver_compat::geom::Triangle, utc_now};
use paddle::{FitStrategy, NutsCheck};
use paddlers_shared_lib::civilization::CivilizationPerk;
use std::f32::consts::SQRT_2;

#[derive(Clone, Debug)]
/// A UI element is an individual area for the player to interacts with.
/// It can be clicked (if a condition is met), hovered, and it may have an overlay showing a "cooldown" effect for abilities.
/// At the moment, UiElements do just what they need to do right now but probably they should be more general in the future.
/// For example, it could also be an enum and differentiate between variants with/without overlay.
/// Or maybe, UI elements could even have their own ECS-like (sub-)structure so that components that can be added flexibly
pub struct UiElement {
    display: RenderVariant,
    pub overlay: Option<(NaiveDateTime, NaiveDateTime)>,
    condition: Option<Condition>,
    on_click: Option<ClickOutput>,
}
#[derive(Clone, Debug)]
/// A grid of UI elements.
pub struct UiBox {
    area: Rectangle,
    elements: Vec<UiElement>,
    notification_indicator: Option<Vec<usize>>,
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
        window: &mut DisplayArea,
        sprites: &mut Sprites,
        tp: &mut TableTextProvider,
        now: NaiveDateTime,
        area: &Rectangle,
        mouse_pos: Option<Vector>,
        z: i16,
    ) {
        self.area = *area;
        let grid = area.grid(self.columns, self.rows);
        let mut notifications = self.notification_indicator.as_ref().map(|vec| vec.iter());
        let z_button_background = z;
        let z_button = z + 1;
        let z_overlay = z + 2;
        let z_button_decoration = z + 3;
        let z_menu_text = z + 5;

        for (el, draw_area) in self.elements.iter().zip(grid) {
            let img = match &el.display {
                RenderVariant::Img(img) => Some(img),
                RenderVariant::ImgWithColBackground(img, col) => {
                    window.draw_ex(
                        &draw_area.padded(self.margin),
                        col,
                        Transform::IDENTITY,
                        z_button_background,
                    );
                    Some(img)
                }
                RenderVariant::ImgWithImgBackground(img, bkg) => {
                    draw_static_image(
                        sprites,
                        window,
                        &draw_area.padded(self.margin),
                        SpriteIndex::Simple(*bkg),
                        z_button_background,
                        FitStrategy::Center,
                    );
                    Some(img)
                }
                RenderVariant::ImgWithHoverAlternative(img, hov) => {
                    if let Some(mouse_pos) = mouse_pos {
                        if mouse_pos.overlaps_rectangle(&draw_area) {
                            Some(hov)
                        } else {
                            Some(img)
                        }
                    } else {
                        Some(img)
                    }
                }
                RenderVariant::ImgWithHoverShape(img, hov, col) => {
                    if let Some(mouse_pos) = mouse_pos {
                        if mouse_pos.overlaps_rectangle(&draw_area) {
                            window.draw_positioned_shape(
                                &draw_area.padded(self.margin),
                                hov,
                                col,
                                FitStrategy::Center,
                                z_overlay,
                            );
                        }
                    }
                    Some(img)
                }
                RenderVariant::Shape(s, col) => {
                    window.draw_positioned_shape(
                        &draw_area.padded(self.margin),
                        s,
                        col,
                        FitStrategy::Center,
                        z_button_background,
                    );
                    None
                }
                RenderVariant::Text(t) => {
                    tp.text_pool
                        .allocate()
                        .write(window, &draw_area, z_menu_text, FitStrategy::Center, &t)
                        .nuts_check();
                    None
                }
                RenderVariant::TextWithColBackground(t, col) => {
                    window.draw_ex(
                        &draw_area.padded(self.margin),
                        col,
                        Transform::IDENTITY,
                        z_button_background,
                    );
                    tp.text_pool
                        .allocate()
                        .write(window, &draw_area, z_menu_text, FitStrategy::Center, &t)
                        .nuts_check();
                    None
                }
                RenderVariant::Hide => None,
                RenderVariant::ImgCollection(collection) => {
                    draw_image_collection(
                        sprites,
                        window,
                        &draw_area.padded(self.padding + self.margin),
                        collection,
                        z_button,
                        FitStrategy::Center,
                    );
                    collection.background().as_ref()
                }
            };
            if let Some(img) = img {
                draw_static_image(
                    sprites,
                    window,
                    &draw_area.padded(self.padding + self.margin),
                    img.default(),
                    z_button,
                    FitStrategy::Center,
                );
            }
            if el.overlay.is_some() {
                el.draw_overlay(window, &draw_area.padded(self.margin), now, z_overlay);
            }
            if let Some(indicator) = notifications.as_mut().and_then(|iter| iter.next()) {
                if *indicator > 0 {
                    let x = draw_area.pos.x + draw_area.size.x * 3.0 / 4.0;
                    let y = draw_area.pos.y + draw_area.size.y / 4.0;
                    let center = (x, y);
                    let radius = draw_area.size.x / 4.0;
                    let notification_area = Circle::new(center, radius);
                    window.draw_ex(
                        &notification_area,
                        &WHITE,
                        Transform::IDENTITY,
                        z_button_decoration,
                    );
                    let d = radius * SQRT_2;
                    let text_area = Rectangle::new_sized((d, d)).with_center(center);
                    // FIXME: This translation is necessary because somehow CSS centering doesn't quite work the way I thought.
                    // Seems to be only with manjari font but needs investigation.
                    let text_area = text_area.translate((0.0, 5.0));
                    tp.text_pool
                        .allocate()
                        .write(
                            window,
                            &text_area,
                            z_menu_text,
                            FitStrategy::Center,
                            &indicator.to_string(),
                        )
                        .nuts_check();
                }
            }
        }
    }
    fn click(&self, mouse: Vector) -> Option<(ClickOutput, Option<Condition>)> {
        if let Some(el) = self.find_element_under_mouse(mouse) {
            el.click()
        } else {
            None
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
            notification_indicator: None,
        }
    }
    /// Delete all UI elements without changing layout properties
    pub fn clear(&mut self) {
        self.elements.clear();
        self.notification_indicator = None;
    }

    pub fn add(&mut self, el: UiElement) {
        self.elements.push(el);
        if self.columns * self.rows < self.elements.len() {
            paddle::println!("Warning: Not all elements of the UI Area will be visible")
        }
    }
    pub fn update_notifications(&mut self, notif: Option<Vec<usize>>) {
        self.notification_indicator = notif;
    }
    fn element_index_under_mouse(&self, mouse: impl Into<Vector>) -> Option<usize> {
        let dx = self.area.width() / self.columns as f32;
        let dy = self.area.height() / self.rows as f32;
        let pos = mouse.into() - self.area.pos;
        if pos.y < 0.0 || pos.x < 0.0 || dx <= 0.0 || dy <= 0.0 {
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
        self.elements
            .retain(|el| el.on_click.is_none() || *el.on_click.as_ref().unwrap() != val);
    }
    pub fn find_by_on_click(&mut self, val: ClickOutput) -> Option<&mut UiElement> {
        self.elements
            .iter_mut()
            .find(|el| el.on_click.is_some() && *el.on_click.as_ref().unwrap() == val)
    }

    pub fn draw_hover_info(
        &mut self,
        display: &mut DisplayArea,
        res_comp: &mut ResourcesComponent,
        area: &Rectangle,
        mouse_pos: Vector,
    ) -> PadlResult<()> {
        let mouse = mouse_pos;
        if let Some(el) = self.find_element_under_mouse(mouse) {
            if let Some(Condition::HasResources(cost)) = &el.condition {
                // TODO: Calling draw every frame is expensive
                res_comp.update(&cost.0)?;
                res_comp.draw(display, area)?;
            } else {
            }
        } else {
            res_comp.update(&[])?;
        }
        Ok(())
    }
}

impl UiElement {
    fn draw_overlay(
        &self,
        window: &mut DisplayArea,
        area: &Rectangle,
        now: NaiveDateTime,
        z_overlay: i16,
    ) {
        if let Some((start, end)) = self.overlay {
            if now > start && now < end {
                let progress = (now - start).num_microseconds().unwrap() as f32
                    / (end - start).num_microseconds().unwrap() as f32;
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
                    let segment_len = ((1.0 - progress) * 8.0 - i as f32).min(1.0);
                    if segment_len <= 0.0 {
                        break;
                    }
                    let t = Triangle::new(
                        center,
                        border[i],
                        border[i] + ((border[i + 1] - border[i]) * segment_len),
                    );
                    window.draw_ex(
                        &t,
                        &Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 0.8,
                        },
                        Transform::IDENTITY,
                        z_overlay,
                    );
                }
            }
        }
    }
    fn click(&self) -> Option<(ClickOutput, Option<Condition>)> {
        if self.is_active().nuts_check().is_some() {
            self.on_click
                .as_ref()
                .map(|c| (c.clone().into(), self.condition.clone()))
        } else {
            None
        }
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

    pub fn new<T: Into<ClickOutput> + Clone>(on_click: T) -> Self {
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
    pub fn with_text(mut self, t: String) -> Self {
        self.display = RenderVariant::Text(t);
        self
    }
    pub fn with_cooldown(mut self, start: NaiveDateTime, end: NaiveDateTime) -> Self {
        self.overlay = Some((start, end));
        self
    }
    pub fn with_render_variant(mut self, rv: RenderVariant) -> Self {
        self.display = rv;
        self
    }
    pub fn with_background_color(mut self, col: Color) -> Self {
        match self.display {
            RenderVariant::Hide => {
                self.display = RenderVariant::TextWithColBackground("".to_owned(), col);
            }
            RenderVariant::Img(i) => {
                self.display = RenderVariant::ImgWithColBackground(i, col);
            }
            RenderVariant::Text(t) => {
                self.display = RenderVariant::TextWithColBackground(t, col);
            }
            _ => panic!("Not implemented"),
        }
        self
    }
    pub fn with_cost(mut self, cost: Price) -> Self {
        self.condition = Some(Condition::HasResources(cost));
        self
    }
    #[allow(dead_code)]
    pub fn with_karma_condition(mut self, minimum_karma: i64) -> Self {
        self.condition = Some(Condition::HasKarma(minimum_karma));
        self
    }
    pub fn with_perk_condition(mut self, perk: CivilizationPerk) -> Self {
        self.condition = Some(Condition::HasCivPerk(perk));
        self
    }

    pub fn empty() -> Self {
        UiElement {
            display: RenderVariant::Hide,
            overlay: None,
            condition: None,
            on_click: None,
        }
    }
}
