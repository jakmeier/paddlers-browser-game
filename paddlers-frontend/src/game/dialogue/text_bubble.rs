use lyon::{math::point, path::Path, tessellation::*};
use paddle::*;

/// Creates a shape for tesselation that forms a left-open text bubble.
/// total_area: Maximum space that text bubble should use
/// text_area: Minimum space that text should have. Must be a subset of total_area.
pub fn build_text_bubble(total_area: Rectangle, text_area: Rectangle) -> AbstractMesh {
    // Define start point
    let x0 = total_area.pos.x;
    let y0 = total_area.pos.y + total_area.size.y / 2.0;
    // Define text corners
    let left = text_area.pos.x;
    let top = text_area.pos.y;
    let right = text_area.pos.x + text_area.size.x;
    let bottom = text_area.pos.y + text_area.size.y;
    // Degree of curvature
    let s = text_area.size.x * 0.125;
    // Define control points for bezier curves
    let ctrl_x0 = text_area.pos.x;
    let ctrl_y0 = y0;
    let ctrl_x1 = text_area.pos.x + text_area.size.x / 2.0;
    let ctrl_y1 = text_area.pos.y - s;
    let ctrl_x2 = text_area.pos.x + text_area.size.x + s;
    let ctrl_y2 = text_area.pos.y + text_area.size.y + s;

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
    mesh.normalize();

    mesh
}
