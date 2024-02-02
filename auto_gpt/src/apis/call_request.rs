use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client};
use std::env;

pub async fn call_gpt(msgs: Vec<Message>){
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
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create Client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-4-turbo-preview".to_string(),
        messages: msgs,
        temperature: 0.1,
    };

    // Trouble shooting

    let response = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();


    dbg!(response.text().await.unwrap());
}


#[cfg(test)]
mod test{

    use super::*;

    #[tokio::test]
    async fn test_call_to_openai(){
        let msg = Message{
            role:"user".to_string(),
            content: "Hi There, give me short replay".to_string()
        };

        let msgs = vec![msg];

        call_gpt(msgs).await;
    }
}