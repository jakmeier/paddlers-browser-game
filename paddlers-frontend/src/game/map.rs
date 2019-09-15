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
    view_width: i32,
    loaded: (i32,i32),
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
            view_width: w,
            loaded: (0,0),
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
        
        window.draw_ex(area, Col(MAP_GREEN), Transform::IDENTITY, Z_TEXTURE);

        self.apply_scaling(area.size());
        self.draw_grid(window);
        self.draw_water(window, area);
        self.draw_villages(window, sprites)?;

        Ok(())
    }
    pub fn drag(&mut self, v: Vector) {
        self.x_offset += v.x;
    }
    const LOAD_AHEAD: i32 = 10;
    const LOAD_STEP: i32 = 10;
    pub fn update(&mut self) {
        let x = - self.x_offset as i32;
        if self.loaded.0 > x - Self::LOAD_AHEAD {
            let (low, high) = (self.loaded.0 - 1 - Self::LOAD_STEP, self.loaded.0 - 1);
            crate::net::request_map_read(low, high);

            self.loaded.0 = self.loaded.0.min(low);
            self.loaded.1 = self.loaded.1.max(high);
        }
        if self.loaded.1 <  x + self.view_width + Self::LOAD_AHEAD {
            let (low, high) = (self.loaded.1 + 1, self.loaded.1 + 1 + Self::LOAD_STEP);
            crate::net::request_map_read(low, high);

            self.loaded.0 = self.loaded.0.min(low);
            self.loaded.1 = self.loaded.1.max(high);
        }
    }
    fn draw_grid(&mut self, window: &mut Window) {
        let mut x = self.x_offset % 1.0;
        if x > 0.0 { x -= 1.0 }
        let t = Transform::translate((x * self.scaling, 0));
        extend_transformed(window.mesh(), &self.grid_mesh, t);
    }
    fn draw_water(&mut self, window: &mut Window, area: &Rectangle) {
        let visible_frame = Rectangle::new(
            (-self.x_offset, 0),
            area.size() / self.scaling,
        );
        let t = self.view_transform();
        for segment in self.segments.iter_mut() {
            if segment.is_visible(visible_frame) {
                segment.apply_scaling(self.scaling);
                window.flush().unwrap();
                extend_transformed(&mut window.mesh(), &segment.water_mesh, t)
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
            draw_image(
                sprites, 
                window,
                &sprite_area, 
                SpriteIndex::Simple(SingleSprite::Shack), 
                Z_BUILDINGS, 
                FitStrategy::Center,
                self.view_transform(),
            )?;
        }
        Ok(())
    }

    fn display_shape() -> (i32,i32) {
        let w = 15; // TODO: determine dynamically what fits the viewport
        let h = paddlers_shared_lib::game_mechanics::map::MAP_H as i32;
        (w,h)
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