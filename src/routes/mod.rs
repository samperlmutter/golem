mod strikes;

use rocket::request::LenientForm;

use super::StrikesDbConn;
use crate::slack;
use crate::db::strike;

#[post("/", format = "application/x-www-form-urlencoded", data = "<request>")]
pub fn index<'a>(conn: StrikesDbConn, request: LenientForm<slack::SlackSlashCommand>) -> &'a str {
    let slack_msg = request.into_inner();

    match slack_msg.command.as_str() {
        "/strikes" => strike::strike_handler(&conn, slack_msg.text),
        _ => "Something's wrong with the slack bot, contact the Slack Master immediately"
    }
}
