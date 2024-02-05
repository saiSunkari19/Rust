use std::fmt::format;

use crate::models::general::llm::Message;

// Extend  ai function to get specific function
pub fn extend_ai_functions(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_functions = ai_func(func_input);


    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTION: You are function printer. you ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.",
        ai_functions, func_input
    );

    // Return Msg 
    Message{
        role: "system".to_string(),
        content: msg
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
       let extened =  extend_ai_functions(convert_user_input_to_goal, "dummy variable");
       dbg!(&extened);
       dbg!(extened.role);
    }
}
