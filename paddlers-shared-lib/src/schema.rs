#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    attacks (id) {
        id -> Int8,
        departure -> Timestamp,
        arrival -> Timestamp,
        origin_village_id -> Int8,
        destination_village_id -> Int8,
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
        village_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    resources (resource_type, village_id) {
        resource_type -> Resource_type,
        amount -> Int8,
        village_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    streams (id) {
        id -> Int8,
        start_x -> Float4,
        control_points -> Array<Float4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    tasks (id) {
        id -> Int8,
        unit_id -> Int8,
        task_type -> Task_type,
        x -> Int4,
        y -> Int4,
        start_time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    units (id) {
        id -> Int8,
        home -> Int8,
        x -> Int4,
        y -> Int4,
        unit_type -> Unit_type,
        color -> Nullable<Unit_color>,
        hp -> Int8,
        speed -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    villages (id) {
        id -> Int8,
        x -> Float4,
        y -> Float4,
        stream_id -> Int8,
    }
}

joinable!(attacks_to_units -> attacks (attack_id));
joinable!(attacks_to_units -> units (unit_id));
joinable!(buildings -> villages (village_id));
joinable!(resources -> villages (village_id));
joinable!(tasks -> units (unit_id));
joinable!(units -> villages (home));
joinable!(villages -> streams (stream_id));

allow_tables_to_appear_in_same_query!(
    attacks,
    attacks_to_units,
    buildings,
    resources,
    streams,
    tasks,
    units,
    villages,
);
