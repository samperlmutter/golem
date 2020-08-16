table! {
    brothers (slack_id) {
        slack_id -> Varchar,
        can_strike -> Bool,
        can_reset -> Bool,
        name -> Varchar,
    }
}

table! {
    points (id) {
        id -> Integer,
        reason_id -> Integer,
        brother_id -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    point_presets (preset_id) {
        preset_id -> Integer,
        title -> Varchar,
        point_quantity -> Integer,
    }
}

table! {
    strikes (id) {
        id -> Integer,
        excusability -> Integer,
        offense -> Integer,
        reason -> Varchar,
        brother_id -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    brothers,
    points,
    point_presets,
    strikes,
);
