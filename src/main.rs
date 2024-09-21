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
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    let usr_req: String = get_user_response("What website are we building today");
   
   let mut managing_agend:ManagingAgent = ManagingAgent::new(usr_req)
    .await
    .expect("Error creating agent");
   
    dbg!(managing_agend);
}
