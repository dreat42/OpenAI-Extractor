#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod base;
mod models;
mod requests;

use base::command_line::get_user_response;

fn main() {
    let usr_req: String = get_user_response("What webserver are we building today");
    dbg!(usr_req);
}
