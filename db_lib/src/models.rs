use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Unit {
    pub id: i64,
    pub sprite: String,
    pub hp: i64,
    pub speed: f32,
}

#[derive(Debug, Queryable)]
pub struct Attack {
    pub id: i64,
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct AttackToUnit {
    pub attack_id: i64,
    pub unit_id: i64,
}

use super::schema::units;
#[allow(non_camel_case_types)]
pub type UNIT_ALL_COLUMNS_T =  (
    units::id,
    units::sprite,
    units::hp,
    units::speed,
);
pub const UNIT_ALL_COLUMNS: UNIT_ALL_COLUMNS_T = (
    units::id,
    units::sprite,
    units::hp,
    units::speed,
);

// use ::diesel_derive_enum;
// use diesel_derive_enum::DbEnum;
// #[derive(Debug, PartialEq, DbEnum, Clone)]
// #[allow(non_camel_case_types)]
// pub enum Building_type {
//     BlueFlowers,
//     RedFlowers,
// }

#[derive(Queryable)]
pub struct Building {
    pub id: i64,
    pub x: i32,
    pub y: i32,
    // pub building_type: db_enum_impl_Building_type::Building_typeMapping,
    pub building_range: Option<f32>, 
    pub attack_power: Option<f32>, 
    pub attacks_per_cycle: Option<i32>,
}


// use super::schema::buildings;
// #[allow(non_camel_case_types)]
// pub type BUILDING_ALL_COLUMNS_T =  (
//     buildings::id,
//     buildings::x,
//     buildings::y,
// );
// pub const BUILDING_ALL_COLUMNS: BUILDING_ALL_COLUMNS_T = (
//     buildings::id,
//     buildings::x,
//     buildings::y,
// );