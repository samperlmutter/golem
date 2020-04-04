pub mod interactions;

use std::io::Read;
use std::collections::HashMap;
use std::fmt;

use serde::Serialize;
use rocket::request::Request;
use rocket::http::ContentType;
use rocket::data;
use rocket::http::Status;
use rocket::response::{self, Response, Responder};
use rocket::Outcome;
use rocket_contrib::json::Json;
use diesel::prelude::*;

use crate::db::Brother;
use crate::schema::brothers::dsl::*;

#[derive(Clone)]
pub enum StrikeAction {
    Add,
    Remove(Brother, i32),
    List(Brother),
    Rank,
    Reset,
}

impl StrikeAction {
    pub fn from_str(conn: &crate::StrikesDbConn, params: &[&str]) -> Result<Self, SlackError> {
        let help =
"*Available commands*:
>*Add a strike*
>Type `/golem strikes add @{name} {excused | unexcused} {tardy | absence} {reason}` to add a strike to the specified user
>*Remove a strike*
>Type `/golem strikes remove @{name} {strikeNumber}` to remove the specified strike from the specified
>*List everyone's strikes*
>Type `/golem strikes list [@{name}]` to list how many strikes each user has, sorted numerically
>Optionally mention a user to list information about their strikes
>*Reset strikes*
>Type `/golem strikes reset` to reset everyone's strikes to 0
>This should only be done at the end of the semester
>*Help*
>Type `/golem strikes help` to display this message"
.to_string();

        if params.len() > 0 {
            match params[0] {
                "add" => Ok(StrikeAction::Add),
                "remove" if params.len() == 3 => {
                    let bro_id = parse_slack_id(&params[1])?;
                    let brother = brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0)?;
                    let strike_id = params[2].parse::<i32>()?;

                    Ok(StrikeAction::Remove(brother, strike_id))
                },
                "list" if params.len() == 2 => {
                    let bro_id = parse_slack_id(&params[1])?;
                    let brother = brothers.filter(slack_id.eq(bro_id)).first::<Brother>(&conn.0)?;

                    Ok(StrikeAction::List(brother))
                },
                "list" if params.len() == 1 => Ok(StrikeAction::Rank),
                "reset" => Ok(StrikeAction::Reset),
                _ => Err(SlackError::InvalidCmd(help))
            }
        } else {
            Err(SlackError::InvalidCmd(help))
        }
    }
}

#[derive(Clone)]
pub enum SlashCmd {
    Strikes(StrikeAction),
}

#[derive(Serialize, Debug)]
pub enum SlackResponse {
    #[serde(rename = "text")]
    Text(String),
    None,
    Raw(String),
}

impl<'r> Responder<'r> for SlackResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut response = Response::build();
        match self {
            SlackResponse::Text(text) => {response.merge(Json(SlackResponse::Text(text)).respond_to(req).unwrap());}
            SlackResponse::None => {},
            SlackResponse::Raw(text) => {
                response.merge(text.respond_to(req).unwrap());
                response.header(ContentType::JSON);
            }
        }
        response.ok()
    }
}

#[derive(Debug)]
pub enum SlackError {
    InternalServerError(String),
    Unauthorized,
    InvalidArgs,
    DatabaseError,
    UserError(String),
    InvalidCmd(String),
}

pub type SlackResult = Result<SlackResponse, SlackError>;

impl fmt::Display for SlackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlackError::InternalServerError(msg) => write!(f, "Internal server error, contact the Slack Master: {}", msg),
            SlackError::Unauthorized => write!(f, "Sorry, you're not authorized to use this command"),
            SlackError::InvalidArgs => write!(f, "Invalid number of arguments"),
            SlackError::DatabaseError => write!(f, "Error querying database"),
            SlackError::UserError(msg) => write!(f, "Error: {}", msg),
            SlackError::InvalidCmd(help_text) => write!(f, "{}", help_text),
        }
    }
}

impl<E: std::error::Error> From<E> for SlackError {
    fn from(e: E) -> Self {
        SlackError::InternalServerError(format!("{:?}", e))
    }
}

pub struct SlackSlashCommand {
    pub user_id: String,
    pub command: SlashCmd,
    pub trigger_id: String,
    pub brother: Brother,
}

impl data::FromDataSimple for SlackSlashCommand {
    type Error = SlackError;

    fn from_data(req: &Request, data: data::Data) -> data::Outcome<SlackSlashCommand, SlackError> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Outcome::Failure((Status::InternalServerError, SlackError::InternalServerError(format!("{:?}", e))));
        }

        let body;

        match percent_encoding::percent_decode(string.as_bytes()).decode_utf8() {
            Ok(req) => body = req.replace("+", " "),
            Err(e) => return Outcome::Failure((Status::InternalServerError, SlackError::InternalServerError(format!("{:?}", e))))
        }

        let mut fields: HashMap<&str, String> = HashMap::new();
        for f in body.split("&") {
            let (key, val) = match f.find('=') {
                Some(i) => (&f[..i], f[(i + 1)..].to_string()),
                None => continue
            };
            fields.insert(key, val);
        }

        let conn = req.guard::<crate::StrikesDbConn>().succeeded().unwrap();
        let mut iter = fields.get("text").unwrap().as_str().split_whitespace();

        let cmd = match iter.next() {
            Some(c) => c,
            None => return Outcome::Failure((Status::InternalServerError, SlackError::InvalidArgs))
        };

        let command = match cmd {
            "strikes" => {
                let action = match StrikeAction::from_str(&conn, &iter.collect::<Vec<&str>>()) {
                    Ok(sa) => sa,
                    Err(e) => return Outcome::Failure((Status::InternalServerError, e))
                };
                SlashCmd::Strikes(action)
            }
            _ => return Outcome::Failure((Status::InternalServerError, SlackError::InvalidArgs))
        };

        let user_id = fields.get("user_id").unwrap().clone();
        let trigger_id = fields.get("trigger_id").unwrap().clone();

        let brother = match brothers.filter(slack_id.eq(fields.get("user_id").unwrap())).first::<Brother>(&conn.0) {
            Ok(brother) => brother,
            Err(_) => return Outcome::Failure((Status::InternalServerError, SlackError::DatabaseError))
        };

        Outcome::Success(SlackSlashCommand {
            user_id,
            command,
            brother,
            trigger_id,
        })
    }
}

pub fn parse_slack_id(id: &str) -> Result<&str, SlackError> {
    let (_, id) = id.split_at(2);
    let mat: regex::Match = match regex::Regex::new(r"([A-Z0-9])\w+")
                                .map_err(|e|
                                    SlackError::InternalServerError(format!("Error during slack id parsing: {:?}", e)))?
                                .find(id) {
        Some(mat) => mat,
        None => return Err(SlackError::InternalServerError("Invalid user specified".to_string()))
    };

    match id.get(mat.start()..mat.end()) {
        Some(s) => Ok(s),
        None => Err(SlackError::InternalServerError("Error parsing slack id".to_string()))
    }
}
