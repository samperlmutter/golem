pub mod strikes;
pub mod config;

use super::StrikesDbConn;
use crate::slack::{ SlackSlashCommand, SlackError };

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_req>")]
pub fn index(conn: StrikesDbConn, slack_req: Result<SlackSlashCommand, SlackError>) -> String {
    match slack_req.as_ref().map(|msg| (msg.text.split_whitespace().nth(0).unwrap(), msg)) {
        Ok(("strikes", slack_msg)) => strikes::auth_strikes(conn, slack_msg).unwrap_or_else(|e| e.to_string()),
        Ok(("config", slack_msg)) if slack_msg.brother.is_admin => config::config_handler(conn, slack_msg).unwrap_or_else(|e| e.to_string()),
        Ok(("config", _)) => SlackError::Unauthorized.to_string(),
        Ok(_) => SlackError::InternalServerError("Slash command parsing error".to_string()).to_string(),
        Err(err) => err.to_string()
    }
}
