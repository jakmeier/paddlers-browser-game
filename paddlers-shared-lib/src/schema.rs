table! {
    use diesel::sql_types::*;
    use crate::models::*;

    abilities (ability_type, worker_id) {
        ability_type -> Ability_type,
        worker_id -> Int8,
        last_used -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    attacks (id) {
        id -> Int8,
        departure -> Timestamp,
        arrival -> Timestamp,
        origin_village_id -> Nullable<Int8>,
        destination_village_id -> Int8,
        entered_destination -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    attacks_to_hobos (attack_id, hobo_id) {
        attack_id -> Int8,
        hobo_id -> Int8,
        satisfied -> Nullable<Bool>,
        released -> Nullable<Timestamp>,
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
        attack_power -> Nullable<Int4>,
        attacks_per_cycle -> Nullable<Int4>,
        creation -> Timestamp,
        village_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    effects (id) {
        id -> Int8,
        hobo_id -> Int8,
        attribute -> Hobo_attribute_type,
        strength -> Nullable<Int4>,
        start_time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    hobos (id) {
        id -> Int8,
        home -> Int8,
        color -> Nullable<Unit_color>,
        speed -> Float4,
        hp -> Int8,
        hurried -> Bool,
        nest -> Nullable<Int8>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    players (id) {
        id -> Int8,
        uuid -> Uuid,
        karma -> Int8,
        display_name -> Varchar,
        story_state -> Story_state_type,
        civ_perks -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    quest_building_conditions (id) {
        id -> Int8,
        quest_id -> Int8,
        building_type -> Building_type,
        amount -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    quest_res_conditions (id) {
        id -> Int8,
        quest_id -> Int8,
        resource_type -> Resource_type,
        amount -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    quest_res_rewards (id) {
        id -> Int8,
        quest_id -> Int8,
        resource_type -> Resource_type,
        amount -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    quest_to_player (quest_id, player_id) {
        quest_id -> Int8,
        player_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    quest_worker_conditions (id) {
        id -> Int8,
        quest_id -> Int8,
        task_type -> Task_type,
        amount -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    quests (id) {
        id -> Int8,
        quest_key -> Varchar,
        karma_condition -> Nullable<Int8>,
        pop_condition -> Nullable<Int8>,
        follow_up_quest -> Nullable<Varchar>,
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

    rewards (id) {
        id -> Int8,
        visit_report_id -> Int8,
        resource_type -> Resource_type,
        amount -> Int8,
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
        worker_id -> Int8,
        task_type -> Task_type,
        x -> Int4,
        y -> Int4,
        start_time -> Timestamp,
        target_hobo_id -> Nullable<Int8>,
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
        player_id -> Nullable<Int8>,
        faith -> Int2,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    visit_reports (id) {
        id -> Int8,
        village_id -> Int8,
        reported -> Timestamp,
        karma -> Int8,
        sender -> Nullable<Int8>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    worker_flags (worker_id, flag_type) {
        worker_id -> Int8,
        flag_type -> Worker_flag_type,
        last_update -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    workers (id) {
        id -> Int8,
        home -> Int8,
        x -> Int4,
        y -> Int4,
        unit_type -> Unit_type,
        color -> Nullable<Unit_color>,
        speed -> Float4,
        mana -> Nullable<Int4>,
        level -> Int4,
        exp -> Int4,
    }
}

joinable!(abilities -> workers (worker_id));
joinable!(attacks_to_hobos -> attacks (attack_id));
joinable!(attacks_to_hobos -> hobos (hobo_id));
joinable!(buildings -> villages (village_id));
joinable!(effects -> hobos (hobo_id));
joinable!(hobos -> buildings (nest));
joinable!(hobos -> villages (home));
joinable!(quest_building_conditions -> quests (quest_id));
joinable!(quest_res_conditions -> quests (quest_id));
joinable!(quest_res_rewards -> quests (quest_id));
joinable!(quest_to_player -> players (player_id));
joinable!(quest_to_player -> quests (quest_id));
joinable!(quest_worker_conditions -> quests (quest_id));
joinable!(resources -> villages (village_id));
joinable!(rewards -> visit_reports (visit_report_id));
joinable!(tasks -> hobos (target_hobo_id));
joinable!(tasks -> workers (worker_id));
joinable!(villages -> players (player_id));
joinable!(villages -> streams (stream_id));
joinable!(visit_reports -> hobos (sender));
joinable!(visit_reports -> villages (village_id));
joinable!(worker_flags -> workers (worker_id));
joinable!(workers -> villages (home));

allow_tables_to_appear_in_same_query!(
    abilities,
    attacks,
    attacks_to_hobos,
    buildings,
    effects,
    hobos,
    players,
    quest_building_conditions,
    quest_res_conditions,
    quest_res_rewards,
    quest_to_player,
    quest_worker_conditions,
    quests,
    resources,
    rewards,
    streams,
    tasks,
    villages,
    visit_reports,
    worker_flags,
    workers,
);
