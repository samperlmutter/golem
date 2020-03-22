pub mod strikes;

use super::StrikesDbConn;
use crate::slack::{ SlackSlashCommand, SlackError };

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_req>")]
pub fn index(conn: StrikesDbConn, slack_req: Result<SlackSlashCommand, SlackError>) -> String {
    match slack_req.as_ref().map(|msg| (msg.command.as_str(), msg)) {
        Ok(("/strikes", slack_msg)) => strikes::auth_strikes(conn, slack_msg).unwrap_or_else(|e| e.to_string()),
        Ok(_) => SlackError::InternalServerError("Slash command parsing error".to_string()).to_string(),
        Err(err) => err.to_string()
    }
}
