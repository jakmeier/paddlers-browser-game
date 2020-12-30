//! Generic GUI Utilities
//! Keep the dependencies to a minimum,
//! no connection with game logic in here

pub mod colors;
use crate::gui::shapes::PadlShapeIndex;
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
    ImgWithHoverShape(SpriteSet, PadlShapeIndex),
    Text(String),
    TextWithColBackground(String, Color),
    Shape(PadlShapeIndex),
    Hide,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Undirected,
    North,
    East,
    South,
    West,
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
    text_transform: Transform,
) {
    let img = sprites.index(i);
    let unfitted_area = Rectangle::new(max_area.pos, img.natural_size());
    let area = unfitted_area.fit_into_ex(max_area, fit_strat, false);
    window.draw_ex(&area, Background::ImgView(&img, text_transform), Transform::IDENTITY, z);
}
pub fn draw_shape(
    sprites: &mut Sprites,
    window: &mut DisplayArea,
    draw_area: &Rectangle,
    i: PadlShapeIndex,
    fit_strat: FitStrategy,
    z: i16,
) {
    let shape = sprites.shape_index(i);
    let place = shape.bounding_box.fit_into_ex(&draw_area, fit_strat, true);
    let factor = (
        place.size.x / shape.bounding_box.size.x,
        place.size.y / shape.bounding_box.size.y,
    );
    let t = Transform::translate(place.pos) * Transform::scale(factor);
    window.draw_mesh_ex(&shape.mesh, t, z);
}

pub fn horizontal_flip() -> Transform {
    Transform::from_array([[-1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, 1f32]])
}

pub fn h_line(start: impl Into<Vector>, len: f32, thickness: f32) -> Rectangle {
    Rectangle::new(start, (len, thickness))
}
pub fn v_line(start: impl Into<Vector>, len: f32, thickness: f32) -> Rectangle {
    Rectangle::new(start, (thickness, len))
}
