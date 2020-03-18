pub mod strike;

use serde::{Deserialize, Serialize};
use crate::schema::{ brothers, strikes };

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize, rocket::request::FromForm)]
#[primary_key(slack_id)]
pub struct Brother {
    pub slack_id: String,
    pub can_act: bool,
    pub can_reset: bool,
    pub name: String,
    pub points: i32,
}

#[derive(PartialEq, Debug)]
pub enum Excusability {
    Excused,
    Unexcused
}

#[derive(PartialEq, Debug)]
pub enum Offense {
    Tardy,
    Absent
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Brother)]
pub struct Strike {
    id: u32,
    excusability: Excusability,
    offense: Offense,
    reason: String,
    brother_id: String,
}
