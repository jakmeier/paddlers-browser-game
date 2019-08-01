use quicksilver::prelude::*;
use quicksilver::graphics::Color;
use specs::prelude::*;
use crate::game::{
    Game,
    movement::Position,
    fight::{Health, Range},
    town::Town,
};
use crate::gui::{
    sprites::{SpriteIndex, Sprites},
    z::*,
    utils::*,
    animation::AnimationState,
};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderVariant,
}

impl Game<'_, '_> {
    pub fn render_entities(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        let pos_store = world.read_storage::<Position>();
        let rend_store = world.read_storage::<Renderable>();
        let animation_store = world.read_storage::<AnimationState>();
        let sprites = &mut self.sprites;
        let entities = self.world.entities();
        let now = crate::wasm_setup::utc_now();
        for (e, pos, r) in (&entities, &pos_store, &rend_store).join() {
            match r.kind {
                RenderVariant::ImgWithImgBackground(i, _) => {
                    if let Some(animation) = animation_store.get(e) {
                        draw_animated_sprite(sprites, window, &pos.area, i, pos.z, FitStrategy::TopLeft, animation)?;
                    } else {
                        draw_static_image(sprites, window, &pos.area, i, pos.z, FitStrategy::TopLeft)?;
                    }
                },
                RenderVariant::DynImgWithImgBackground(di, _) => {
                    let i = di.sprite(now);
                    if let Some(animation) = animation_store.get(e) {
                        draw_animated_sprite(sprites, window, &pos.area, i, pos.z, FitStrategy::TopLeft, animation)?;
                    } else {
                        draw_static_image(sprites, window, &pos.area, i, pos.z, FitStrategy::TopLeft)?;
                    }
                }
                _ => { panic!("Not implemented")}
            }
        }
        Ok(())
    }

    pub fn render_hovering(&mut self, window: &mut Window, entity: Entity) -> Result<()> {
        let position_store = self.world.read_storage::<Position>();
        let range_store = self.world.read_storage::<Range>();
        let health_store = self.world.read_storage::<Health>();

        if let Some((range,p)) = (&range_store, &position_store).join().get(entity, &self.world.entities()) {
            range.draw(window, &self.town(), &p.area)?;
        }

        if let Some((health,p)) = (&health_store, &position_store).join().get(entity, &self.world.entities()) {
            render_health(&health, &mut self.sprites, window, &p.area)?;
        }
        Ok(())
    }

}

fn render_health(health: &Health, sprites: &mut Asset<Sprites>, window: &mut Window, area: &Rectangle) -> Result<()> {
    let (max, hp) = (health.max_hp, health.hp);
    let unit_pos = area.pos;
    let w = area.width();
    let h = 10.0;
    let max_area = Rectangle::new((unit_pos.x,unit_pos.y-h),(w,h));

    match hp {
        0 => {
            let h = 20.0;
            let max_area = Rectangle::new((unit_pos.x,unit_pos.y-h),(w,h));
            draw_static_image(sprites, window, &max_area, SpriteIndex::Heart, Z_HP_BAR, FitStrategy::Center)?;
        }
        hp if hp < 10 => {
            let d = w / hp as f32;
            let mut hp_block = max_area.clone();
            hp_block.size.x = d * 0.9;
            for _ in 0..hp as usize {
                draw_rect(window, &hp_block, GREY);
                hp_block.pos.x += d;
            }
        },
        hp if hp < 50 => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max-hp) as f32 / max as f32;
            draw_rect(window, &max_area, GREY);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        },
        _ => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max-hp) as f32 / max as f32;
            draw_rect(window, &max_area, BLACK);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        }
    }

    Ok(())
}

impl Range {
    fn draw(&self, window: &mut Window, town: &Town, area: &Rectangle) -> Result<()> {
        // TODO Check if this aligns 100% with server. Also consider changing interface to TileIndex instead of center
        town.shadow_rectified_circle(window, area.center(), self.range);
        Ok(())
    }
}
#[inline]
fn draw_rect(window: &mut Window, area: &Rectangle, col: Color) {
    draw_rect_z(window, area, col, 0);
}
#[inline]
fn draw_rect_z(window: &mut Window, area: &Rectangle, col: Color, z_shift: i32) {
    window.draw_ex(
        area,
        Col(col),
        Transform::IDENTITY, 
        Z_HP_BAR + z_shift,
    );
}