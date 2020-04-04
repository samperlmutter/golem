use rocket::State;
use serde_json::Value;
use diesel::prelude::*;

use crate::{ StrikesDbConn, SlackAuthToken};
use crate::slack::{ SlackSlashCommand, SlackResult, SlackError, SlackResponse };
use crate::slack::interactions::ViewPayload;
use crate::db::{ Strike, InsertableStrike };
use crate::db::excusability::Excusability;
use crate::db::offense::Offense;
use crate::db::Brother;
use crate::schema::brothers::dsl::*;
use crate::schema::strikes::dsl::*;

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
    let excusability_val = view_payload.values["add_strike_excusability_block"]["add_strike_excusability"]["selected_option"]["value"].as_str().unwrap().parse::<Excusability>()?;
    let offense_val = view_payload.values["add_strike_offense_block"]["add_strike_offense"]["selected_option"]["value"].as_str().unwrap().parse::<Offense>()?;
    let reason_val = view_payload.values["add_strike_reason_block"]["add_strike_reason"]["value"].as_str().unwrap();
    let brother_id_val = view_payload.values["add_strike_target_block"]["add_strike_target"]["selected_user"].as_str().unwrap();

    let strike = InsertableStrike {
        excusability: excusability_val,
        offense: offense_val,
        reason: reason_val.to_string(),
        brother_id: brother_id_val.to_string()
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
    let modal_json: Value = serde_json::from_str(include_str!("../json/remove-strike-modal-user-submission.json"))?;
    send_modal(modal_json, &slack_msg.trigger_id, auth_token)?;

    Ok(SlackResponse::None)
}

pub fn update_remove_strike_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    let bro_id = view_payload.values["remove_strike_target_block"]["remove_strike_target"]["selected_user"].as_str().unwrap();
    let brother = brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0)?;
    let brother_strikes = Strike::belonging_to(&brother).load::<Strike>(&conn.0)?;

    if brother_strikes.is_empty() {
        let response = json!({
            "response_action": "update",
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
                            "text": format!("{} doesn't have any strikes", brother.name)
                        }
                    }
                ]
            }
        });

        return Ok(SlackResponse::Raw(response.to_string()));
    }

    let mut options = Vec::new();
    for strike in brother_strikes {
        options.push(json!({
            "text": {
                "type": "mrkdwn",
                "text": format!("{}", strike)
            },
            "value": strike.id.to_string()
        }));
    }

    let mut response: Value = serde_json::from_str(include_str!("../json/remove-strike-modal-strike-submission.json"))?;
    response["view"]["blocks"][0]["element"].as_object_mut().unwrap().insert("options".to_string(), Value::Array(options));

    Ok(SlackResponse::Raw(response.to_string()))
}

pub fn receive_remove_strike_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    let strike_id = view_payload.values["remove_strike_strike_block"]["remove_strike_strike"]["selected_option"]["value"].as_str().unwrap().parse::<i32>()?;
    diesel::delete(strikes.filter(id.eq(strike_id))).execute(&conn.0)?;

    let response = json!({
        "response_action": "clear"
    });

    Ok(SlackResponse::Raw(response.to_string()))
}
