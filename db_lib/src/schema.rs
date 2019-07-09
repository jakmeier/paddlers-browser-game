table! {
    attacks (id) {
        id -> Int8,
        departure -> Timestamp,
        arrival -> Timestamp,
    }
}

table! {
    attacks_to_units (attack_id, unit_id) {
        attack_id -> Int8,
        unit_id -> Int8,
    }
}

table! {
    buildings (id) {
        id -> Int8,
        x -> Int4,
        y -> Int4,
        building_range -> Nullable<Float4>,
        attack_power -> Nullable<Float4>,
        attacks_per_cycle -> Nullable<Int4>,
    }
}

table! {
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
    units,
);
