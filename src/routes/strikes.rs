use super::StrikesDbConn;
use crate::slack::{ SlackSlashCommand, SlackError };
use crate::db::strike;

pub fn auth_strikes(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> Result<String, SlackError> {
    let params: Vec<&str> = slack_msg.text.split_whitespace().collect();

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
    todo!();
}

fn rank_strikes<'a>(conn: &StrikesDbConn) -> Result<String, SlackError> {
    todo!();
}

fn list_brother_strikes(conn: &StrikesDbConn, params: &[&str]) -> Result<String, SlackError> {
    todo!();
}

fn remove_strike(conn: &StrikesDbConn, params: &[&str]) -> Result<String, SlackError> {
    todo!();
}

fn reset_strikes(conn: &StrikesDbConn) -> Result<String, SlackError> {
    strike::delete_all_strikes()?;
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
