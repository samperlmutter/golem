use rocket::State;
use serde_json::Value;

use crate::{ StrikesDbConn, SlackAuthToken};
use crate::slack::{ SlackSlashCommand, SlackResult, SlackError };
use crate::slack::interactions::ViewPayload;
use crate::db::InsertableStrike;
use crate::db::excusability::Excusability;
use crate::db::offense::Offense;

pub fn send_add_strike_modal<'a>(slack_msg: &SlackSlashCommand, auth_token: State<'a, SlackAuthToken>) -> SlackResult {
    let modal_json: Value = serde_json::from_str(include_str!("../json/add-brother-modal.json"))?;
    let body = json! ({
        "trigger_id": slack_msg.trigger_id,
        "view": modal_json
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://slack.com/api/views.open")
        .bearer_auth(&auth_token.0)
        .body(body.to_string())
        .send()?;

    let text = res.text()?;
    let json_res: Value = serde_json::from_str(&text)?;

    if let Value::Bool(ok) = json_res["ok"] {
        if !ok {
            return Err(SlackError::InternalServerError("".to_string()));
        }
    }

    Ok(String::new())
}

pub fn receive_add_strike_modal(conn: StrikesDbConn, view_payload: ViewPayload) -> SlackResult {
    if !view_payload.brother.can_act {
        return Err(SlackError::Unauthorized);
    }

    let strike = InsertableStrike {
        excusability: view_payload.values.get("add_brother_excusability").unwrap().parse::<Excusability>()?,
        offense: view_payload.values.get("add_brother_offense").unwrap().parse::<Offense>()?,
        reason: view_payload.values.get("add_brother_reason").unwrap().clone(),
        brother_id: view_payload.brother.slack_id.clone()
    };

    let response_msg = super::strikes::add_strike(&conn, strike)?;

    let response = json!({
        "response_action": "update",
        "view": {
            "type": "modal",
            "title": {
                "type": "plain_text",
                "text": "Updated view"
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

    Ok(response.to_string())
}
