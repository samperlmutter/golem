#![feature(proc_macro_hygiene, decl_macro)]

pub mod models;
pub mod schema;
pub mod types;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;

use crate::models::{Brother, Strike};
use crate::schema::brothers::dsl::*;
use dotenv::dotenv;
use rocket_contrib::json::Json;

#[database("strikes")]
struct StrikesDbConn(diesel::MysqlConnection);

#[get("/")]
fn index(conn: StrikesDbConn) -> Result<Json<Vec<Brother>>, String> {
    brothers
        .load::<Brother>(&conn.0)
        .map_err(|err: diesel::result::Error| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(|val: Vec<Brother>|Json(val))
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(StrikesDbConn::fairing())
        .mount("/", routes![index])
        .launch();
}
