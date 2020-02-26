use serde::{Deserialize, Serialize};
use crate::schema::brothers;

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[primary_key(slack_id)]
pub struct Brother {
    slack_id: String,
    can_act: bool,
    can_reset: bool,
    name: String,
    points: i32,
}
