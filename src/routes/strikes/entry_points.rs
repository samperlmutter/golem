use diesel::prelude::*;
use rocket::State;
use serde_json::Value;

use crate::{ StrikesDbConn, SlackAuthToken };
use crate::db::{ Strike, Brother };
use crate::routes::send_modal;
use crate::schema::brothers::dsl::*;
use crate::schema::strikes::dsl::*;
use crate::slack::{ SlackSlashCommand, SlackError, SlackResult, SlackResponse, StrikeAction, SlashCmd };

pub fn handle_strikes(conn: StrikesDbConn, slack_msg: &SlackSlashCommand, auth_token: State<SlackAuthToken>) -> SlackResult {
    match &slack_msg.command {
        SlashCmd::Strikes(StrikeAction::Add) if slack_msg.brother.can_act => send_add_strike_modal(slack_msg, auth_token),
        SlashCmd::Strikes(StrikeAction::Remove) if slack_msg.brother.can_act => send_remove_strike_modal(slack_msg, auth_token),
        SlashCmd::Strikes(StrikeAction::List(brother)) => list_brother_strikes(conn, brother),
        SlashCmd::Strikes(StrikeAction::Rank) => rank_strikes(conn),
        SlashCmd::Strikes(StrikeAction::Reset) if slack_msg.brother.can_reset => reset_strikes(conn),
        _ => Err(SlackError::Unauthorized)
    }
}

fn send_add_strike_modal<'a>(slack_msg: &SlackSlashCommand, auth_token: State<'a, SlackAuthToken>) -> SlackResult {
    let modal_json: Value = serde_json::from_str(include_str!("../../json/strikes/add-strike-modal.json"))?;
    send_modal(modal_json, &slack_msg.trigger_id, auth_token)?;

    Ok(SlackResponse::None)
}

fn send_remove_strike_modal<'a>(slack_msg: &SlackSlashCommand, auth_token: State<'a, SlackAuthToken>) -> SlackResult {
    let modal_json: Value = serde_json::from_str(include_str!("../../json/strikes/remove-strike-modal-user-submission.json"))?;
    send_modal(modal_json, &slack_msg.trigger_id, auth_token)?;

    Ok(SlackResponse::None)
}

fn rank_strikes(conn: StrikesDbConn) -> SlackResult {
    let mut res = String::new();

    for brother in brothers.order(name.asc()).load::<Brother>(&conn.0)? {
        let brother = brothers.filter(slack_id.eq(brother.slack_id)).first::<Brother>(&conn.0)?;
        let num_strikes = Strike::belonging_to(&brother).load::<Strike>(&conn.0)?.len();

        res += &format!("• {} has {} strike{}\n",
                        brother.name,
                        num_strikes,
                        if num_strikes == 1 { "" } else { "s" }
        );
    }

    Ok(SlackResponse::Text(res))
}

fn list_brother_strikes(conn: StrikesDbConn, brother: &Brother) -> SlackResult {
    let brother_strikes = Strike::belonging_to(brother).load::<Strike>(&conn.0)?;

    if brother_strikes.is_empty() {
        return Ok(SlackResponse::Text(format!("{} has 0 strikes", brother.name)));
    }

    let mut res = String::new();

    for (i, strike) in brother_strikes.iter().enumerate() {
        res += &format!("{}. {} has an {}\n",
                        i + 1,
                        brother.name,
                        strike
        );
    }

    Ok(SlackResponse::Text(res))
}

fn reset_strikes(conn: StrikesDbConn) -> SlackResult {
    diesel::delete(strikes).execute(&conn.0)?;
    Ok(SlackResponse::Text("Strikes have been reset".to_string()))
}
