use diesel::prelude::*;

use crate::StrikesDbConn;
use crate::slack::{ SlackResult, SlackResponse };
use crate::slack::interactions::ViewPayload;
use crate::db::{ Brother, InsertablePointsEntry, PointsEntry };
use crate::schema::brothers::dsl::*;
use crate::schema::points::dsl::*;
use crate::schema::point_presets::dsl::*;

pub fn receive_add_points_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    let brother_ids = view_payload.values["add_points_brothers_block"]["add_points_brothers_action"]["selected_users"].as_array().unwrap();
    let reason_val = view_payload.values["add_points_reason_block"]["add_points_reason_action"]["selected_option"]["value"].as_str().unwrap().parse::<i32>()?;
    let mut points_entries: Vec<InsertablePointsEntry> = vec![];
    let mut response_msg = String::new();
    let get_point_quantity = |preset: i32| -> i32 {
        return point_presets.filter(preset_id.eq(preset)).select(point_quantity).first(&conn.0).unwrap();
    };
    for b_id in brother_ids {
        let b_id = b_id.as_str().unwrap().to_string();
        points_entries.push(InsertablePointsEntry {
            reason_id: reason_val,
            brother_id: b_id.clone()
        });
        let brother = brothers.filter(slack_id.eq(&b_id)).first::<Brother>(&conn.0)?;
        let current_points = PointsEntry::belonging_to(&brother).load::<PointsEntry>(&conn.0)?
            .iter()
            .fold(0, |acc, p| {
                let amt = get_point_quantity(p.reason_id);
                acc + amt
            });
        let num_points = current_points + get_point_quantity(reason_val);
        response_msg.push_str(&format!("{} now has {} points\n",
            brother.name,
            num_points
        ));
    }

    diesel::insert_into(points).values(&points_entries).execute(&conn.0)?;

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
