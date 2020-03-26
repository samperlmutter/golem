pub mod excusability;
pub mod offense;

use std::fmt::*;
use serde::Deserialize;

use crate::schema::{ brothers, strikes };
use excusability::Excusability;
use offense::Offense;

#[derive(Identifiable, Queryable, Debug, Deserialize, PartialEq)]
#[primary_key(slack_id)]
pub struct Brother {
    pub slack_id: String,
    pub can_act: bool,
    pub is_admin: bool,
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
