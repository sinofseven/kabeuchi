use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SavedMessage {
    System(String),
    Assistant(String),
    User(String),
}
