use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(rocket::request::FromForm, Serialize)]
pub struct SlackSlashCommand {
    pub user_id: String,
    pub command: String,
    pub text: String,
}

#[derive(Serialize)]
pub enum SlackResponse<'a> {
    #[serde(rename = "text")]
    Text(&'a str),
}

pub fn response(string: &str) -> Json<SlackResponse> {
    Json(SlackResponse::Text(string))
}
