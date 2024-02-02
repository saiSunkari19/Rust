mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
fn main() {
    println!("Hello, world!");

    let response = get_user_response("What we are building today");

    dbg!(response);
}
