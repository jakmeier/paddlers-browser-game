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
