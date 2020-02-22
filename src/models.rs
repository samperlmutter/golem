use crate::schema::{brothers, strikes};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[primary_key(slack_id)]
pub struct Brother {
    slack_id: String,
    can_act: bool,
    can_reset: bool,
    name: String,
    points: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Brother)]
pub struct Strike {
    id: u32,
    excusability: crate::types::Excusability,
    offense: crate::types::Offense,
    reason: String,
    brother_id: String,
}
