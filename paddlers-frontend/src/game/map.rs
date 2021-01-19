mod map_frame;
mod map_position;
mod map_segment;
mod map_tesselation;
mod village_meta;

use crate::net::authentication::keycloak_preferred_name;
use crate::{
    gui::{input::Clickable, render::Renderable, sprites::*, ui_state::*, utils::*, z::*},
    resolution::MAIN_AREA_H,
};
use map_position::*;
use map_segment::MapSegment;
use map_tesselation::*;
use paddle::*;
use paddlers_shared_lib::game_mechanics::map::MAP_H;
use specs::prelude::*;

pub(crate) use map_frame::MapFrame;
pub use map_position::MapPosition;
pub use village_meta::VillageMetaInfo;

/// Helper struct to combine private and shared map state
pub struct GlobalMap<'a> {
    /// State that is not shareable between threads
    /// It is only accessible in the central game loop.
    private: &'a mut GlobalMapPrivateState,
    /// State than can be shared with threads safely.
    /// It is used in specs systems.
    shared: specs::shred::FetchMut<'a, GlobalMapSharedState>,
}

pub struct GlobalMapPrivateState {
    grid_mesh: AbstractMesh,
    segments: Vec<MapSegment>,
    villages: Vec<VillageMetaInfo>,
    view_width: i32,
    loaded: (i32, i32),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalMapSharedState {
    /// Offset in map coordinates (1.0 = one village width)
    x_offset: f32,
}

impl<'a> GlobalMap<'a> {
    pub fn combined<'b>(
        private: &'b mut GlobalMapPrivateState,
        shared: specs::shred::FetchMut<'b, GlobalMapSharedState>,
    ) -> GlobalMap<'b> {
        GlobalMap::<'b> { private, shared }
    }
    pub fn new() -> (GlobalMapPrivateState, GlobalMapSharedState) {
        let (w, h) = Self::display_shape();
        let grid_mesh = tesselate_grid_net(w, h);

        let map = GlobalMapPrivateState {
            grid_mesh,
            segments: vec![],
            villages: vec![],
            view_width: w,
            loaded: (0, -1),
        };
        let shared = GlobalMapSharedState { x_offset: 0.0 };
        (map, shared)
    }

    const LOAD_AHEAD: i32 = 10;
    const LOAD_STEP: i32 = 10;
    pub fn update(&mut self) {
        let x = -self.shared.x_offset as i32;
        if self.private.loaded.0 > x - Self::LOAD_AHEAD {
            let (low, high) = (
                self.private.loaded.0 - 1 - Self::LOAD_STEP,
                self.private.loaded.0 - 1,
            );
            crate::net::request_map_read(low, high);

            self.private.loaded.0 = self.private.loaded.0.min(low);
            self.private.loaded.1 = self.private.loaded.1.max(high);
        }
        if self.private.loaded.1 < x + self.private.view_width + Self::LOAD_AHEAD {
            let (low, high) = (
                self.private.loaded.1 + 1,
                self.private.loaded.1 + 1 + Self::LOAD_STEP,
            );
            crate::net::request_map_read(low, high);

            self.private.loaded.0 = self.private.loaded.0.min(low);
            self.private.loaded.1 = self.private.loaded.1.max(high);
        }
    }
    fn draw_grid(&mut self, window: &mut DisplayArea) {
        let mut x = self.shared.x_offset % 1.0;
        if x > 0.0 {
            x -= 1.0
        }
        let t = Transform::translate((x * Self::unit_length(), 0));
        window.draw_mesh_ex(
            &self.private.grid_mesh,
            Rectangle::new_sized(window.size() + Vector::X * Self::unit_length()),
            TRANSPARENT_BLACK,
            t,
            Z_GRID,
        );
    }
    fn draw_water(&mut self, window: &mut DisplayArea, area: &Rectangle) {
        let visible_frame = Rectangle::new(
            (-self.shared.x_offset, 0),
            area.size() / Self::unit_length(),
        );
        let t = self.view_transform();
        for segment in self.private.segments.iter_mut() {
            if segment.is_visible(visible_frame) {
                window.draw_mesh_ex(
                    &segment.water_mesh,
                    segment.scaled_base_shape(),
                    BLUE,
                    t,
                    Z_RIVER,
                );
            }
        }
    }
    fn draw_villages(&mut self, window: &mut DisplayArea, sprites: &mut Sprites) {
        #[cfg(feature = "dev_view")]
        self.visualize_control_points(window);

        for vil in &self.private.villages {
            let (x, y) = vil.coordinates;
            // translate human-readable to nerd indexing
            let (x, y) = (x - 1, y - 1);
            let sprite_area = Rectangle::new(
                (
                    (x as f32 + self.shared.x_offset) * Self::unit_length(),
                    y as f32 * Self::unit_length(),
                ),
                (Self::unit_length(), Self::unit_length()),
            );
            draw_image(
                sprites,
                window,
                &sprite_area,
                SpriteIndex::Simple(SingleSprite::Shack),
                Z_BUILDINGS,
                FitStrategy::Center,
                Transform::IDENTITY,
            );
        }
    }
    const fn display_shape() -> (i32, i32) {
        let w = 15;
        let h = MAP_H as i32;
        (w, h)
    }
    const fn unit_length() -> f32 {
        MAIN_AREA_H as f32 / MAP_H as f32
    }
    fn view_offset(&self) -> Vector {
        Vector::new(self.shared.x_offset * Self::unit_length(), 0)
    }
    fn view_transform(&self) -> Transform {
        Transform::translate(self.view_offset())
    }

    #[cfg(feature = "dev_view")]
    fn visualize_control_points(&self, window: &mut DisplayArea) {
        let pt = self.shared.scaling / 5.0;
        for seg in &self.private.segments {
            for s in &seg.streams {
                for (x, y) in s {
                    let area = Rectangle::new(
                        (
                            (self.shared.x_offset + x) * self.shared.scaling - pt / 2.0,
                            y * self.shared.scaling - pt / 2.0,
                        ),
                        (pt, pt),
                    );
                    window.draw_ex(&area, WHITE, Transform::rotate(45), 1000);
                }
            }
        }
    }
}

