use serde::Serialize;

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
