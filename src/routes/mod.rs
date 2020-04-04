pub mod strikes;
pub mod interactions;

use rocket::State;

use super::{ StrikesDbConn, SlackAuthToken };
use crate::slack::{ SlackSlashCommand, SlackError, SlackResponse, SlashCmd };
use crate::slack::interactions::{ ViewPayload, ModalAction };

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_req>")]
pub fn index(conn: StrikesDbConn, slack_req: Result<SlackSlashCommand, SlackError>, auth_token: State<SlackAuthToken>) -> SlackResponse {
    match slack_req.as_ref().map(|msg| (msg.command.clone(), msg)) {
        Ok((SlashCmd::Strikes(_), slack_msg)) => strikes::handle_strikes(conn, slack_msg, auth_token).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Ok(_) => SlackResponse::Text(SlackError::InternalServerError("Slash command parsing error".to_string()).to_string()),
        Err(err) => SlackResponse::Text(err.to_string())
    }
}

#[post("/interaction", format = "application/x-www-form-urlencoded", data = "<view_payload>")]
pub fn interaction(conn: StrikesDbConn, view_payload: Result<ViewPayload, SlackError>) -> SlackResponse {
    match view_payload.as_ref().map(|payload| (payload.modal_action, payload)) {
        Ok((ModalAction::AddStrike, payload)) => interactions::receive_add_strike_modal(conn, payload).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Err(err) => SlackResponse::Text(err.to_string())
    }
}
