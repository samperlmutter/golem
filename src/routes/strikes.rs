use super::StrikesDbConn;
use crate::slack::{ SlackSlashCommand, SlackError };

pub fn auth_strikes(conn: StrikesDbConn, slack_msg: SlackSlashCommand) -> Result<String, SlackError> {
    let cmd = slack_msg.text.split_whitespace().nth(0).unwrap();

    match cmd {
        "add" | "remove" => {
            if slack_msg.brother.can_act {
                strikes_handler(conn, slack_msg)
            } else {
                Err(SlackError::Unauthorized)
            }
        }
        "reset" => {
            if slack_msg.brother.can_reset {
                strikes_handler(conn, slack_msg)
            } else {
                Err(SlackError::Unauthorized)
            }
        }
        _ => strikes_handler(conn, slack_msg)
    }
}

pub fn strikes_handler(conn: StrikesDbConn, slack_msg: SlackSlashCommand) -> Result<String, SlackError> {
    let param_list: Vec<&str> = slack_msg.text.split_whitespace().collect();
    match param_list[0] {
        "add" => add_strike(&conn, &param_list[1..]),
        "list" => {
            match param_list.len() {
                1 => rank_strikes(&conn),
                2 => list_brother_strikes(&conn, &param_list[1..]),
                _ => Err(SlackError::InvalidArgs)
            }
        },
        "remove" => remove_strike(&conn, &param_list[1..]),
        "reset" => reset_strikes(&conn),
        _ => help()
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
    todo!();
}

fn help() -> Result<String, SlackError> {
    todo!();
}
