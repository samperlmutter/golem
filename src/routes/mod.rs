pub mod strikes;

use super::StrikesDbConn;
use crate::slack::{ SlackSlashCommand, SlackError };

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_req>")]
pub fn index(conn: StrikesDbConn, slack_req: Result<SlackSlashCommand, SlackError>) -> String {
    match slack_req {
        Ok(slack_msg) => {
            match slack_msg.command.as_str() {
                "/strikes" => {
                    match strikes::auth_strikes(conn, slack_msg) {
                        Ok(res) => res,
                        Err(err) => err.to_string()
                    }
                }
                _ => SlackError::InternalServerError("Slash command parsing error".to_string()).to_string()
            }
        }
        Err(err) => err.to_string()
    }
}
