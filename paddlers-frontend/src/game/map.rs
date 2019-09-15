mod map_tesselation;
mod village_meta;
mod map_segment;

use quicksilver::prelude::*;
use quicksilver::graphics::{Mesh};
use crate::gui::{
    sprites::*,
    utils::*,
    z::*,
};
use map_segment::MapSegment;
use map_tesselation::*;
pub use village_meta::VillageMetaInfo;

pub struct GlobalMap {
    grid_mesh: Mesh,
    segments: Vec<MapSegment>,
    villages: Vec<VillageMetaInfo>,
    scaling: f32,
    x_offset: f32,
}

impl GlobalMap {
    pub fn new(view_size: Vector) -> Self {
        let scaling = Self::calculate_scaling(view_size);
        let (w,h) = Self::display_shape();
        let view_port = Rectangle::new((0,0), Vector::new(w,h) * scaling);
        let grid_mesh = tesselate_map_background(view_port, w, h);

        GlobalMap {
            grid_mesh,
            segments: vec![],
            villages: vec![],
            scaling,
            x_offset: 0.0,
        }
    }
    pub fn add_segment(&mut self, streams: Vec<Vec<(f32,f32)>>, villages: Vec<VillageMetaInfo>, min_x: i32, max_x: i32 ) {
        let w = max_x - min_x;
        let h = paddlers_shared_lib::game_mechanics::map::MAP_H as i32;
        let mut segment = MapSegment::new(min_x, 0, w, h, streams);
        segment.tesselate_rivers();
        self.segments.push(segment);
        self.villages.extend(villages.into_iter());
    }
    pub fn render(&mut self, window: &mut Window, sprites: &mut Sprites, area: &Rectangle) -> Result<()> {

        self.apply_scaling(area.size());
        self.draw_grid(window);

        self.draw_water(window, area);
        self.draw_villages(window, sprites)?;

        /* Draw padding background 
         * Atm, this is drawn BELOW water to enable a smooth transition
         * Could be drawn OVER to cut off rivers
         */
        let drawn_area = self.draw_area();
        let x = drawn_area.x() + drawn_area.width();
        let y = drawn_area.y() + drawn_area.height();
        let dx = area.width() - drawn_area.width();
        let dy = area.height() - drawn_area.height();
        if dx > 0.0 {
            let margin = Rectangle::new((x,area.y()),(dx, area.height()));
            window.draw_ex(&margin, Col(MAP_GREEN), Transform::IDENTITY, Z_TEXTURE);
        }
        if dy > 0.0 {
            let margin = Rectangle::new((area.x(),y),(area.width(), dy));
            window.draw_ex(&margin, Col(MAP_GREEN), Transform::IDENTITY, Z_TEXTURE);
        }
        Ok(())
    }
    fn draw_grid(&mut self, window: &mut Window) {
        let t = self.view_transform();
        extend_transformed(window.mesh(), &self.grid_mesh, t);
    }
    fn draw_water(&mut self, window: &mut Window, area: &Rectangle) {
        let visible_frame = Rectangle::new(
            (self.x_offset, 0),
            area.size() / self.scaling,
        );
        for segment in self.segments.iter_mut() {
            if segment.is_visible(visible_frame) {
                segment.apply_scaling(self.scaling);
                window.flush().unwrap();
                window.mesh().extend(&segment.water_mesh);
            }
        }
    }
    fn draw_villages(&mut self, window: &mut Window, sprites: &mut Sprites) -> Result<()> {
        #[cfg(feature="dev_view")]
        self.visualize_control_points(window);

        for vil in &self.villages {
            let (x,y) = vil.coordinates;
            // translate human-readable to nerd indexing
            let (x,y) = (x-1, y-1);
            let sprite_area = Rectangle::new(
                (x as f32 * self.scaling, y as f32 * self.scaling),
                (self.scaling, self.scaling),
            );
            draw_static_image(
                sprites, 
                window,
                &sprite_area, 
                SpriteIndex::Simple(SingleSprite::Shack), 
                Z_BUILDINGS, 
                FitStrategy::Center
            )?;
        }
        Ok(())
    }

    fn display_shape() -> (i32,i32) {
        let w = 15; // TODO: determine dynamically what fits the viewport
        let h = paddlers_shared_lib::game_mechanics::map::MAP_H as i32;
        (w,h)
    }
    fn draw_area(&self) -> Rectangle {
        let (w,h) = Self::display_shape();
        Rectangle::new(
            (0,0),
            (w as f32 * self.scaling, h as f32 * self.scaling)
        )
    }
    pub fn calculate_scaling(view_size: Vector) -> f32 {
        let (w,h) = Self::display_shape();
        let rx = view_size.x / w as f32;
        let ry = view_size.y / h as f32;
        rx.min(ry)
    }
    fn apply_scaling(&mut self, size: Vector) {
        let r = Self::calculate_scaling(size);
        if self.scaling != r {
            scale_mesh(&mut self.grid_mesh, r / self.scaling);
            self.scaling = r;
        }
    }
    fn view_transform(&self) -> Transform {
        Transform::translate((self.x_offset * self.scaling, 0))
    }

    #[cfg(feature="dev_view")]
    fn visualize_control_points(&self, window: &mut Window) {
        let pt = self.scaling / 5.0;
        for s in &self.skeleton.streams {
            for (x,y) in s {
                let area = Rectangle::new(
                    (x * self.scaling - pt/2.0, y * self.scaling - pt/2.0),
                    (pt, pt),
                );
                window.draw_ex(
                    &area, 
                    Col(Color::WHITE), 
                    Transform::rotate(45), 
                    1000
                );
            }
        }
    }
}