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
use super::MapSkeleton;

impl MapSkeleton {
    pub fn static_test_map() -> Self {

        let mut streams = vec!();
        streams.push(vec![
                (4,5.5).into(),
                (4,5).into(),
                (5,3).into(),
                (7,1.2).into(),
                (5,0).into(),
                (5,-1.2).into(),
            ]);

        streams.push(vec![
                (1,5.5).into(),
                (3,5).into(),
                (0.5,3).into(),
                (2,1.2).into(),
                (1.2,0).into(),
                (1.3,-1.2).into(),
            ]);
        
        streams.push(vec![
                (8,5.5).into(),
                (6.5,6).into(),
                (10,6.5).into(),
                (9,8.8).into(),
                (13,9).into(),
                (10,10).into(),
            ]);
        MapSkeleton {
            w: 15,
            h: 11,
            streams
        }
    }

    pub fn base_shape(&self) -> Rectangle {
        Rectangle::new(
            (0,0),
            (self.w, self.h),
        )
    }
    pub fn tesselate_rivers(&mut self, area: &Rectangle) -> Mesh {
        let norm_area = self.base_shape();
        let total_area = norm_area.fit_into(&area, FitStrategy::Center);
        let scaling = total_area.width() / norm_area.width();

        let main_river_area = Rectangle::new(
            (0,(self.h/2) as f32 * scaling),
            (self.w as f32 * scaling, scaling)
        );
        let main_path = river_path(main_river_area, 2);
        let mut mesh = Mesh::new();
        add_path_to_mesh(&mut mesh, &main_path, 0.75 * scaling);

        for stream_points in &mut self.streams {
            scale_vec(stream_points, scaling);
            add_path_to_mesh(
                &mut mesh,
                &stream_path(&stream_points),
                0.2 * scaling
            );
        }
        mesh
    }

    pub fn tesselate_background(&self) -> Mesh {
        // For now, the map is static and cannot be scrolled or zoomed
        let mut mesh = Mesh::new();
        self.base_shape().draw(
            &mut mesh,
            Col(MAP_GREEN),
            Transform::IDENTITY,
            Z_TEXTURE,
        );

        let (w,h) = (self.w, self.h);
        let thickness = 0.02;
        for x in 0..w+1 {
            let line = v_line((x,0), h as f32, thickness);
            line.draw(
                &mut mesh,
                Col(Color::BLACK),
                Transform::IDENTITY,
                Z_GRID,
            );
        }
        for y in 0..h+1 {
            let line = h_line((0,y), w as f32, thickness);
            line.draw(
                &mut mesh,
                Col(Color::BLACK),
                Transform::IDENTITY,
                Z_GRID,
            );
        }
        mesh
    }
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