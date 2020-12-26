//! Pre-rendered shapes that can be drawn like sprites

use crate::gui::utils::*;
use lyon::lyon_tessellation::{FillOptions, StrokeTessellator};
use lyon::{math::point, path::Path, tessellation::*};
use paddle::quicksilver_compat::graphics::{Mesh, ShapeRenderer};
use paddle::Rectangle;

/// A single mesh of triangles ready to be drawn
pub struct PadlShape {
    pub bounding_box: Rectangle,
    pub mesh: Mesh,
    pub z: i16,
}
#[derive(Debug, Clone, Copy)]
pub enum PadlShapeIndex {
    LeftArrow = 0,
    RightArrow = 1,
    Frame = 2,
}

pub fn load_shapes() -> Vec<PadlShape> {
    let z = 0;
    let mut shapes = Vec::new();
    let base = Rectangle::new_sized((200, 100));

    shapes.push(PadlShape {
        mesh: build_arrow(base, true, z),
        bounding_box: base,
        z,
    });

    shapes.push(PadlShape {
        mesh: build_arrow(base, false, z),
        bounding_box: base,
        z,
    });

    let base = Rectangle::new_sized((600, 200));
    shapes.push(PadlShape {
        mesh: build_frame(base, z),
        bounding_box: base,
        z,
    });

    shapes
}

impl PadlShape {
    pub fn set_z(&mut self, z: i16) {
        if self.z != z {
            self.z = z;
            self.mesh.set_z(z);
        }
    }
}

/// Shape used as button to go back/forth
fn build_arrow(total_area: Rectangle, left: bool, z: i16) -> Mesh {
    let w = total_area.size.x;
    let h = total_area.size.y;
    let mut x0 = total_area.pos.x;
    let mut x1 = total_area.pos.x + total_area.size.x * 0.38195;
    let mut x2 = total_area.pos.x + total_area.size.x;

    let d = h / 3.0;
    let mut y0 = total_area.pos.y;
    let mut y1 = total_area.pos.y + d;
    let mut y2 = total_area.pos.y + h / 2.0;
    let mut y3 = total_area.pos.y + h - d;
    let mut y4 = total_area.pos.y + h;

    if !left {
        x0 = w - x0;
        x1 = w - x1;
        x2 = w - x2;
        y0 = h - y0;
        y1 = h - y1;
        y2 = h - y2;
        y3 = h - y3;
        y4 = h - y4;
    }

    // Create enclosing path
    let mut builder = Path::builder();
    builder.move_to(point(x0, y2));
    builder.line_to(point(x1, y0));
    builder.line_to(point(x1, y1));
    builder.line_to(point(x2, y1));
    builder.line_to(point(x2, y3));
    builder.line_to(point(x1, y3));
    builder.line_to(point(x1, y4));
    builder.line_to(point(x0, y2));
    builder.close();

    let path = builder.build();

    // Tesselate path to mesh
    let mut mesh = Mesh::new();
    let mut tessellator = FillTessellator::new();
    let mut shape = ShapeRenderer::new(&mut mesh, DARK_GREEN);
    shape.set_z(z as f32);

    tessellator
        .tessellate_path(path.into_iter(), &FillOptions::default(), &mut shape)
        .unwrap();

    mesh
}

fn build_frame(area: Rectangle, z: i16) -> Mesh {
    // Create enclosing path
    let mut builder = Path::builder();
    builder.move_to(point(area.x(), area.y()));
    builder.line_to(point(area.x() + area.width(), area.y()));
    builder.line_to(point(area.x() + area.width(), area.y() + area.height()));
    builder.line_to(point(area.x(), area.y() + area.height()));
    builder.close();

    let path = builder.build();

    // Tesselate path to mesh
    let mut mesh = Mesh::new();
    let mut tessellator = StrokeTessellator::new();
    let mut shape = ShapeRenderer::new(&mut mesh, DARK_GREEN);
    shape.set_z(z as f32);

    let thickness = 5.0;

    tessellator
        .tessellate_path(
            path.into_iter(),
            &StrokeOptions::default().with_line_width(thickness),
            &mut shape,
        )
        .unwrap();

    mesh
}
