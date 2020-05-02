//! Generic GUI Utilities
//! Keep the dependencies to a minimum,
//! no connection with game logic in here

pub mod colors;
use crate::gui::shapes::PadlShapeIndex;
pub use colors::*;
mod grid;
pub use grid::*;
mod jmr_geometry;
pub use jmr_geometry::*;
mod progress_bar;
pub use progress_bar::*;

use crate::gui::animation::AnimationState;
use crate::gui::sprites::*;
use crate::prelude::*;
use crate::view::FloatingText;
use quicksilver::graphics::Mesh;
use quicksilver::prelude::*;

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

#[derive(Copy, Clone, Debug)]
pub enum FitStrategy {
    TopLeft,
    Center,
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
    window: &mut Window,
    max_area: &Rectangle,
    i: SpriteSet,
    z: i32,
    fit_strat: FitStrategy,
    animation_state: &AnimationState,
    frame: u32,
) -> Result<()> {
    let (image, transform) = i.animated(&animation_state.direction, frame);
    draw_image(asset, window, max_area, image, z, fit_strat, transform)
}
pub fn draw_static_image(
    asset: &mut Sprites,
    window: &mut Window,
    max_area: &Rectangle,
    i: SpriteIndex,
    z: i32,
    fit_strat: FitStrategy,
) -> Result<()> {
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
    window: &mut Window,
    max_area: &Rectangle,
    i: SpriteIndex,
    z: i32,
    fit_strat: FitStrategy,
    transform: Transform,
) -> Result<()> {
    let img = sprites.index(i);
    let mut area = *max_area;
    let img_slope = img.area().height() / img.area().width();
    if img_slope < area.height() / area.width() {
        // high image
        area.size.y = area.width() * img_slope;
        match fit_strat {
            FitStrategy::Center => {
                area = area.translate((0, (max_area.height() - area.height()) / 2.0));
            }
            FitStrategy::TopLeft => {}
        }
    } else {
        area.size.x = area.height() / img_slope;
        match fit_strat {
            FitStrategy::Center => {
                area = area.translate(((max_area.width() - area.width()) / 2.0, 0.0));
            }
            FitStrategy::TopLeft => {}
        }
    }

    window.draw_ex(&area, Img(&img), transform, z);
    Ok(())
}
pub fn draw_shape(
    sprites: &mut Sprites,
    window: &mut Window,
    draw_area: &Rectangle,
    i: PadlShapeIndex,
    fit_strat: FitStrategy,
) {
    let shape = sprites.shape_index(i);
    let place = shape.bounding_box.fit_into_ex(&draw_area, fit_strat, true);
    let factor = (
        place.size.x / shape.bounding_box.size.x,
        place.size.y / shape.bounding_box.size.y,
    );
    let t = Transform::translate(place.pos) * Transform::scale(factor);
    extend_transformed(window.mesh(), &shape.mesh, t);
}

impl FloatingText {
    pub fn write(
        &mut self,
        _window: &Window,
        max_area: &Rectangle,
        _z: i32,                 // TODO
        _fit_strat: FitStrategy, // TODO
        text: &str,
    ) -> PadlResult<()> {
        self.update_text(text);
        self.update_position(max_area)?;
        self.draw();
        Ok(())
    }
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

/// Scales all vertices in the mesh by the given factor, taking (0,0) as origin
pub fn scale_mesh(mesh: &mut Mesh, r: f32) {
    for p in mesh.vertices.iter_mut() {
        p.pos *= r;
        if let Some(mut tp) = p.tex_pos {
            tp *= r;
        }
    }
}

/// Adds all vertices from one mesh to another mesh after applying a transformation
pub fn extend_transformed(mesh: &mut Mesh, other: &Mesh, t: Transform) {
    let n = mesh.vertices.len() as u32;
    for mut vertex in other.vertices.iter().cloned() {
        vertex.pos = t * vertex.pos;
        vertex.tex_pos = vertex.tex_pos.map(|v| t * v);
        mesh.vertices.push(vertex);
    }
    mesh.triangles
        .extend(other.triangles.iter().cloned().map(|mut t| {
            t.indices[0] += n;
            t.indices[1] += n;
            t.indices[2] += n;
            t
        }));
}
