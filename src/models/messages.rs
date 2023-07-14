use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SavedMessage {
    System(String),
    Assistant(String),
    User(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawSavedMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl From<SavedMessage> for RawSavedMessage {
    fn from(value: SavedMessage) -> Self {
        let mut result = RawSavedMessage {
            system: None,
            assistant: None,
            user: None,
        };

        match value {
            SavedMessage::System(text) => result.system = Some(text),
            SavedMessage::Assistant(text) => result.assistant = Some(text),
            SavedMessage::User(text) => result.user = Some(text),
        }

        result
    }
}

impl TryFrom<RawSavedMessage> for SavedMessage {
    type Error = String;

    fn try_from(value: RawSavedMessage) -> Result<Self, Self::Error> {
        if let Some(text) = value.system {
            Ok(Self::System(text))
        } else if let Some(text) = value.assistant {
            Ok(Self::Assistant(text))
        } else if let Some(text) = value.user {
            Ok(Self::User(text))
        } else {
            Err("no message".to_string())
        }
    }
}
