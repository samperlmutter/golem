pub mod strikes;
pub mod points;
pub mod interactions;

use rocket::State;
use serde_json::Value;

use super::{ StrikesDbConn, SlackAuthToken };
use crate::slack::{ SlackSlashCommand, SlackError, SlackResponse, SlashCmd };
use crate::slack::interactions::{ ViewPayload, ModalAction };

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_req>")]
pub fn index(conn: StrikesDbConn, slack_req: Result<SlackSlashCommand, SlackError>, auth_token: State<SlackAuthToken>) -> SlackResponse {
    match slack_req.as_ref().map(|msg| (msg.command.clone(), msg)) {
        Ok((SlashCmd::Strikes(_), slack_msg)) => strikes::handle_strikes(conn, slack_msg, auth_token).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Ok((SlashCmd::Points(_), slack_msg)) => points::handle_points(conn, slack_msg, auth_token).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Err(err) => SlackResponse::Text(err.to_string())
    }
}

#[post("/interaction", format = "application/x-www-form-urlencoded", data = "<view_payload>")]
pub fn interaction(conn: StrikesDbConn, view_payload: Result<ViewPayload, SlackError>) -> SlackResponse {
    match view_payload.as_ref().map(|payload| (payload.modal_action, payload)) {
        Ok((ModalAction::AddStrike, payload)) => interactions::receive_add_strike_modal(conn, payload).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Ok((ModalAction::RemoveStrikeUser, payload)) => interactions::update_remove_strike_modal(conn, payload).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Ok((ModalAction::RemoveStrikeStrike, payload)) => interactions::receive_remove_strike_modal(conn, payload).unwrap_or_else(|e| SlackResponse::Text(e.to_string())),
        Err(err) => SlackResponse::Text(err.to_string())
    }
}

fn send_modal<'a>(modal: Value, trigger_id: &String, auth_token: State<'a, SlackAuthToken>) -> Result<(), SlackError> {
    let body = json! ({
        "trigger_id": trigger_id,
        "view": modal
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://slack.com/api/views.open")
        .header("Content-Type", "application/json")
        .bearer_auth(&auth_token.0)
        .body(body.to_string())
        .send()?;

    let text = res.text()?;
    let json_res: Value = serde_json::from_str(&text)?;

    if let Value::Bool(ok) = json_res["ok"] {
        if !ok {
            return Err(SlackError::InternalServerError(text));
        }
    }

    Ok(())
}
