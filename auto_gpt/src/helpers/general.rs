use std::fmt::format;

use serde::de::DeserializeOwned;

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;

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
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn ai_task_request(
    msg_contest: String,
    agent_pos: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI Function
    let extend_msg = extend_ai_functions(function_pass, &msg_contest);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_pos, agent_operation);

    // Get LLM response

    let llm_response_res = call_gpt(vec![extend_msg.clone()]).await;

    match llm_response_res {
        Ok(llm_res) => llm_res,
        Err(_) => call_gpt(vec![extend_msg])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

// Perform call to LLM GPT- Decoded
pub async fn ai_task_request_decode<T: DeserializeOwned>(
    msg_contest: String,
    agent_pos: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let res = ai_task_request(msg_contest, agent_pos, agent_operation, function_pass).await;

    let decoded_response =
        serde_json::from_str(res.as_str()).expect("Failed to decode ai response from serde_json");

    decoded_response
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
        let extened = extend_ai_functions(convert_user_input_to_goal, "dummy variable");
        dbg!(&extened);
        dbg!(extened.role);
    }

    #[tokio::test]
    pub async fn test_ai_task_request() {
        let ai_func_params = "Build a prediction market that similar to pancake swap prediction maket on top of Binace Blockchain".to_string();

        let res = ai_task_request(
            ai_func_params,
            "Managing Agent",
            "Defining User requirements",
            convert_user_input_to_goal,
        )
        .await;

        dbg!(res);
    }
}
