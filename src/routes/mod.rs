pub mod strikes;
pub mod interactions;

use rocket::State;

use super::{ StrikesDbConn, SlackAuthToken };
use crate::slack::{ SlackSlashCommand, SlackError };
use crate::slack::interactions::ViewPayload;

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_req>")]
pub fn index(conn: StrikesDbConn, slack_req: Result<SlackSlashCommand, SlackError>, auth_token: State<SlackAuthToken>) -> String {
    match slack_req.as_ref().map(|msg| (msg.text.split_whitespace().nth(0).unwrap(), msg)) {
        Ok(("strikes", slack_msg)) => strikes::auth_strikes(conn, slack_msg, auth_token).unwrap_or_else(|e| e.to_string()),
        Ok(_) => SlackError::InternalServerError("Slash command parsing error".to_string()).to_string(),
        Err(err) => err.to_string()
    }
}

#[post("/interaction", format = "application/json", data = "<view_payload>")]
pub fn interaction(conn: StrikesDbConn, view_payload: Result<ViewPayload, SlackError>) -> String {
    match view_payload {
        Ok(view_payload) => interactions::receive_add_strike_modal(conn, view_payload).unwrap_or_else(|e| e.to_string()),
        Err(err) => err.to_string()
    }
}
