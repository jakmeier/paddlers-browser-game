mod visitor_maintenance;

pub use visitor_maintenance::*;

use crate::game::town::town_defence::AttackingHobo;
use crate::game::{
    components::NetObj,
    fight::Health,
    input::Clickable,
    movement::{Moving, Position, TargetPosition},
    status_effects::StatusEffects,
    visits::attacks::Attack,
};
use crate::gui::ui_state::Now;
use crate::gui::{render::Renderable, sprites::*, utils::*, z::Z_VISITOR};
use crate::net::graphql::query_types::HoboEffect;
use crate::prelude::*;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::graphql_types::*;
use quicksilver::geom::Vector;
use specs::prelude::*;

const ATTACKER_SIZE_FACTOR_X: f32 = 0.6;
const ATTACKER_SIZE_FACTOR_Y: f32 = 0.4;

#[derive(Default, Debug, Component)]
#[storage(HashMapStorage)]
/// A visitor is an attacking hobo
pub struct Visitor {
    pub hurried: bool,
    pub speed: f32,
    pub arrival: Timestamp,
    pub rank_offset: usize,
}

#[cfg(feature = "dev_view")]
pub fn insert_duck(
    world: &mut World,
    pos: impl Into<Vector>,
    color: UnitColor,
    birth_time: Timestamp,
    speed: impl Into<Vector>,
    hp: i64,
    ul: f32,
    netid: i64,
    effects: &[HoboEffect],
    final_pos: Option<Vector>,
) -> PadlResult<Entity> {
    let mut builder = world.create_entity();
    let pos = pos.into();
    let speed: Vector = speed.into();
    builder = builder.with(Moving::new(birth_time, pos, speed, speed.len()));
    if let Some(pos) = final_pos {
        builder = builder.with(TargetPosition::new(pos));
    }

    build_new_duck_entity(
        builder,
        pos,
        color,
        birth_time,
        speed.len(),
        Health::new_full_health(hp),
        ul,
        netid,
        effects,
        false,
        0,
    )
    .map(specs::EntityBuilder::build)
}

pub fn build_new_duck_entity<'a>(
    builder: specs::EntityBuilder<'a>,
    pos: impl Into<Vector>,
    color: UnitColor,
    arrival: Timestamp,
    speed: f32,
    hp: Health,
    ul: f32,
    netid: i64,
    effects: &[HoboEffect],
    hurried: bool,
    rank_offset: usize,
) -> PadlResult<specs::EntityBuilder<'a>> {
    let size: Vector = Vector::new(ATTACKER_SIZE_FACTOR_X * ul, ATTACKER_SIZE_FACTOR_Y * ul).into();
    let status_effects = StatusEffects::from_gql_query(effects)?;
    let mut renderable = Renderable::new(hobo_sprite_sad(color));
    if hp.hp == 0 {
        change_duck_sprite_to_happy(&mut renderable);
    }
    let pos = pos.into();
    let builder = builder
        .with(Position::new(pos, size, Z_VISITOR))
        .with(renderable)
        .with(Clickable)
        .with(status_effects)
        .with(NetObj::hobo(netid))
        .with(Visitor {
            hurried,
            speed,
            arrival,
            rank_offset,
        })
        .with(hp);

    Ok(builder)
}

