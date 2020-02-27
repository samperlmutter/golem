pub mod service;

use crate::brother::Brother;
use crate::schema::strikes;

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
