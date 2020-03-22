use diesel::prelude::*;

use crate::schema::strikes::dsl::*;
use super::{ Strike, Brother };
use crate::slack::SlackError;

// Add a new strike
pub fn insert_strike(strike: Strike) -> Result<(), SlackError> {
    todo!();
}

// Delete a strike
pub fn delete_strike(strike_id: u32) -> Result<(), SlackError> {
    todo!();
}

// Delete all strikes from the database
pub fn delete_all_strikes() -> Result<(), SlackError> {
    todo!();
}

// Get info about all strikes a brother has
pub fn get_brothers_strikes(brother: Brother) -> Result<Vec<Strike>, SlackError> {
    todo!();
}
