use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable)]
pub struct Unit {
    pub id: i64,
    pub sprite: String,
    pub hp: i64,
    pub speed: f32,
}
#[derive(Insertable)]
#[table_name = "units"]
pub struct NewUnit<'a> {
    pub sprite: &'a str,
    pub hp: i64,
    pub speed: f32,
}

use super::schema::attacks;
#[derive(Debug, Queryable, Identifiable)]
pub struct Attack {
    pub id: i64,
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}
#[derive(Insertable)]
#[table_name = "attacks"]
pub struct NewAttack {
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}

use super::schema::attacks_to_units;
#[derive(Debug, Queryable,Insertable)]
#[table_name = "attacks_to_units"]
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

#[derive(Queryable, Debug)]
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