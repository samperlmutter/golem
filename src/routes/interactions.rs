use rocket::State;
use serde_json::Value;
use diesel::prelude::*;

use crate::{ StrikesDbConn, SlackAuthToken};
use crate::slack::{ SlackSlashCommand, SlackResult, SlackError, SlackResponse };
use crate::slack::interactions::ViewPayload;
use crate::db::{ Strike, InsertableStrike };
use crate::db::excusability::Excusability;
use crate::db::offense::Offense;

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

pub fn send_add_strike_modal<'a>(slack_msg: &SlackSlashCommand, auth_token: State<'a, SlackAuthToken>) -> SlackResult {
    let modal_json: Value = serde_json::from_str(include_str!("../json/add-strike-modal.json"))?;
    send_modal(modal_json, &slack_msg.trigger_id, auth_token)?;

    Ok(SlackResponse::None)
}

pub fn receive_add_strike_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    let excusability = view_payload.values["add_strike_excusability_block"]["add_strike_excusability"]["selected_option"]["value"].as_str().unwrap().parse::<Excusability>()?;
    let offense = view_payload.values["add_strike_offense_block"]["add_strike_offense"]["selected_option"]["value"].as_str().unwrap().parse::<Offense>()?;
    let reason = view_payload.values["add_strike_reason_block"]["add_strike_reason"]["value"].as_str().unwrap();
    let brother_id = view_payload.values["add_strike_target_block"]["add_strike_target"]["selected_user"].as_str().unwrap();

    let strike = InsertableStrike {
        excusability,
        offense,
        reason: reason.to_string(),
        brother_id: brother_id.to_string()
    };

    let response_msg = super::strikes::add_strike(&conn, strike)?;

    let response = json!({
        "response_action": "update",
        "view": {
            "type": "modal",
            "title": {
                "type": "plain_text",
                "text": "Success!"
            },
            "close": {
                "type": "plain_text",
                "text": "Close"
            },
            "blocks": [
                {
                    "type": "section",
                    "text": {
                        "type": "plain_text",
                        "text": response_msg
                    }
                }
            ]
        }
    });

    Ok(SlackResponse::Raw(response.to_string()))
}

pub fn send_remove_strike_modal<'a>(slack_msg: &SlackSlashCommand, auth_token: State<'a, SlackAuthToken>) -> SlackResult {
    let modal_json: Value = serde_json::from_str(include_str!("../json/remove-strike-modal.json"))?;
    send_modal(modal_json, &slack_msg.trigger_id, auth_token)?;

    Ok(SlackResponse::None)
}

pub fn update_remove_strike_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    let brother_strikes = Strike::belonging_to(&view_payload.brother).load::<Strike>(&conn.0)?;
    let response = json!({
        "response_action": "push",
        "view": {
            "type": "modal",
            "title": {
                "type": "plain_text",
                "text": "Remove a strike"
            },
            "close": {
                "type": "plain_text",
                "text": "Close"
            },
            "blocks": [
                {
                    "type": "section",
                    "text": {
                        "type": "plain_text",
                        "text": "hi there"
                    }
                }
            ]
        }
    });

    Ok(SlackResponse::Raw(response.to_string()))
}
