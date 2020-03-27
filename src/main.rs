#![feature(proc_macro_hygiene, decl_macro)]

mod schema;
mod routes;
mod slack;
mod db;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::response::Responder;
use rocket_contrib::json::Json;

use crate::slack::SlackResponse;

#[database("strikes")]
pub struct StrikesDbConn(diesel::MysqlConnection);

pub struct SlackAuthToken(String);

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(StrikesDbConn::fairing())
        .attach(AdHoc::on_response("Slack response", |req, response: &mut rocket::Response| {
            let body_str = response.body_string().unwrap_or(String::new());
            let json = Json(SlackResponse::Text(body_str.as_str()));
            response.merge(json.respond_to(req).unwrap());
        }))
        .attach(AdHoc::on_attach("Slack auth token config", |rocket| {
            let token = rocket.config()
                .get_str("slack_auth_token")
                .unwrap_or("faje_token")
                .to_string();

            Ok(rocket.manage(SlackAuthToken(token)))
        }))
        .mount("/", routes![
            routes::index,
            ])
        .launch();
}
