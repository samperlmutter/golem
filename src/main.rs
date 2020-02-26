#![feature(proc_macro_hygiene, decl_macro)]

mod schema;
mod routes;
mod slack;
mod strike;
mod brother;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;

#[database("strikes")]
pub struct StrikesDbConn(diesel::MysqlConnection);

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(StrikesDbConn::fairing())
        .mount("/", routes![
            routes::index
            ])
        .launch();
}
