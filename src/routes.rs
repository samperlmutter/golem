use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket::request::LenientForm;
use crate::schema::brothers::dsl::*;
use crate::brother::Brother;
use super::StrikesDbConn;
use crate::slack::SlackSlashCommand;

#[get("/")]
pub fn index(conn: StrikesDbConn) -> Result<Json<Vec<Brother>>, String> {
    brothers
        .load::<Brother>(&conn.0)
        .map_err(|err: diesel::result::Error| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(|val: Vec<Brother>|Json(val))
}
