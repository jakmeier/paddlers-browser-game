use crate::game::map::map_segment::MapSegment;
use crate::gui::utils::*;
use ::lyon::{math::point, path::Path, tessellation::*};
use paddle::*;
use paddlers_shared_lib::game_mechanics::map::MAP_H;

impl MapSegment {
    pub fn tesselate_rivers(&mut self) {
        self.water_mesh.clear();
        // Natural size of mesh is not a square but for AbstractMesh to draw properly in its current (unfortunate) state, the mesh needs to be in exactly this area
        let area = Rectangle::new((-1, -1), (2, 2));
        let stretch = self.base_shape().project(&area);
        let d = area.height() / MAP_H as f32;
        let main_river_area = Rectangle::new((area.x() - 0.5 * d, -0.5 * d), (area.width() + d, d));
        let main_path = river_path(main_river_area, 2);
        add_path_to_mesh(&mut self.water_mesh, &main_path, 0.75 * d);

        for stream_points in &mut self.streams {
            let stream_points: Vec<Vector> = stream_points
                .iter()
                .map(|(x, y)| stretch * Vector::new(*x, *y))
                .collect();
            add_path_to_mesh(&mut self.water_mesh, &stream_path(&stream_points), 0.2 * d);
        }
    }
}

pub fn tesselate_grid_net(w: i32, h: i32) -> AbstractMesh {
    let mut mesh = AbstractMesh::new();

    let width = 2.0;
    let height = 2.0;
    let dx = 0.02 * width / w as f32;
    let dy = 0.02 * height / h as f32;

    let rect_triangles = vec![[0, 1, 2], [2, 3, 0]];
    for x in 0..w + 2 {
        let x = width * x as f32 / w as f32 - 1.0;
        let line = v_line((x, -1.0), height, dx);
        // FIXME: tesselation to meshes
        // This essentially ignores any parameters of the lines (line: Rectangle) right now! It would just fills the normalized area...
        // line.tessellate(&mut mesh);
        mesh.add_triangles(
            &vec![
                line.top_left(),
                line.top_left() + Vector::X * line.width(),
                line.bottom_right(),
                line.top_left() + Vector::Y * line.height(),
            ],
            &rect_triangles,
        );
    }
    for y in 0..h + 2 {
        let y = width * y as f32 / h as f32 - 1.0;
        let line = h_line((-1.0, y), width + dx, dy);
        // line.tessellate(&mut mesh);
        mesh.add_triangles(
            &vec![
                line.top_left(),
                line.top_left() + Vector::X * line.width(),
                line.bottom_right(),
                line.top_left() + Vector::Y * line.height(),
            ],
            &rect_triangles,
        );
    }
    mesh
}

fn river_path(area: Rectangle, windings: usize) -> Path {
    let dx = area.width() / windings as f32 / 4.0;
    let dy = area.height() / 2.0;
    let x0 = area.x();
    let y0 = area.y() + dy;

    let mut builder = Path::builder();
    builder.begin(point(x0, y0));

    for i in 0..windings {
        let x = x0 + 4.0 * dx * i as f32;
        builder.quadratic_bezier_to(point(x + dx, y0 + dy), point(x + 2.0 * dx, y0));
        builder.quadratic_bezier_to(point(x + 3.0 * dx, y0 - dy), point(x + 4.0 * dx, y0));
    }
    builder.end(false);

    builder.build()
}

fn stream_path(points: &[Vector]) -> Path {
    let mut builder = Path::builder();
    let p0 = points[0];
    builder.begin(point(p0.x, p0.y));
    for slice in points[1..].windows(2) {
        match slice {
            &[p, q] => {
                let r = (p + q) / 2.0;
                builder.quadratic_bezier_to(point(p.x, p.y), point(r.x, r.y));
            }
            _ => panic!(),
        }
    }
    builder.end(false);

    builder.build()
}

fn add_path_to_mesh(mesh: &mut AbstractMesh, path: &Path, thickness: f32) {
    let mut shape = ShapeRenderer::new(mesh);
    let mut tessellator = StrokeTessellator::new();
    // Use this to change how many triangles are drawn
    #[cfg(not(debug_assertions))]
    let tolerance = 1.0 / 1024.0;
    #[cfg(debug_assertions)]
    let tolerance = 1.0 / 32.0;
    tessellator
        .tessellate_path(
            path,
            &StrokeOptions::default()
                .with_tolerance(tolerance)
                .with_line_width(thickness)
                .with_line_cap(LineCap::Round),
            &mut shape,
        )
        .unwrap();
    // This number is getting quite high, even with high tolerance > 1000 triangles for each segment and with something like 1/2048 tolerance it is more like 6k per segment.
    // This might be okay if transformations on the mesh are done on the GPU, if it's cached, or if it is at least offloaded from the main thread. But for now this stresses the CPU way too much.
    // Especially when running without optimizations, this is very noticeable in FPS.
    // Therefore, in debug mode, the tolerance if set to a high number and the water shapes on map look very rough.
    // println!("Number of triangles in mesh: {}", mesh.triangles.len());
}
