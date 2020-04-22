use crate::game::{
    fight::{Health, Range},
    movement::Position,
    town::Town,
    Game,
};
use crate::gui::ui_state::ClockTick;
use crate::gui::{
    animation::AnimationState, input::Grabbable, sprites::*, ui_state::*, utils::*, z::*,
};
use crate::logging::text_to_user::TextBoard;
use crate::prelude::*;
use quicksilver::graphics::Color;
use quicksilver::input::MouseCursor;
use quicksilver::prelude::*;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderVariant,
    /// Size factor is applied when rendering in main window, not in menu
    in_game_transformation: f32,
}
impl Renderable {
    pub fn new(kind: RenderVariant) -> Self {
        Renderable {
            kind,
            in_game_transformation: std::f32::NAN,
        }
    }
    pub fn new_transformed(kind: RenderVariant, in_game_transformation: f32) -> Self {
        Renderable {
            kind,
            in_game_transformation,
        }
    }
}

impl Game<'_, '_> {
    pub fn draw_main(&mut self, window: &mut Window) -> PadlResult<()> {
        let ui_state = self.world.read_resource::<UiState>();
        let hovered_entity = ui_state.hovered_entity;
        let grabbed_item = ui_state.grabbed_item.clone();
        std::mem::drop(ui_state);

        self.render_text_messages(window)?;

        if let Some(entity) = hovered_entity {
            self.render_hovering(window, entity)?;
        }
        if let Some(grabbed) = grabbed_item {
            self.render_grabbed_item(window, &grabbed)?;
            window.set_cursor(MouseCursor::None);
        } else {
            window.set_cursor(MouseCursor::Default);
        }
        #[cfg(feature = "dev_view")]
        self.draw_dev_view(window);
        Ok(())
    }

    pub fn render_town_entities(&mut self, window: &mut Window) -> PadlResult<()> {
        let world = &self.world;
        let pos_store = world.read_storage::<Position>();
        let rend_store = world.read_storage::<Renderable>();
        let animation_store = world.read_storage::<AnimationState>();
        let sprites = &mut self.sprites;
        let entities = self.world.entities();
        let tick = self.world.read_resource::<ClockTick>();
        for (e, pos, r) in (&entities, &pos_store, &rend_store).join() {
            let mut area = pos.area;
            if r.in_game_transformation.is_normal() {
                area = area.shrink_to_center(r.in_game_transformation);
            }
            match r.kind {
                RenderVariant::ImgWithImgBackground(i, _) => {
                    if let Some(animation) = animation_store.get(e) {
                        draw_animated_sprite(
                            sprites,
                            window,
                            &area,
                            i,
                            pos.z,
                            FitStrategy::TopLeft,
                            animation,
                            tick.0,
                        )?;
                    } else {
                        draw_static_image(
                            sprites,
                            window,
                            &area,
                            i.default(),
                            pos.z,
                            FitStrategy::TopLeft,
                        )?;
                    }
                }
                _ => panic!("Not implemented"),
            }
        }
        Ok(())
    }

    pub fn render_hovering(&mut self, window: &mut Window, entity: Entity) -> PadlResult<()> {
        let position_store = self.world.read_storage::<Position>();
        let range_store = self.world.read_storage::<Range>();
        let health_store = self.world.read_storage::<Health>();

        if let Some((range, p)) = (&range_store, &position_store)
            .join()
            .get(entity, &self.world.entities())
        {
            range.draw(window, &self.town(), &p.area)?;
        }

        if let Some((health, p)) = (&health_store, &position_store)
            .join()
            .get(entity, &self.world.entities())
        {
            render_health(&health, &mut self.sprites, window, &p.area)?;
        }
        Ok(())
    }

    pub fn render_grabbed_item(&mut self, window: &mut Window, item: &Grabbable) -> PadlResult<()> {
        let mouse = window.mouse().pos();
        let ul = self.world.fetch::<ScreenResolution>().unit_length();
        let center = mouse - (ul / 2.0, ul / 2.0).into();
        let max_area = Rectangle::new(center, (ul, ul));
        match item {
            Grabbable::NewBuilding(building_type) => draw_static_image(
                &mut self.sprites,
                window,
                &max_area,
                building_type.sprite().default(),
                Z_GRABBED_ITEM,
                FitStrategy::TopLeft,
            )?,
            Grabbable::Ability(ability) => draw_static_image(
                &mut self.sprites,
                window,
                &max_area.shrink_to_center(0.375),
                ability.sprite().default(),
                Z_GRABBED_ITEM,
                FitStrategy::TopLeft,
            )?,
        }
        Ok(())
    }

    pub fn render_text_messages(&mut self, window: &mut Window) -> PadlResult<()> {
        let screen = window.project() * window.screen_size();
        let w = 300.0;
        let h = screen.y;
        let x = (screen.x - w) / 2.0;
        let y = 0.0;
        let area = Rectangle::new((x, y), (w, h));
        let mut tb = self.world.write_resource::<TextBoard>();
        tb.draw(&area)?;
        Ok(())
    }
}

fn render_health(
    health: &Health,
    sprites: &mut Sprites,
    window: &mut Window,
    area: &Rectangle,
) -> PadlResult<()> {
    let (max, hp) = (health.max_hp, health.hp);
    let unit_pos = area.pos;
    let w = area.width();
    let h = 10.0;
    let max_area = Rectangle::new((unit_pos.x, unit_pos.y - h), (w, h));

    match hp {
        0 => {
            let h = 20.0;
            let max_area = Rectangle::new((unit_pos.x, unit_pos.y - h), (w, h));
            draw_static_image(
                sprites,
                window,
                &max_area,
                SpriteIndex::Simple(SingleSprite::Heart),
                Z_HP_BAR,
                FitStrategy::Center,
            )?;
        }
        hp if hp < 10 => {
            let d = w / hp as f32;
            let mut hp_block = max_area.clone();
            hp_block.size.x = d * 0.9;
            for _ in 0..hp as usize {
                draw_rect(window, &hp_block, GREY);
                hp_block.pos.x += d;
            }
        }
        hp if hp < 50 => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max - hp) as f32 / max as f32;
            draw_rect(window, &max_area, GREY);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        }
        _ => {
            let mut lost_hp_area = max_area.clone();
            lost_hp_area.size.x *= (max - hp) as f32 / max as f32;
            draw_rect(window, &max_area, BLACK);
            draw_rect_z(window, &lost_hp_area, GREEN, 1);
        }
    }

    Ok(())
}

impl Range {
    fn draw(&self, window: &mut Window, town: &Town, area: &Rectangle) -> PadlResult<()> {
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
    window.draw_ex(area, Col(col), Transform::IDENTITY, Z_HP_BAR + z_shift);
}
