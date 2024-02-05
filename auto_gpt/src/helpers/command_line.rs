use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use serde::{Deserialize, Serialize};

use std::io::{stdin, stdout};

#[derive(PartialEq, Serialize, Debug, Deserialize)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statment: &str) {
        let mut stdout = stdout();

        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {} ", agent_pos);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statment);

        // Reset Color
        stdout.execute(ResetColor).unwrap();
    }
}

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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall.print_agent_message("Managaing", "Testing testing");
    }
}
