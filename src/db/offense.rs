use std::fmt::*;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;

use crate::slack::SlackError;

#[derive(PartialEq, Debug, Eq, AsExpression)]
#[sql_type = "Integer"]
pub enum Offense {
    Tardy,
    Absence
}

impl std::str::FromStr for Offense {
    type Err = SlackError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "tardy" => Ok(Offense::Tardy),
            "absence" => Ok(Offense::Absence),
            _ => Err(SlackError::UserError("Invalid offense. Valid options are `tardy` and `absence`".to_string()))
        }
    }
}

impl<DB: Backend> ToSql<Integer, DB> for Offense
    where
        i32: ToSql<Integer, DB>,
    {
        fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        let v = match *self {
            Offense::Tardy => 0,
            Offense::Absence => 1,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Integer, DB> for Offense
    where
        i32: FromSql<Integer, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            let v = i32::from_sql(bytes)?;
            Ok(match v {
                0 => Offense::Tardy,
                _ => Offense::Absence,
            })
    }
}

impl Into<Offense> for i32 {
    fn into(self) -> Offense {
        match self {
            0 => Offense::Tardy,
            _ => Offense::Absence
        }
    }
}

impl Display for Offense {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Offense::Tardy => write!(f, "tardy"),
            Offense::Absence => write!(f, "absence")
        }
    }
}
