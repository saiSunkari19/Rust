use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout};

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Red)).unwrap();
    println!("");
    println!("{}", question);
    stdout.execute(ResetColor).unwrap();

    // Read stdin
    let mut user_response = String::new();
    stdin().read_line(&mut user_response);

    return user_response.trim().to_string();
}
