use std::io::Read;

use rocket::data;
use rocket::http::Status;
use rocket::{ Request, Outcome };
use serde_json::Value;
use diesel::prelude::*;

use crate::db::Brother;
use super::SlackError;
use crate::schema::brothers::dsl::*;

pub enum InteractionType {
    ViewSubmission,
    ViewClosed
}

impl std::str::FromStr for InteractionType {
    type Err = SlackError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "view_submission" => Ok(InteractionType::ViewSubmission),
            "view_closed" => Ok(InteractionType::ViewClosed),
            _ => Err(SlackError::InternalServerError("Error during modal interaction`".to_string()))
        }
    }
}

pub struct ViewPayload {
    pub interaction_type: InteractionType,
    pub brother: Brother,
    pub values: Value,
}

impl data::FromDataSimple for ViewPayload {
    type Error = SlackError;

    fn from_data(req: &Request, data: data::Data) -> data::Outcome<ViewPayload, SlackError> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Outcome::Failure((Status::InternalServerError, SlackError::InternalServerError(format!("{:?}", e))));
        }

        let body: String;

        match percent_encoding::percent_decode(string.as_bytes()).decode_utf8() {
            Ok(req) => body = req.replace("+", " ").split("payload=").collect::<String>(),
            Err(e) => return Outcome::Failure((Status::InternalServerError, SlackError::InternalServerError(format!("{:?}", e))))
        }

        let payload: Value = serde_json::from_str(&body).unwrap();
        let interaction_type = match payload["type"].as_str().unwrap().parse::<InteractionType>() {
            Ok(t) => t,
            Err(e) => return Outcome::Failure((Status::InternalServerError, e))
        };
        let user_id = payload["user"]["id"].as_str().unwrap();

        let conn = req.guard::<crate::StrikesDbConn>().succeeded().unwrap();
        let brother = match brothers.filter(slack_id.eq(user_id)).first::<Brother>(&conn.0) {
            Ok(brother) => brother,
            Err(_) => return Outcome::Failure((Status::InternalServerError, SlackError::DatabaseError))
        };

        Outcome::Success(ViewPayload {
            interaction_type,
            brother,
            values: payload["view"]["state"]["values"].clone()
        })
    }
}
