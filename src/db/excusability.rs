use std::fmt::*;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;

use crate::slack::SlackError;

#[derive(PartialEq, Debug, Eq, AsExpression)]
#[sql_type = "Integer"]
pub enum Excusability {
    Excused,
    Unexcused
}

impl std::str::FromStr for Excusability {
    type Err = SlackError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "excused" => Ok(Excusability::Excused),
            "unexcused" => Ok(Excusability::Unexcused),
            _ => Err(SlackError::UserError("Invalid excusability. Valid options are `excused` and `unexcused`".to_string()))
        }
    }
}

impl<DB: Backend> ToSql<Integer, DB> for Excusability
    where
        i32: ToSql<Integer, DB>,
    {
        fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        let v = match *self {
            Excusability::Excused => 0,
            Excusability::Unexcused => 1,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Integer, DB> for Excusability
    where
        i32: FromSql<Integer, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            let v = i32::from_sql(bytes)?;
            match v {
                0 => Ok(Excusability::Excused),
                1 => Ok(Excusability::Unexcused),
                x => Err(SlackError::InternalServerError(format!("Unrecognized variant {} while deserializing Excusability", x)).to_string().into())
            }
    }
}

impl Into<Excusability> for i32 {
    fn into(self) -> Excusability {
        match self {
            0 => Excusability::Excused,
            _ => Excusability::Unexcused // Into doesn't allow errors, but this will never not be `1` because of the `ToSql` impl
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
