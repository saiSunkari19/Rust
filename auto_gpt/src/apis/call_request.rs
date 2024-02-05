use crate::models::general::llm::{APIChoice, APIMessage, APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client};
use std::env;

pub async fn call_gpt(msgs: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API key infomation
    let api_key = env::var("OPENAI_KEY").expect("api key not found in env");
    let api_org = env::var("OPENAI_ORG_ID").expect("org id not found in env");

    // endpint
    let url = "https://api.openai.com/v1/chat/completions";

    // create header
    let mut headers = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create Client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-4-turbo-preview".to_string(),
        messages: msgs,
        temperature: 0.1,
    };

    // Let Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Trouble shooting

    // let response = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(response.text().await.unwrap());
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod test {

    use std::clone;

    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let msg = Message {
            role: "user".to_string(),
            content: "Hi There, give me short replay".to_string(),
        };

        let msgs = vec![msg];

        let res = call_gpt(msgs).await;

        //    if let Ok(res_str) = res {
        //         dbg!(res_str);
        //         assert!(true)
        //    }else {
        //        assert!(false)
        //    }

        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true)
            }
            Err(_) => {
                assert!(false)
            }
        }
    }
}
