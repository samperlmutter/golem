use diesel::prelude::*;

use super::StrikesDbConn;
use crate::db::{ Strike, Brother, InsertableStrike };
use crate::db::excusability::Excusability;
use crate::db::offense::Offense;
use crate::slack::{ SlackSlashCommand, SlackError };
use crate::slack;
use crate::schema::brothers::dsl::*;
use crate::schema::strikes::dsl::*;

pub fn auth_strikes(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> Result<String, SlackError> {
    let params: Vec<&str> = slack_msg.text.split("+").collect();

    match params[0] {
        "add" | "remove" => {
            if slack_msg.brother.can_act {
                strikes_handler(conn, &params)
            } else {
                Err(SlackError::Unauthorized)
            }
        }
        "reset" => {
            if slack_msg.brother.can_reset {
                strikes_handler(conn, &params)
            } else {
                Err(SlackError::Unauthorized)
            }
        }
        _ => strikes_handler(conn, &params)
    }
}

pub fn strikes_handler(conn: StrikesDbConn, params: &[&str]) -> Result<String, SlackError> {
    match params[0] {
        "add" => add_strike(&conn, &params[1..]),
        "list" => {
            match params.len() {
                1 => rank_strikes(&conn),
                2 => list_brother_strikes(&conn, &params[1..]),
                _ => Err(SlackError::InvalidArgs)
            }
        },
        "remove" => remove_strike(&conn, &params[1..]),
        "reset" => reset_strikes(&conn),
        _ => Ok(help())
    }
}

fn add_strike(conn: &StrikesDbConn, params: &[&str]) -> Result<String, SlackError> {
    if params.len() < 4 {
        return Err(SlackError::InvalidArgs);
    }

    let excuse = params[1].parse::<Excusability>()?;
    let action = params[2].parse::<Offense>()?;
    let motive = params[3..].join(" ");
    let bro_id = slack::parse_slack_id(params[0])?.to_string();

    let new_strike = InsertableStrike {
        excusability: excuse,
        offense: action,
        reason: motive,
        brother_id: bro_id
    };

    diesel::insert_into(strikes).values(&new_strike).execute(&conn.0)?;

    let brother = brothers.filter(slack_id.eq(new_strike.brother_id)).first::<Brother>(&conn.0)?;
    let num_strikes = Strike::belonging_to(&brother).load::<Strike>(&conn.0)?.len();

    Ok(format!("{} now has {} strike{}",
        brother.name,
        num_strikes,
        if num_strikes == 1 { "" } else { "s" }
    ))
}

fn rank_strikes(conn: &StrikesDbConn) -> Result<String, SlackError> {
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

    Ok(res)
}

fn list_brother_strikes(conn: &StrikesDbConn, params: &[&str]) -> Result<String, SlackError> {
    let bro_id = slack::parse_slack_id(&params[0])?;
    let brother = brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0)?;
    let brother_strikes = Strike::belonging_to(&brother).load::<Strike>(&conn.0)?;

    if brother_strikes.is_empty() {
        return Ok(format!("{} has 0 strikes", brother.name));
    }

    let mut res = String::new();

    for (i, strike) in brother_strikes.iter().enumerate() {
        res += &format!("{}. {} has an {}\n",
                        i + 1,
                        brother.name,
                        strike
        );
    }

    Ok(res)
}

fn remove_strike(conn: &StrikesDbConn, params: &[&str]) -> Result<String, SlackError> {
    if params.len() != 2 {
        return Err(SlackError::InvalidArgs);
    }

    let bro_id = slack::parse_slack_id(&params[0])?;
    let brother = brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0)?;
    let brother_strikes = Strike::belonging_to(&brother).load::<Strike>(&conn.0)?;

    if brother_strikes.is_empty() {
        return Ok(format!("{} has no strikes to remove", brother.name));
    }

    let strike_num = params[1].to_string().parse::<i32>()?;
    if strike_num < 1 || strike_num > brother_strikes.len() as i32 {
        return Err(SlackError::UserError("Please choose a valid strike id. run `/strikes list <@user>` to see their strikes".to_string()));
    }

    let strike = brother_strikes.get((strike_num - 1) as usize).unwrap();
    diesel::delete(strikes.filter(id.eq(strike.id))).execute(&conn.0)?;

    Ok(format!("{} now has {} strike{}",
                brother.name,
                brother_strikes.len() - 1,
                if brother_strikes.len() - 1 == 1 { "" } else { "s" })
            )
}

fn reset_strikes(conn: &StrikesDbConn) -> Result<String, SlackError> {
    diesel::delete(strikes).execute(&conn.0)?;
    Ok("Strikes have been reset".to_string())
}

fn help() -> String {
    "*Available commands*:
    >*Add a strike*
    >Type `/strikes add @{name} {excused | unexcused} {tardy | absence} {reason}` to add a strike to the specified user
    >*Remove a strike*
    >Type `/strikes remove @{name} {strikeNumber}` to remove the specified strike from the specified
    >*List everyone's strikes*
    >Type `/strikes list [@{name}]` to list how many strikes each user has, sorted numerically
    >Optionally mention a user to list information about their strikes
    >*Reset strikes*
    >Type `/strikes reset` to reset everyone's strikes to 0
    >This should only be done at the end of the semester
    >*Help*
    >Type `/strikes help` to display this message"
    .to_string()
}
