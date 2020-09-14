use diesel::prelude::*;
use rocket::State;
use serde_json::Value;

use crate::{ StrikesDbConn, SlackAuthToken };
use crate::db::{ PointsEntry, Brother, PointPreset };
use crate::routes::send_modal;
use crate::slack::{ SlackSlashCommand, SlackError, SlackResult, SlackResponse, PointAction, SlashCmd };
use crate::schema::brothers::dsl::*;
use crate::schema::points::dsl::*;
use crate::schema::point_presets::dsl::*;

pub fn handle_points(conn: StrikesDbConn, slack_msg: &SlackSlashCommand, auth_token: State<SlackAuthToken>) -> SlackResult {
    match &slack_msg.command {
        SlashCmd::Points(PointAction::Add) if slack_msg.brother.can_act => send_add_points_modal(conn, slack_msg, auth_token),
        SlashCmd::Points(PointAction::List(brother)) => list_brother_points(conn, brother),
        SlashCmd::Points(PointAction::Rank) => rank_points(conn),
        SlashCmd::Points(PointAction::Reset) if slack_msg.brother.can_reset => reset_points(conn),
        _ => Err(SlackError::Unauthorized)
    }
}

fn send_add_points_modal(conn: StrikesDbConn, slack_msg: &SlackSlashCommand, auth_token: State<SlackAuthToken>) -> SlackResult {
    let mut modal_json: Value = serde_json::from_str(include_str!("../../json/points/add-points-modal.json"))?;
    let point_options = point_presets.load::<PointPreset>(&conn.0)?
                                            .iter()
                                            .map(|p| {
                                                serde_json::json!({
                                                "text": {
                                                    "type": "plain_text",
                                                    "text": format!("{} (+{})", p.title, p.point_quantity)
                                                },
                                                "value": p.preset_id
                                                })
                                            })
                                            .collect::<serde_json::Value>();
    modal_json["blocks"][1]["element"]["options"] = point_options;
    send_modal(modal_json, &slack_msg.trigger_id, auth_token)?;

    Ok(SlackResponse::None)
}

fn rank_points(conn: StrikesDbConn) -> SlackResult {
    let mut res = String::new();

    let brothers_t = brothers.load::<Brother>(&conn.0)?;
    let points_entries = PointsEntry::belonging_to(&brothers_t).load::<PointsEntry>(&conn.0)?.grouped_by(&brothers_t);
    let mut rankings = brothers_t.iter()
        .zip(points_entries)
        .map(|(brother, point_entries): (&Brother, Vec<PointsEntry>)|
            (brother.name.clone(), point_entries.iter().fold(0, |acc, p| {
                let amt: i32 = point_presets.filter(preset_id.eq(p.reason_id)).select(point_quantity).first(&conn.0).unwrap();
                acc + amt
            }))
        )
        .collect::<Vec<(String, i32)>>();
    rankings.sort_by_key(|r| r.0.clone());
    rankings.reverse();
    rankings.sort_by_key(|r| r.1);
    rankings.reverse();

    for (i, ranking) in rankings.iter().enumerate() {
        res += &format!("{}. *{}* has *{} point{}*\n",
                        i + 1,
                        ranking.0,
                        ranking.1,
                        if ranking.1 == 1 { "" } else { "s" }
        );
    }

    Ok(SlackResponse::Text(res))
}

fn list_brother_points(conn: StrikesDbConn, brother: &Brother) -> SlackResult {
    let brother_points = PointsEntry::belonging_to(brother).load::<PointsEntry>(&conn.0)?;

    if brother_points.is_empty() {
        return Ok(SlackResponse::Text(format!("{} has 0 points", brother.name)));
    }

    let mut res = String::new();
    let mut total = 0;

    for (i, point_entry) in brother_points.iter().enumerate() {
        let point_preset = point_presets.filter(preset_id.eq(point_entry.reason_id)).first::<PointPreset>(&conn.0).unwrap();
        res += &format!("{}. {} gained *{} point{}* for *{}* on {}\n",
                        i + 1,
                        brother.name,
                        point_preset.point_quantity,
                        if point_preset.point_quantity == 1 { "" } else { "s" },
                        point_preset.title,
                        point_entry.timestamp
        );
        total += point_preset.point_quantity;
    }

    res += &format!("\n{} has a total of *{} point{}*",
                    brother.name,
                    total,
                    if total == 1 { "" } else { "s" }
    );

    Ok(SlackResponse::Text(res))
}

fn reset_points(conn: StrikesDbConn) -> SlackResult {
    diesel::delete(points).execute(&conn.0)?;
    Ok(SlackResponse::Text("Points have been reset".to_string()))
}