use crate::net::graphql::attacks_query::AttacksQueryVillageAttacks;
impl AttacksQueryVillageAttacks {
    pub(crate) fn create_entities<'a, 'b>(
        self,
        game: &mut Game<'a, 'b>,
    ) -> PadlResult<Vec<Entity>> {
        let ul = game.world.fetch::<ScreenResolution>().unit_length();
        let birth_time = GqlTimestamp::from_string(&self.arrival).unwrap().into();
        let now = game.world.fetch::<Now>().0;

        let description = self
            .attacker
            .as_ref()
            .map(|a| &a.display_name)
            .map(|player| format!("From {}", player))
            .unwrap_or("Anarchists".to_owned());
        let size = self.units.len() as u32;
        let atk = Attack::new(birth_time, description, size);

        let mut out = vec![];
        for (i, unit) in self.units.into_iter().enumerate() {
            let unit_rep = AttackingHobo { unit, attack: &atk };
            let effects = game.touched_auras(&unit_rep, now);
            let builder = unit_rep.create_entity(
                game.town_world_mut().create_entity(),
                now,
                birth_time,
                i,
                ul,
                effects,
            )?;
            out.push(builder.build());
        }

        game.world.create_entity().with(atk).build();

        Ok(out)
    }
}
impl<'a> AttackingHobo<'a> {
    fn create_entity(
        &self,
        mut builder: specs::EntityBuilder<'a>,
        now: Timestamp,
        birth: Timestamp,
        pos_rank: usize,
        ul: f32,
        auras: Vec<(<Game<'_, '_> as IDefendingTown>::AuraId, i32)>,
    ) -> PadlResult<specs::EntityBuilder<'a>> {
        let v = self.unit.hobo.speed as f32 * ul;
        let w = TOWN_X as f32 * ul;
        let x = w - ul * ATTACKER_SIZE_FACTOR_X;
        let y = TOWN_LANE_Y as f32 * ul;
        let mut pos = Vector::new(x, y) + attacker_position_rank_offset(pos_rank, ul);
        let mut t0 = birth;
        let hp = self.unit.hobo.hp;
        let netid = self.unit.hobo.id.parse().expect("Parsing id");
        let color = self
            .unit
            .hobo
            .color
            .as_ref()
            .map(|c| c.into())
            .unwrap_or(UnitColor::Yellow);
        let time_until_resting = self.time_until_resting();

        // Simulate all interactions with buildings for the visitor which happened in the past
        let dmg = <Game<'_, '_> as IDefendingTown>::damage(&auras) + self.effects_strength();
        let hp_left = (hp - dmg as i64).max(0);
        let aura_ids = auras.into_iter().map(|a| a.0).collect();
        let health = Health::new(hp, hp_left, aura_ids);

        // Adapt position for units that have been resting and were then released
        if let Some(released) = &self.unit.info.released {
            let released = GqlTimestamp::from_string(released).unwrap().into();
            if released > birth + time_until_resting {
                pos.x = TOWN_RESTING_X as f32 * ul;
                t0 = released;
            }
        }

        // Insert components for movement (unless visitor is currently resting)
        let can_rest = !self.unit.hobo.hurried && self.unit.info.released.is_none();
        let resting = can_rest && birth + time_until_resting <= now;
        if !resting {
            builder = builder.with(Moving::new(t0, pos, Vector::new(-v, 0.0), v));
            if can_rest {
                let final_pos = Vector::new(TOWN_RESTING_X as f32 * ul, pos.y);
                builder = builder.with(TargetPosition::new(final_pos));
            }
        } else {
            pos.x = TOWN_RESTING_X as f32 * ul;
        }

        build_new_duck_entity(
            builder,
            pos,
            color,
            birth,
            v,
            health,
            ul,
            netid,
            &self.unit.hobo.effects,
            self.unit.hobo.hurried,
            pos_rank,
        )
    }
}

fn attacker_position_rank_offset(pr: usize, ul: f32) -> Vector {
    let y = if pr % 2 == 1 { ul * 0.5 } else { 0.0 };
    let x = ul * 0.3 * pr as f32;
    (x, y).into()
}

fn hobo_sprite_sad(color: UnitColor) -> RenderVariant {
    let sprite_index = match color {
        UnitColor::Yellow => SingleSprite::Duck,
        UnitColor::White => SingleSprite::WhiteDuck,
        UnitColor::Camo => SingleSprite::CamoDuck,
        UnitColor::Prophet => SingleSprite::Prophet,
    };
    RenderVariant::ImgWithImgBackground(SpriteSet::Simple(sprite_index), SingleSprite::Water)
}
