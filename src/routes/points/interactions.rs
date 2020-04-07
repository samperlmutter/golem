use std::collections::HashMap;

use serde_json::Value;
use diesel::prelude::*;

use crate::StrikesDbConn;
use crate::slack::{ SlackResult, SlackResponse, SlackError };
use crate::slack::interactions::{ ViewPayload, ModalAction };
use crate::db::{ Brother, InsertablePointsEntry, PointsEntry };
use crate::schema::brothers::dsl::*;
use crate::schema::points::dsl::*;

pub fn receive_change_points_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    let mut amount_val = view_payload.values["change_points_amount_block"]["change_points_amount"]["value"].as_i64().ok_or({
        let mut errs: HashMap<String, String> = HashMap::new();
        errs.insert("change_points_amount_block".to_string(), "Point amount must be an integer greater than 0".to_string());
        SlackError::ModalError(errs)
    })? as i32;
    let reason_val = view_payload.values["change_points_reason_block"]["change_points_reason"]["value"].as_str().unwrap();
    let brother_id_val = view_payload.values["change_points_target_block"]["change_points_target"]["selected_user"].as_str().unwrap();

    if amount_val.is_negative() {
        let mut errs: HashMap<String, String> = HashMap::new();
        errs.insert("change_points_amount_block".to_string(), "Point amount must be greater than 0".to_string());
        return Err(SlackError::ModalError(errs));
    }

    if let ModalAction::SubtractPoints = view_payload.modal_action {
        amount_val *= -1;
    }

    let points_entry = InsertablePointsEntry {
        amount: amount_val,
        reason: reason_val.to_string(),
        brother_id: brother_id_val.to_string()
    };

    diesel::insert_into(points).values(&points_entry).execute(&conn.0)?;

    let brother = brothers.filter(slack_id.eq(&brother_id_val)).first::<Brother>(&conn.0)?;
    let num_points = PointsEntry::belonging_to(&brother).load::<PointsEntry>(&conn.0)?
        .iter()
        .fold(0, |acc, p| acc + p.amount);

    let response_msg = format!("{} now has {} strike{}",
        brother.name,
        num_points,
        if num_points == 1 { "" } else { "s" }
    );

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
