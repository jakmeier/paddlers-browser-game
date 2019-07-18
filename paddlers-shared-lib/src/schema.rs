#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    attacks (id) {
        id -> Int8,
        departure -> Timestamp,
        arrival -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    attacks_to_units (attack_id, unit_id) {
        attack_id -> Int8,
        unit_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    buildings (id) {
        id -> Int8,
        x -> Int4,
        y -> Int4,
        building_type -> Building_type,
        building_range -> Nullable<Float4>,
        attack_power -> Nullable<Float4>,
        attacks_per_cycle -> Nullable<Int4>,
        creation -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    resources (resource_type) {
        resource_type -> Resource_type,
        amount -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    units (id) {
        id -> Int8,
        sprite -> Varchar,
        hp -> Int8,
        speed -> Float4,
    }
}

joinable!(attacks_to_units -> attacks (attack_id));
joinable!(attacks_to_units -> units (unit_id));

allow_tables_to_appear_in_same_query!(
    attacks,
    attacks_to_units,
    buildings,
    resources,
    units,
);
