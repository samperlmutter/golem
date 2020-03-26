use crate::slack::{ SlackSlashCommand, SlackError, SlackResult };
use crate::StrikesDbConn;

pub fn config_handler(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> SlackResult {
    todo!();
}

fn add_brother(conn: &StrikesDbConn, params: &[&str]) -> SlackResult {
    todo!();
}

fn remove_brother(conn: &StrikesDbConn, params: &[&str]) -> SlackResult {
    todo!();
}

fn help() -> String {
    "*Available commands*:
    >*Add a strike*
    >Type `/golem config brother add` to add a a brother to the database
    >*Remove a strike*
    >Type `/golem config brother remove @{name}` to remove a brother from the database
    >*Help*
    >Type `/golem config help` to display this message"
    .to_string()
}
