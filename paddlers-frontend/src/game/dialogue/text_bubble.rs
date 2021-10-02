use lyon::{math::point, path::Path, tessellation::*};
use paddle::*;

/// Defines how rounded the bubble appears (0 = rectangle)
const RELATIVE_CURVATURE: f32 = 0.125;
/// Height of the text bubble tail (only applied to smaller text bubble tail, which points down-right)
const RELATIVE_SMALL_TAIL_H: f32 = 0.075;

/// Creates a shape for tesselation that forms a left-open text bubble.
/// total_area: Maximum space that text bubble should use
/// text_area: Minimum space that text should have. Must be a subset of total_area.
pub fn build_text_bubble_to_left(total_area: Rectangle, text_area: Rectangle) -> ComplexShape {
    // Define start point
    let x0 = total_area.pos.x;
    let y0 = total_area.pos.y + total_area.size.y / 2.0;
    // Define text corners
    let left = text_area.pos.x;
    let top = text_area.pos.y;
    let right = text_area.pos.x + text_area.size.x;
    let bottom = text_area.pos.y + text_area.size.y;
    // Geometric helpers
    let curvature = text_area.size.x * RELATIVE_CURVATURE;
    // Define control points for bezier curves
    let ctrl_x0 = text_area.pos.x;
    let ctrl_y0 = y0;
    let ctrl_x1 = text_area.pos.x + text_area.size.x / 2.0;
    let ctrl_y1 = text_area.pos.y - curvature;
    let ctrl_x2 = text_area.pos.x + text_area.size.x + curvature;
    let ctrl_y2 = text_area.pos.y + text_area.size.y + curvature;

    // Create enclosing path
    let mut builder = Path::builder();
    builder.begin(point(x0, y0));

    builder.quadratic_bezier_to(point(ctrl_x0, ctrl_y0), point(left, top));
    builder.quadratic_bezier_to(point(ctrl_x1, ctrl_y1), point(right, top));
    builder.quadratic_bezier_to(point(ctrl_x2, ctrl_y0), point(right, bottom));
    builder.quadratic_bezier_to(point(ctrl_x1, ctrl_y2), point(left, bottom));
    builder.quadratic_bezier_to(point(ctrl_x0, ctrl_y0), point(x0, y0));
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

/// Variation of a text bubble that opens towards the bottom right
pub fn build_text_bubble_to_bottom(total_area: Rectangle, text_area: Rectangle) -> ComplexShape {
    // Define text corners
    let left = text_area.pos.x;
    let top = text_area.pos.y;
    let right = text_area.pos.x + text_area.size.x;
    let bottom = text_area.pos.y + text_area.size.y;
    // Geometric helpers
    let curvature = text_area.size.x * RELATIVE_CURVATURE;
    let tail_h = text_area.size.y * RELATIVE_SMALL_TAIL_H;
    // Define tail points, with tail positioned bottom-right
    let tail_end_x = total_area.pos.x + total_area.size.x;
    let tail_end_y = total_area.pos.y + total_area.size.y + tail_h / 2.0;
    let tail_start_x = right - tail_h / 2.0;
    let tail_start_y = bottom - tail_h / 2.0;

    // Define control points for bezier curves
    let ctrl_x0 = text_area.pos.x - curvature;
    let ctrl_y0 = text_area.pos.y + text_area.size.y / 2.0;
    let ctrl_x1 = text_area.pos.x + text_area.size.x / 2.0;
    let ctrl_y1 = text_area.pos.y - curvature;
    let ctrl_x2 = text_area.pos.x + text_area.size.x + curvature;
    let ctrl_y2 = text_area.pos.y + text_area.size.y + curvature;
    let ctrl_tail_x = right;
    let ctrl_tail_y = bottom;

    // Create enclosing path
    let mut builder = Path::builder();

    builder.begin(point(tail_end_x, tail_end_y));
    builder.quadratic_bezier_to(point(ctrl_tail_x, ctrl_tail_y), point(tail_start_x, bottom));
    builder.quadratic_bezier_to(point(ctrl_x1, ctrl_y2), point(left, bottom));
    builder.quadratic_bezier_to(point(ctrl_x0, ctrl_y0), point(left, top));
    builder.quadratic_bezier_to(point(ctrl_x1, ctrl_y1), point(right, top));
    builder.quadratic_bezier_to(point(ctrl_x2, ctrl_y0), point(right, tail_start_y));
    builder.quadratic_bezier_to(
        point(ctrl_tail_x, ctrl_tail_y),
        point(tail_end_x, tail_end_y),
    );
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
