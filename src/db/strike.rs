use crate::StrikesDbConn;

pub fn strike_handler<'a>(conn: &StrikesDbConn, params: String) -> &'a str {
    let param_list: Vec<&str> = params.split_whitespace().collect();

    match param_list[0] {
        "add" => add_strike(&conn, param_list),
        "list" => {
            match param_list.len() {
                1 => rank_strikes(&conn),
                2 => list_brother_strikes(&conn, param_list),
                _ => "Invalid number of arguments"
            }
        },
        "remove" => remove_strike(&conn, param_list),
        "reset" => reset_strikes(&conn),
        _ => help()
    }
}

fn add_strike<'a>(conn: &StrikesDbConn, params: Vec<&str>) -> &'a str {
    todo!();
}

fn rank_strikes<'a>(conn: &StrikesDbConn) -> &'a str {
    todo!();
}

fn list_brother_strikes<'a>(conn: &StrikesDbConn, params: Vec<&str>) -> &'a str {
    todo!();
}

fn remove_strike<'a>(conn: &StrikesDbConn, params: Vec<&str>) -> &'a str {
    todo!();
}

fn reset_strikes<'a>(conn: &StrikesDbConn) -> &'a str {
    todo!();
}

fn help<'a>() -> &'a str {
    todo!();
}
