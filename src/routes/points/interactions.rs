use serde_json::Value;
use diesel::prelude::*;

use crate::StrikesDbConn;
use crate::slack::{ SlackResult, SlackResponse };
use crate::slack::interactions::ViewPayload;
use crate::db::InsertablePointsEntry;
use crate::db::Brother;
use crate::schema::brothers::dsl::*;
use crate::schema::points::dsl::*;

pub fn receive_add_points_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    todo!();
}

pub fn receive_remove_points_modal<'a>(conn: StrikesDbConn, view_payload: &ViewPayload) -> SlackResult {
    todo!();
}
