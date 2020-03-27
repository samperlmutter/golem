use rocket::State;
use serde_json::Value;

use crate::SlackAuthToken;
use crate::slack::{ SlackSlashCommand, SlackResult, SlackError };

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
