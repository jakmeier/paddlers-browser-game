use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::prelude::*;
use crate::gui::{
    render::Renderable,
    z::Z_VISITOR,
    sprites::*,
    utils::*,
};
use crate::game::{
    input::Clickable,
    movement::{Position, Moving},
    fight::Health,
    components::NetObj,
    status_effects::StatusEffects,
};
use crate::net::graphql::query_types::HoboEffect;
use paddlers_shared_lib::graphql_types::*;
use paddlers_shared_lib::game_mechanics::town::*;

const ATTACKER_SIZE_FACTOR_X: f32 = 0.6; 
const ATTACKER_SIZE_FACTOR_Y: f32 = 0.4; 

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
) -> PadlResult<Entity> {
    let pos = pos.into();
    let speed = speed.into();
    let size: Vector = Vector::new(ATTACKER_SIZE_FACTOR_X * ul, ATTACKER_SIZE_FACTOR_Y * ul).into();
    let status_effects = StatusEffects::from_gql_query(effects)?;
    let entity = world.create_entity()
        .with(Position::new(pos, size, Z_VISITOR))
        .with(Moving::new(birth_time, pos, speed, speed.len()))
        .with(
            Renderable::new (
                hobo_sprite_sad(color)
            )
        )
        .with(Clickable)
        .with(status_effects)
        .with(NetObj::hobo(netid))
        .with(Health::new_full_health(hp))
        .build();
    Ok(entity)
}

pub fn change_duck_sprite_to_happy(r: &mut Renderable) {
    match r.kind {
        RenderVariant::ImgWithImgBackground(SpriteSet::Simple(ref mut img), _bkg)
        => {
            match img {
                SingleSprite::Duck => {
                    *img = SingleSprite::DuckHappy;
                }
                SingleSprite::CamoDuck => {
                    *img = SingleSprite::CamoDuckHappy;
                }
                SingleSprite::WhiteDuck => {
                    *img = SingleSprite::WhiteDuckHappy;
                }
                _ => { }
            }
        },
        _ => { }
    }
}

use crate::net::graphql::attacks_query::{AttacksQueryVillageAttacksUnits,AttacksQueryVillageAttacks};
impl AttacksQueryVillageAttacks {
    pub fn create_entities(&self, world: &mut World, ul: f32) -> PadlResult<Vec<Entity>> {
        let birth_time = GqlTimestamp::from_string(&self.arrival).unwrap().0;
        self.units
            .iter()
            .enumerate()
            .map(|(i, u)|{u.create_entity(world, birth_time, i, ul)})
            .collect()
    }
}
impl AttacksQueryVillageAttacksUnits {
    // TODO [IMPORTANT]: For entities already well into the map, compute the attacks so far.
    fn create_entity(&self, world: &mut World, birth: Timestamp, pos_rank: usize, ul: f32) -> PadlResult<Entity> {
        let v = -self.speed as f32 * ul;
        let w = TOWN_X as f32 * ul;
        let x = w - ul * ATTACKER_SIZE_FACTOR_X;
        let y = TOWN_LANE_Y as f32 * ul;
        let pos = Vector::new(x,y) + attacker_position_rank_offset(pos_rank, ul);
        let hp = self.hp;
        let netid = self.id.parse().expect("Parsing id");
        let color = self.color.as_ref().map(|c|c.into()).unwrap_or(UnitColor::Yellow);
        insert_duck(world, pos, color, birth, (v as f32, 0.0), hp, ul, netid, &self.effects)
    }
}

fn attacker_position_rank_offset(pr: usize, ul: f32) -> Vector {
    let y = if pr % 2 == 1 { ul * 0.5 } else { 0.0 };
    let x = ul * 0.3 * pr as f32;
    (x,y).into()
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