#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

#[database("strikes")]
struct StrikesDbConn(diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(StrikesDbConn::fairing())
        .mount("/", routes![index])
        .launch();
}
