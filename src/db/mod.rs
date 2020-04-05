pub mod excusability;
pub mod offense;

use std::fmt::*;
use serde::Deserialize;

use crate::schema::{ brothers, strikes, points };
use excusability::Excusability;
use offense::Offense;

#[derive(Identifiable, Queryable, Debug, Deserialize, PartialEq, Clone)]
#[primary_key(slack_id)]
pub struct Brother {
    pub slack_id: String,
    pub can_act: bool,
    pub can_reset: bool,
    pub name: String,
    pub points: i32,
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
    pub amount: i32,
    pub reason: String,
    pub brother_id: String,
}

#[derive(Insertable, PartialEq)]
#[table_name = "points"]
pub struct InsertablePointsEntry {
    pub amount: i32,
    pub reason: String,
    pub brother_id: String,
}

impl Display for PointsEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "*{} point{}* for *{}*", self.amount, if self.amount == 1 { "" } else { "s" }, self.reason)
    }
}
