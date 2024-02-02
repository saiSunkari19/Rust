use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
   pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  ChatCompletion {
    pub model:String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}