use rocket_contrib::json::Json;
use rocket::request::LenientForm;
use super::StrikesDbConn;
use crate::slack::{ SlackSlashCommand, SlackResponse };
use crate::strike::service;

#[post("/", format = "application/x-www-form-urlencoded", data = "<request>")]
pub fn index<'a>(conn: StrikesDbConn, request: LenientForm<SlackSlashCommand>) -> Json<SlackResponse<'a>> {
    let slack_msg = request.into_inner();

    match slack_msg.command.as_str() {
        "/strikes" => service::strike_handler(&conn, slack_msg.text),
        _ => Json(SlackResponse::Text("Something's wrong with the slack bot, contact the Slack Master immediately"))
    }
}
