pub use excusability::Excusability;
pub use offense::Offense;

mod excusability;
mod offense;

use std::fmt::*;
use serde::Deserialize;
use chrono::naive::NaiveDateTime;

use crate::schema::*;

#[derive(Identifiable, Queryable, Debug, Deserialize, PartialEq, Clone)]
#[primary_key(slack_id)]
pub struct Brother {
    pub slack_id: String,
    pub can_act: bool,
    pub can_reset: bool,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Brother)]
pub struct Strike {
    pub id: i32,
    #[diesel(deserialize_as = "i32")]
    excusability: Excusability,
    #[diesel(deserialize_as = "i32")]
    offense: Offense,
    reason: String,
    brother_id: String,
}

#[derive(Insertable, PartialEq)]
#[table_name = "strikes"]
pub struct InsertableStrike {
    pub excusability: Excusability,
    pub offense: Offense,
    pub reason: String,
    pub brother_id: String,
}

impl Display for Strike {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "*{} {}* for reason: *{}*", self.excusability, self.offense, self.reason)
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Brother)]
#[table_name = "points"]
pub struct PointsEntry {
    pub id: i32,
    pub reason_id: i32,
    pub brother_id: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, PartialEq)]
#[table_name = "points"]
pub struct InsertablePointsEntry {
    pub reason_id: i32,
    pub brother_id: String,
}

#[derive(Identifiable, Queryable)]
#[primary_key(preset_id)]
pub struct PointPreset {
    pub preset_id: i32,
    pub title: String,
    pub point_quantity: i32,
}
