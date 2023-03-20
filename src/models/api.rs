use crate::models::messages::SavedMessage;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl From<&SavedMessage> for Message {
    fn from(value: &SavedMessage) -> Self {
        match value {
            SavedMessage::System(text) => Message {
                role: Role::System,
                content: text.clone(),
            },
            SavedMessage::Assistant(text) => Message {
                role: Role::Assistant,
                content: text.clone(),
            },
            SavedMessage::User(text) => Message {
                role: Role::User,
                content: text.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    // n: Option<u32>,
    // stream: Option<bool>,
    // stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    // logit_bias: ??
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: Option<TokenUsage>,
    pub choices: Vec<Choice>,
}

impl ResponseChatCompletion {
    pub fn get_assistant_message(&self) -> String {
        self.choices[0].message.content.clone()
    }
}
