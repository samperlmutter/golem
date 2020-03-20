use std::io::Read;
use std::collections::HashMap;

use serde::Serialize;
use rocket::request::Outcome as ReqOutcome;
use rocket::request::{ Request, FromRequest };
use rocket::data::{ self };
use rocket::http::Status;
use rocket::Outcome;
use diesel::prelude::*;
use percent_encoding::percent_decode;

use crate::db::Brother;
use crate::schema::brothers::dsl::*;

#[derive(Debug, Serialize)]
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

impl<'a> data::FromDataSimple for SlackSlashCommand {
    type Error = String;

    fn from_data(req: &Request, data: data::Data) -> data::Outcome<SlackSlashCommand, String> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let body;

        match percent_decode(string.as_bytes()).decode_utf8() {
            Ok(req) => body = req,
            Err(e) => return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)))
        }

        let mut fields: HashMap<&str, String> = HashMap::new();
        for f in body.split("&") {
            let (key, val) = match f.find('=') {
                Some(i) => (&f[..i], f[(i + 1)..].to_string().clone()),
                None => continue
            };
            fields.insert(key, val);
        }

        let user_id = fields.get("user_id").unwrap().clone();
        let command = fields.get("command").unwrap().clone();
        let text = fields.get("text").unwrap().clone();

        let conn = req.guard::<crate::StrikesDbConn>().succeeded().unwrap();
        let brother = brothers.filter(slack_id.eq(fields.get("user_id").unwrap())).first::<Brother>(&conn.0).ok().unwrap();

        Outcome::Success(SlackSlashCommand {
            user_id,
            command,
            text,
            brother
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'a SlackSlashCommand {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> ReqOutcome<&'a SlackSlashCommand, Self::Error> {
        todo!();
    }
}
