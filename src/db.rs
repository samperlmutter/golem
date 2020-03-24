use std::fmt::*;
use serde::Deserialize;

use crate::schema::{ brothers, strikes };

#[derive(Identifiable, Queryable, Debug, Deserialize, PartialEq)]
#[primary_key(slack_id)]
pub struct Brother {
    pub slack_id: String,
    pub can_act: bool,
    pub can_reset: bool,
    pub name: String,
    pub points: i32,
}

#[derive(PartialEq, Debug, Copy, Clone, Eq)]
pub enum Excusability {
    Excused,
    Unexcused
}

impl Into<Excusability> for i32 {
    fn into(self) -> Excusability {
        match self {
            0 => Excusability::Excused,
            _ => Excusability::Unexcused
        }
    }
}

impl Display for Excusability {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Excusability::Excused => write!(f, "excused"),
            Excusability::Unexcused => write!(f, "unexcused")
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Eq)]
pub enum Offense {
    Tardy,
    Absent
}

impl Into<Offense> for i32 {
    fn into(self) -> Offense {
        match self {
            0 => Offense::Tardy,
            _ => Offense::Absent
        }
    }
}

impl Display for Offense {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Offense::Tardy => write!(f, "tardy"),
            Offense::Absent => write!(f, "absent")
        }
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Brother)]
pub struct Strike {
    id: i32,
    #[diesel(deserialize_as = "i32")]
    excusability: Excusability,
    #[diesel(deserialize_as = "i32")]
    offense: Offense,
    reason: String,
    brother_id: String,
}

impl Display for Strike {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "*{} {}* for reason: *{}*", self.excusability, self.offense, self.reason)
    }
}
