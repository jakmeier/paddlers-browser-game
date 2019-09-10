mod map_tesselation;
mod village_meta;

use quicksilver::prelude::*;
use quicksilver::graphics::{Mesh};
use crate::gui::{
    sprites::*,
    utils::*,
    z::*,
};
pub use village_meta::VillageMetaInfo;

pub struct MapSkeleton {
    w: u32,
    h: u32,
    streams: Vec<Vec<(f32,f32)>>,
}

pub struct GlobalMap {
    water_mesh: Mesh,
    grid_mesh: Mesh,
    skeleton: MapSkeleton,
    villages: Vec<VillageMetaInfo>,
    scaling: f32,
}

impl GlobalMap {
    pub fn new_test() -> Self {
        let mut skeleton = MapSkeleton::static_test_map();
        let base_shape = skeleton.base_shape();
        let water_mesh = skeleton.tesselate_rivers(&base_shape);
        let grid_mesh = skeleton.tesselate_background();
        let test_villages : Vec<(usize,usize)> = vec![(1,3),(2,1),(2,5),(5,5),(6,2),(6,4),(7,3),(8,7),(9,7),(10,8),(10,9),(12,9),(13,10)];
        let test_villages = test_villages.into_iter().map(|coordinates| VillageMetaInfo{ coordinates }).collect();
        GlobalMap {
            water_mesh,
            grid_mesh,
            skeleton,
            villages: test_villages,
            scaling: 1.0,
        }
    }
    pub fn new(streams: Vec<Vec<(f32,f32)>>, villages: Vec<VillageMetaInfo> ) -> Self {
        let mut skeleton = MapSkeleton {
            w: 15,
            h: paddlers_shared_lib::game_mechanics::map::MAP_H,
            streams,
        };
        let base_shape = skeleton.base_shape();
        let water_mesh = skeleton.tesselate_rivers(&base_shape);
        let grid_mesh = skeleton.tesselate_background();
        GlobalMap {
            water_mesh,
            grid_mesh,
            skeleton,
            villages: villages,
            scaling: 1.0,
        }
    }
    pub fn render(&mut self, window: &mut Window, sprites: &mut Asset<Sprites>, area: &Rectangle) -> Result<()> {
        self.apply_scaling(area);
        window.mesh().extend(&self.grid_mesh);
        window.flush()?;
        window.mesh().extend(&self.water_mesh);
        self.draw_villages(window, sprites)?;
        Ok(())
    }
    fn apply_scaling(&mut self, area: &Rectangle) {
        let base_shape = self.skeleton.base_shape();
        let rx = area.width() / base_shape.width();
        let ry = area.height() / base_shape.height();
        let r = rx.min(ry);
        if self.scaling != r {
            scale_mesh(&mut self.grid_mesh, r / self.scaling);
            self.water_mesh = self.skeleton.tesselate_rivers(&area);
            self.scaling = r;
        }
    }
    pub fn draw_villages(&mut self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
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
}