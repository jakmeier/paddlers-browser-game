//! Generic GUI Utilities
//! Keep the dependencies to a minimum,
//! no connection with game logic in here

pub mod colors;
pub use colors::*;
mod progress_bar;
pub use progress_bar::*;

use crate::gui::animation::AnimationState;
use crate::gui::sprites::*;
use crate::prelude::*;
use paddle::quicksilver_compat::*;
use paddle::*;
use paddle::{DisplayArea, FitStrategy};

// Improvement: Would be nice to have Copy here (maybe with string interning)
#[derive(Debug, Clone)]
pub enum RenderVariant {
    #[allow(dead_code)]
    Img(SpriteSet),
    ImgWithImgBackground(SpriteSet, SingleSprite),
    ImgWithColBackground(SpriteSet, Color),
    ImgWithHoverAlternative(SpriteSet, SpriteSet),
    ImgWithHoverShape(SpriteSet, ShapeDesc, Color),
    ImgCollection(ImageCollection),
    Text(String),
    TextWithColBackground(String, Color),
    Shape(ShapeDesc, Color),
    Hide,
}

pub fn draw_animated_sprite(
    asset: &mut Sprites,
    window: &mut DisplayArea,
    max_area: &Rectangle,
    i: SpriteSet,
    z: i16,
    fit_strat: FitStrategy,
    animation_state: &AnimationState,
    frame: u32,
) {
    let (image, transform) = i.animated(&animation_state.direction, frame);
    draw_image(asset, window, max_area, image, z, fit_strat, transform)
}
pub fn draw_static_image(
    asset: &mut Sprites,
    window: &mut DisplayArea,
    max_area: &Rectangle,
    i: SpriteIndex,
    z: i16,
    fit_strat: FitStrategy,
) {
    draw_image(
        asset,
        window,
        max_area,
        i,
        z,
        fit_strat,
        Transform::IDENTITY,
    )
}
pub fn draw_image(
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    max_area: &Rectangle,
    i: SpriteIndex,
    z: i16,
    fit_strat: FitStrategy,
    texture_transform: Transform,
) {
    let img = sprites.index(i);
    let unfitted_area = Rectangle::new(max_area.pos, img.natural_size());
    let area = unfitted_area.fit_into_ex(max_area, fit_strat, false);

    // To apply the texture properly, we hav to move around the plane before and after
    let half_size = area.size / 2.0;
    let left_pos = area.pos;
    let left_transform = Transform::translate(left_pos + half_size);
    let right_transform = Transform::translate(-left_pos - half_size);

    window.draw_ex(
        &area,
        &img,
        left_transform * texture_transform * right_transform,
        z,
    );
}

pub fn draw_image_collection(
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    max_area: &Rectangle,
    collection: &ImageCollection,
    z: i16,
    fit_strat: FitStrategy,
) {
    let unfitted_area = Rectangle::new(max_area.pos, collection.size);
    let fitted_area = unfitted_area.fit_into_ex(max_area, fit_strat, true);
    let xs = fitted_area.width() / unfitted_area.width();
    let ys = fitted_area.height() / unfitted_area.height();
    for img in &collection.images {
        let sub_area = Rectangle::new(
            fitted_area.pos + img.pos.pos.times((xs, ys)),
            img.pos.size.times((xs, ys)),
        );
        window.draw_ex(
            &sub_area,
            &sprites.index(img.img),
            Transform::IDENTITY,
            z + img.z_offset,
        );
    }
}

pub fn h_line(start: impl Into<Vector>, len: f32, thickness: f32) -> Rectangle {
    Rectangle::new(start, (len, thickness))
}
pub fn v_line(start: impl Into<Vector>, len: f32, thickness: f32) -> Rectangle {
    Rectangle::new(start, (thickness, len))
}

#[derive(Debug, Clone)]
pub struct ImageCollection {
    size: Vector,
    images: Vec<SubImg>,
    background: Option<SpriteSet>,
}

#[derive(Debug, Clone)]
pub struct SubImg {
    pos: Rectangle,
    z_offset: i16,
    img: SpriteIndex,
}

impl ImageCollection {
    pub fn new(size: impl Into<Vector>, images: Vec<SubImg>) -> Self {
        Self {
            size: size.into(),
            images,
            background: None,
        }
    }
    pub fn background(&self) -> &Option<SpriteSet> {
        &self.background
    }
    pub fn with_background(mut self, bkg: SingleSprite) -> Self {
        self.background = Some(SpriteSet::Simple(bkg));
        self
    }
}
impl SubImg {
    pub fn new(
        img: SingleSprite,
        offset: impl Into<Vector>,
        size: impl Into<Vector>,
        z_offset: i16,
    ) -> Self {
        Self {
            pos: Rectangle::new(offset, size),
            img: SpriteIndex::Simple(img),
            z_offset,
        }
    }
}
