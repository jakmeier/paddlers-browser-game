//! Generic GUI Utilities
//! Keep the dependencies to a minimum,
//! no connection with game logic in here

pub mod colors;
pub use colors::*;
mod grid;
pub use grid::*;
mod jmr_geometry;
pub use jmr_geometry::*;
mod progress_bar;
pub use progress_bar::*;

use crate::gui::animation::AnimationState;
use crate::gui::sprites::*;
use quicksilver::graphics::Mesh;
use quicksilver::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum RenderVariant {
    #[allow(dead_code)]
    Img(SpriteSet),
    ImgWithImgBackground(SpriteSet, SingleSprite),
    ImgWithColBackground(SpriteSet, Color),
    ImgWithHoverAlternative(SpriteSet, SpriteSet),
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

#[inline]
pub fn write_text(
    asset: &mut Asset<Font>,
    window: &mut Window,
    max_area: &Rectangle,
    z: i32,
    fit_strat: FitStrategy,
    text: &str,
) -> Result<f32> {
    write_text_col(asset, window, max_area, z, fit_strat, text, Color::BLACK)
}
pub fn write_text_col(
    asset: &mut Asset<Font>,
    window: &mut Window,
    max_area: &Rectangle,
    z: i32,
    fit_strat: FitStrategy,
    text: &str,
    col: Color,
) -> Result<f32> {
    let mut res = 0.0;
    asset.execute(|font| {
        let style = FontStyle::new(max_area.height(), col);
        let img = font.render(text, &style).unwrap();
        let area = img.area().shrink_and_fit_into(max_area, fit_strat);
        window.draw_ex(&area, Img(&img), Transform::IDENTITY, z);
        res = area.width();
        Ok(())
    })?;
    Ok(res)
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
    for mut vertex in other.vertices.iter().cloned() {
        vertex.pos = t * vertex.pos;
        vertex.tex_pos = vertex.tex_pos.map( |v| t*v ) ;
        mesh.vertices.push(vertex);
    }
    mesh.triangles.extend(other.triangles.iter().cloned());
}
