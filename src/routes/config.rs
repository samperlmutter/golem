use crate::slack::{ SlackSlashCommand, SlackError, SlackResult };
use crate::StrikesDbConn;

pub fn auth_config(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> SlackResult {
    todo!();
}

fn config_handler(conn: StrikesDbConn, params: &[&str]) -> SlackResult {
    todo!();
}

fn add_brother(conn: &StrikesDbConn, params: &[&str]) -> SlackResult {
    todo!();
}

fn remove_brother(conn: &StrikesDbConn, params: &[&str]) -> SlackResult {
    todo!();
}
