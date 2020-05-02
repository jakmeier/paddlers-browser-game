use crate::game::{
    components::NetObj,
    fight::Health,
    input::Clickable,
    movement::{Moving, Position},
    status_effects::StatusEffects,
    visits::attacks::Attack,
};
use crate::gui::{render::Renderable, sprites::*, utils::*, z::Z_VISITOR};
use crate::net::graphql::query_types::HoboEffect;
use crate::prelude::*;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::graphql_types::*;
use quicksilver::geom::Vector;
use specs::prelude::*;

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
    let entity = world
        .create_entity()
        .with(Position::new(pos, size, Z_VISITOR))
        .with(Moving::new(birth_time, pos, speed, speed.len()))
        .with(Renderable::new(hobo_sprite_sad(color)))
        .with(Clickable)
        .with(status_effects)
        .with(NetObj::hobo(netid))
        .with(Health::new_full_health(hp))
        .build();
    Ok(entity)
}

pub fn change_duck_sprite_to_happy(r: &mut Renderable) {
    match r.kind {
        RenderVariant::ImgWithImgBackground(SpriteSet::Simple(ref mut img), _bkg) => match img {
            SingleSprite::Duck => {
                *img = SingleSprite::DuckHappy;
            }
            SingleSprite::CamoDuck => {
                *img = SingleSprite::CamoDuckHappy;
            }
            SingleSprite::WhiteDuck => {
                *img = SingleSprite::WhiteDuckHappy;
            }
            _ => {}
        },
        _ => {}
    }
}

use crate::net::graphql::attacks_query::{
    AttacksQueryVillageAttacks, AttacksQueryVillageAttacksUnits,
};
impl AttacksQueryVillageAttacks {
    pub fn create_entities(&self, world: &mut World) -> PadlResult<Vec<Entity>> {
        let ul = world.fetch::<ScreenResolution>().unit_length();
        let birth_time = GqlTimestamp::from_string(&self.arrival).unwrap().0;

        let description = self
            .attacker
            .as_ref()
            .map(|a| &a.display_name)
            .map(|player| format!("From {}", player))
            .unwrap_or("Anarchists".to_owned());
        let size = self.units.len() as u32;
        let atk = Attack::new(birth_time, description, size);
        world.create_entity().with(atk).build();

        self.units
            .iter()
            .enumerate()
            .map(|(i, u)| u.create_entity(world, birth_time, i, ul))
            .collect()
    }
}
impl AttacksQueryVillageAttacksUnits {
    // TODO [IMPORTANT]: For entities already well into the map, compute the attacks so far.
    fn create_entity(
        &self,
        world: &mut World,
        birth: Timestamp,
        pos_rank: usize,
        ul: f32,
    ) -> PadlResult<Entity> {
        let v = -self.speed as f32 * ul;
        let w = TOWN_X as f32 * ul;
        let x = w - ul * ATTACKER_SIZE_FACTOR_X;
        let y = TOWN_LANE_Y as f32 * ul;
        let pos = Vector::new(x, y) + attacker_position_rank_offset(pos_rank, ul);
        let hp = self.hp;
        let netid = self.id.parse().expect("Parsing id");
        let color = self
            .color
            .as_ref()
            .map(|c| c.into())
            .unwrap_or(UnitColor::Yellow);
        insert_duck(
            world,
            pos,
            color,
            birth,
            (v as f32, 0.0),
            hp,
            ul,
            netid,
            &self.effects,
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
