use crate::StrikesDbConn;
use crate::slack::SlackResponse;
use rocket_contrib::json::Json;
use crate::slack;

pub fn strike_handler<'a>(conn: &StrikesDbConn, params: String) -> Json<SlackResponse<'a>> {
    let param_list: Vec<&str> = params.split_whitespace().collect();

    match param_list[0] {
        "add" => add_strike(&conn, param_list),
        "list" => {
            match param_list.len() {
                1 => rank_strikes(&conn),
                2 => list_brother_strikes(&conn, param_list),
                _ => slack::response("Invalid number of arguments")
            }
        },
        "remove" => remove_strike(&conn, param_list),
        "reset" => reset_strikes(&conn),
        _ => help()
    }
}

fn add_strike<'a>(conn: &StrikesDbConn, params: Vec<&str>) -> Json<SlackResponse<'a>> {
    todo!();
}

fn rank_strikes<'a>(conn: &StrikesDbConn) -> Json<SlackResponse<'a>> {
    todo!();
}

fn list_brother_strikes<'a>(conn: &StrikesDbConn, params: Vec<&str>) -> Json<SlackResponse<'a>> {
    todo!();
}

fn remove_strike<'a>(conn: &StrikesDbConn, params: Vec<&str>) -> Json<SlackResponse<'a>> {
    todo!();
}

fn reset_strikes<'a>(conn: &StrikesDbConn) -> Json<SlackResponse<'a>> {
    todo!();
}

fn help<'a>() -> Json<SlackResponse<'a>> {
    todo!();
}