impl GlobalMapPrivateState {
    pub fn add_segment(
        &mut self,
        world: &mut World,
        streams: Vec<Vec<(f32, f32)>>,
        villages: Vec<VillageMetaInfo>,
        min_x: i32,
        max_x: i32,
    ) {
        let w = max_x - min_x;
        let h = MAP_H as i32;
        let mut segment = MapSegment::new(min_x, 0, w, h, streams);
        segment.tesselate_rivers();
        self.segments.push(segment);

        let my_name = keycloak_preferred_name().unwrap();
        for village in villages.iter() {
            let owner_name = village.player_name();
            let is_mine = owner_name.is_some() && owner_name.unwrap() == my_name; // TODO: Better check not relying on unique display names
            world
                .create_entity()
                .with(MapPosition::new(village.coordinates))
                .with(Renderable::new(RenderVariant::ImgWithColBackground(
                    SpriteSet::Simple(SingleSprite::Shack),
                    GREEN,
                )))
                .with(Clickable)
                .with((*village).clone())
                .with(village.new_village_menu(is_mine))
                .build();
        }

        self.villages.extend(villages.into_iter());
    }
}

impl GlobalMapSharedState {
    pub fn drag(&mut self, v: Vector) {
        self.x_offset += v.x;
    }
    pub fn left_click_on_main_area<'a>(
        &mut self,
        mouse_pos: Vector,
        ui_state: &'a mut UiState,
        entities: Entities<'a>,
        position: ReadStorage<'a, MapPosition>,
        clickable: ReadStorage<'a, Clickable>,
    ) {
        let r = GlobalMap::unit_length();
        let map_coordinates = Vector::new(mouse_pos.x / r - self.x_offset, mouse_pos.y / r);

        ui_state.selected_entity =
            map_position_lookup(map_coordinates, entities, position, clickable);
    }
}
