//! Pre-rendered shapes that can be drawn like sprites

use lyon::lyon_tessellation::{FillOptions, StrokeTessellator};
use lyon::{math::point, path::Path, tessellation::*};
use paddle::ShapeDesc;
use paddle::{AbstractMesh, ComplexShape, Rectangle, ShapeRenderer};

// paint: DARK_GREEN.into(),
pub const SHAPE_LEFT_ARROW: ShapeDesc = ShapeDesc::named("left_arrow");
pub const SHAPE_RIGHT_ARROW: ShapeDesc = ShapeDesc::named("right_arrow");
pub const SHAPE_FRAME: ShapeDesc = ShapeDesc::named("frame");

pub fn load_shapes() {
    let base = Rectangle::new_sized((200, 100));

    SHAPE_LEFT_ARROW.define(build_arrow(base, true));
    SHAPE_RIGHT_ARROW.define(build_arrow(base, false));

    let base = Rectangle::new_sized((600, 200));
    SHAPE_FRAME.define(build_frame(base));
}

/// Shape used as button to go back/forth
fn build_arrow(total_area: Rectangle, left: bool) -> ComplexShape {
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
    builder.begin(point(x0, y2));
    builder.line_to(point(x1, y0));
    builder.line_to(point(x1, y1));
    builder.line_to(point(x2, y1));
    builder.line_to(point(x2, y3));
    builder.line_to(point(x1, y3));
    builder.line_to(point(x1, y4));
    builder.close();

    let path = builder.build();

    // Tesselate path to mesh
    let mut mesh = AbstractMesh::new();
    let mut tessellator = FillTessellator::new();
    let mut shape = ShapeRenderer::new(&mut mesh);

    tessellator
        .tessellate_path(&path, &FillOptions::default(), &mut shape)
        .unwrap();
    ComplexShape::new(mesh, total_area)
}

fn build_frame(area: Rectangle) -> ComplexShape {
    // Create enclosing path
    let mut builder = Path::builder();
    builder.begin(point(area.x(), area.y()));
    builder.line_to(point(area.x() + area.width(), area.y()));
    builder.line_to(point(area.x() + area.width(), area.y() + area.height()));
    builder.line_to(point(area.x(), area.y() + area.height()));
    builder.close();

    let path = builder.build();

    // Tesselate path to mesh
    let mut mesh = AbstractMesh::new();
    let mut tessellator = StrokeTessellator::new();
    let mut shape = ShapeRenderer::new(&mut mesh);

    let thickness = 5.0;

    tessellator
        .tessellate_path(
            &path,
            &StrokeOptions::default().with_line_width(thickness),
            &mut shape,
        )
        .unwrap();
    ComplexShape::new(mesh, area)
}
