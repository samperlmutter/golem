use std::io::Read;
use std::collections::HashMap;
use std::fmt;

use serde::{ Serialize, Deserialize };
use rocket::request::Request;
use rocket::data;
use rocket::http::Status;
use rocket::Outcome;
use diesel::prelude::*;
use percent_encoding::percent_decode;

use crate::db::Brother;
use crate::schema::brothers::dsl::*;

#[derive(Debug, Deserialize)]
pub struct SlackSlashCommand {
    pub user_id: String,
    pub command: String,
    pub text: String,
    pub brother: Brother,
}

#[derive(Serialize, Debug)]
pub enum SlackResponse<'a> {
    #[serde(rename = "text")]
    Text(&'a str),
}

#[derive(Debug)]
pub enum SlackError {
    InternalServerError(String),
    Unauthorized,
    InvalidArgs,
    DatabaseError,
    UserError(String),
}

pub type SlackResult = Result<String, SlackError>;

impl fmt::Display for SlackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlackError::InternalServerError(msg) => write!(f, "Internal server error, contact the Slack Master: {}", msg),
            SlackError::Unauthorized => write!(f, "Sorry, you're not authorized to use this command"),
            SlackError::InvalidArgs => write!(f, "Invalid number of arguments"),
            SlackError::DatabaseError => write!(f, "Error querying database"),
            SlackError::UserError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl<E: std::error::Error> From<E> for SlackError {
    fn from(e: E) -> Self {
        SlackError::InternalServerError(format!("{:?}", e))
    }
}

impl data::FromDataSimple for SlackSlashCommand {
    type Error = SlackError;

    fn from_data(req: &Request, data: data::Data) -> data::Outcome<SlackSlashCommand, SlackError> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Outcome::Failure((Status::InternalServerError, SlackError::InternalServerError(format!("{:?}", e))));
        }

        let body;

        match percent_decode(string.as_bytes()).decode_utf8() {
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

        let user_id = fields.get("user_id").unwrap().clone();
        let command = fields.get("command").unwrap().clone();
        let text = fields.get("text").unwrap().clone();

        let conn = req.guard::<crate::StrikesDbConn>().succeeded().unwrap();
        let brother = match brothers.filter(slack_id.eq(fields.get("user_id").unwrap())).first::<Brother>(&conn.0) {
            Ok(brother) => brother,
            Err(_) => return Outcome::Failure((Status::InternalServerError, SlackError::DatabaseError))
        };

        Outcome::Success(SlackSlashCommand {
            user_id,
            command,
            text,
            brother
        })
    }
}

pub fn parse_slack_id(id: &str) -> SlackResult {
    let (_, id) = id.split_at(2);
    let mat: regex::Match = match regex::Regex::new(r"([A-Z0-9])\w+")
                                .map_err(|e|
                                    SlackError::InternalServerError(format!("Error during slack id parsing: {:?}", e)))?
                                .find(id) {
        Some(mat) => mat,
        None => return Err(SlackError::InternalServerError("Error parsing slack id".to_string()))
    };

    match id.get(mat.start()..mat.end()) {
        Some(s) => Ok(s.to_string()),
        None => Err(SlackError::InternalServerError("Error parsing slack id".to_string()))
    }
}
