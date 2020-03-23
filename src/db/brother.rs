use diesel::prelude::*;

use crate::schema::brothers::dsl::*;
use crate::slack::SlackError;
use super::{ Brother, Strike };
use crate::StrikesDbConn;

// Get a specific brother from the database
pub fn get_brother(conn: &StrikesDbConn, bro_id: &String) -> Result<Brother, SlackError> {
    match brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0) {
        Ok(brother) => Ok(brother),
        Err(_) => Err(SlackError::DatabaseError)
    }
}

// Get list of all brothers from the database
pub fn get_all_brothers(conn: &StrikesDbConn, ) -> Result<Vec<Brother>, SlackError> {
    match brothers.order(name.asc()).load::<Brother>(&conn.0) {
        Ok(brothers_list) => Ok(brothers_list),
        Err(_) => Err(SlackError::DatabaseError)
    }
}

// Get the number of strikes a specific brother has
pub fn get_brother_num_strikes(conn: &StrikesDbConn, bro_id: &String) -> Result<i64, SlackError> {
    let brother = brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0)?;
    let brother_strikes = Strike::belonging_to(&brother).load::<Strike>(&conn.0)?;
    Ok(brother_strikes.len() as i64)
}
