use diesel::prelude::*;
use rocket::State;
use serde_json::Value;

use super::{ StrikesDbConn, SlackAuthToken };
use crate::db::{ PointsEntry, Brother };
use crate::slack::{ SlackSlashCommand, SlackError, SlackResult, SlackResponse, StrikeAction, SlashCmd };
use crate::schema::brothers::dsl::*;
use crate::schema::points::dsl::*;

pub fn handle_points(conn: StrikesDbConn, slack_msg: &SlackSlashCommand, auth_token: State<SlackAuthToken>) -> SlackResult {
    todo!();
}

fn send_add_points_modal(slack_msg: &SlackSlashCommand, auth_token: State<SlackAuthToken>) -> SlackResult {
    todo!();
}

fn send_remove_points_modal(slack_msg: &SlackSlashCommand, auth_token: State<SlackAuthToken>) -> SlackResult {
    todo!();
}

fn rank_points(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> SlackResult {
    todo!();
}

fn list_points(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> SlackResult {
    todo!();
}

fn reset_points(conn: StrikesDbConn, slack_msg: &SlackSlashCommand) -> SlackResult {
    todo!();
}
