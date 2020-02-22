table! {
    brothers (slack_id) {
        slack_id -> Varchar,
        can_strike -> Bool,
        can_reset -> Bool,
        name -> Varchar,
        points -> Integer,
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
    strikes,
);
