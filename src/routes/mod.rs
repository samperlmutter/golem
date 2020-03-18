mod strikes;

use super::StrikesDbConn;
use crate::slack;
use crate::db::strike;

#[post("/", format = "application/x-www-form-urlencoded", data = "<slack_msg>")]
pub fn index<'a>(conn: StrikesDbConn, slack_msg: slack::SlackSlashCommand) -> &'a str {
    match slack_msg.command.as_str() {
        "/strikes" => strike::strike_handler(&conn, slack_msg.text),
        _ => "Something's wrong with the slack bot, contact the Slack Master immediately"
    }
}
