use quicksilver::prelude::*;
use quicksilver::graphics::{Mesh, ShapeRenderer, Drawable};
use quicksilver::lyon::{
    path::{Path},
    math::{point},
    tessellation::*,
};
use crate::gui::{
    utils::*,
    z::*,
};
use crate::game::map::map_segment::MapSegment;

impl MapSegment {
    pub fn tesselate_rivers(&mut self) {
        let area = self.scaled_base_shape();
        self.water_mesh.clear();
        let norm_area = self.base_shape();
        let total_area = norm_area.fit_into(&area, FitStrategy::Center);
        let scaling = total_area.width() / norm_area.width();

        let main_river_area = Rectangle::new(
            (area.x() - 0.5 * scaling, (self.h/2.0).floor() * scaling),
            ((self.w + 1.0) * scaling, scaling)
        );
        let main_path = river_path(main_river_area, 2);
        add_path_to_mesh(&mut self.water_mesh, &main_path, 0.75 * scaling);

        for stream_points in &mut self.streams {
            let mut stream_points: Vec<Vector> = 
                stream_points.iter()
                .map(|tup| (*tup).into())
                .collect();
            scale_vec(&mut stream_points, scaling);
            add_path_to_mesh(
                &mut self.water_mesh,
                &stream_path(&stream_points),
                0.2 * scaling
            );
        }
    }
}

pub fn tesselate_map_background(base_shape: Rectangle, w: i32, h: i32) -> Mesh {
    let mut mesh = Mesh::new();

    let width = base_shape.width();
    let height = base_shape.height();
    let dx =  width / w as f32;
    let dy = height / h as f32;
    let thickness = 1.0;
    
    for x in 0..w+2 {
        let x = dx * x as f32;
        let line = v_line((x,0), height, thickness);
        line.draw(
            &mut mesh,
            Col(TRANSPARENT_BLACK),
            Transform::IDENTITY,
            Z_GRID,
        );
    }
    for y in 0..h+2 {
        let y = dy * y as f32;
        let line = h_line((0,y), width + dx, thickness);
        line.draw(
            &mut mesh,
            Col(TRANSPARENT_BLACK),
            Transform::IDENTITY,
            Z_GRID,
        );
    }
    mesh
}

fn scale_vec(points: &mut Vec<Vector>, scaling: f32) {
    points.iter_mut().for_each(
        |p| *p *= scaling
    );
}

fn river_path(area: Rectangle, windings: usize) -> Path {

    let dx = area.width() / windings as f32 / 4.0;
    let dy = area.height() / 2.0;
    let x0 = area.x();
    let y0 = area.y() + dy;

    let mut builder = Path::builder();
    builder.move_to(point(x0, y0));

    for i in 0..windings {
        let x = x0 + 4.0 * dx * i as f32;
        builder.quadratic_bezier_to(point(x + dx, y0 + dy), point(x + 2.0*dx, y0));
        builder.quadratic_bezier_to(point(x + 3.0*dx, y0 - dy), point(x + 4.0*dx, y0));
    }

    builder.build()
}

fn stream_path(points: &[Vector]) -> Path {
    let mut builder = Path::builder();
    let p0 = points[0];
    builder.move_to(point(p0.x, p0.y));
    for slice in points[1..].windows(2) {
        match slice {
            &[p,q] => {
                let r = (p + q) / 2.0;
                builder.quadratic_bezier_to(point(p.x, p.y), point(r.x, r.y));
            }
            _ => {panic!()},
        }
    }

    builder.build()
}

fn add_path_to_mesh(mesh: &mut Mesh, path: &Path, thickness: f32) {
    let mut shape = ShapeRenderer::new(mesh, MAP_BLUE);
    shape.set_z(Z_RIVER as f32);
    let mut tessellator = StrokeTessellator::new();
    tessellator.tessellate_path(
        path,
        &StrokeOptions::default()
            .with_line_width(thickness)
            .with_line_cap(LineCap::Round),
        &mut shape
    ).unwrap();
}