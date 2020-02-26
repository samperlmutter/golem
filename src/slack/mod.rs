use serde::{Deserialize, Serialize};

#[derive(rocket::request::FromForm, Serialize)]
pub struct SlackSlashCommand {
    pub user_id: String,
    pub command: String,
    pub text: String
}
