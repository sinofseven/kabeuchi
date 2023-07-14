use crate::models::api::{Message, RequestChatCompletion, ResponseChatCompletion, Role};
use crate::models::config::{Profile, DEFAULT_MODEL};
use crate::models::messages::{RawSavedMessage, SavedMessage};
use crate::path::{
    get_files_in_dir, get_path_profile_history_dir, get_path_profile_pre_messages_dir,
};

const URL_CHAT_COMPLETION: &str = "https://api.openai.com/v1/chat/completions";

fn get_pre_messages(profile_name: &str) -> Result<Vec<SavedMessage>, String> {
    let directory = get_path_profile_pre_messages_dir(profile_name)?;

    if !directory.is_dir() {
        return Ok(Vec::new());
    }

    let list_path = get_files_in_dir(&directory)?;

    let mut result = Vec::new();

    for path in list_path {
        let text = crate::fs::load_text(&path)?;
        if let Some(extension) = path.extension() {
            let extension = extension
                .to_str()
                .ok_or("failed to get string of extname".to_string())?
                .to_lowercase();
            if extension == "json" {
                let mut pre_messages: Vec<SavedMessage> =
                    serde_json::from_str(&text).map_err(|e| {
                        format!(
                            "failed to deserialize json: path={}, err={}",
                            path.display(),
                            e
                        )
                    })?;
                result.append(&mut pre_messages);
                continue;
            } else if extension == "yaml" || extension == "yml" {
                let raw_pre_messages: Vec<RawSavedMessage> =
                    serde_yaml::from_str(&text).map_err(|e| {
                        format!(
                            "failed to deserialize yaml: path={}, err={}",
                            path.display(),
                            e
                        )
                    })?;
                for message in raw_pre_messages {
                    result.push(message.try_into()?);
                }
                continue;
            }
        }
        let file_name = path
            .file_name()
            .ok_or("failed to get filename".to_string())?
            .to_str()
            .ok_or("failed to get string of filename".to_string())?
            .to_lowercase();

        if file_name.contains("system") {
            result.push(SavedMessage::System(text));
        } else if file_name.contains("assistant") {
            result.push(SavedMessage::Assistant(text));
        } else if file_name.contains("user") {
            result.push(SavedMessage::User(text))
        }
    }

    Ok(result)
}

fn get_histories(profile_name: &str) -> Result<Vec<SavedMessage>, String> {
    let mut result = Vec::new();
    let directory = get_path_profile_history_dir(profile_name)?;

    let list_path = get_files_in_dir(&directory)?;

    for path in list_path {
        let text = crate::fs::load_text(&path)?;

        let histories: Vec<RawSavedMessage> = serde_yaml::from_str(&text).map_err(|e| {
            format!(
                "failed to deserialize yaml: path={}, err={}",
                path.display(),
                e
            )
        })?;

        for message in histories {
            result.push(message.try_into()?);
        }
    }

    Ok(result)
}

fn save_history(
    profile_name: &str,
    message: &str,
    response: &ResponseChatCompletion,
) -> Result<(), String> {
    let directory = get_path_profile_history_dir(profile_name)?;
    let path = directory.join(format!("{}.yaml", response.created));

    let answer = response.get_assistant_message();

    let history: Vec<RawSavedMessage> = vec![
        RawSavedMessage {
            system: None,
            assistant: None,
            user: Some(message.to_string()),
        },
        RawSavedMessage {
            system: None,
            assistant: Some(answer),
            user: None,
        },
    ];

    let text =
        serde_yaml::to_string(&history).map_err(|e| format!("failed to serialize yaml: {e}"))?;

    crate::fs::save_text(&path, &text)
}

pub fn call_chat_completion(profile: &Profile, message: &str) -> Result<(), String> {
    let mut messages = Vec::new();

    let use_history = profile.use_history.unwrap_or(true);

    let mut pre_messages = get_pre_messages(&profile.name)?
        .iter()
        .map(|m| m.into())
        .collect();
    messages.append(&mut pre_messages);

    if use_history {
        let mut histories = get_histories(&profile.name)?
            .iter()
            .map(|m| m.into())
            .collect();
        messages.append(&mut histories);
    }

    messages.push(Message {
        role: Role::User,
        content: message.to_string(),
    });

    let request = RequestChatCompletion {
        model: match &profile.model {
            Some(model) => model.to_string(),
            None => DEFAULT_MODEL.to_string(),
        },
        messages,
        temperature: profile.temperature,
        top_p: profile.top_p,
        max_tokens: profile.max_tokens,
        presence_penalty: profile.presence_penalty,
        frequency_penalty: profile.frequency_penalty,
        user: match &profile.user {
            Some(user) => Some(user.clone()),
            None => Some(profile.name.clone()),
        },
    };

    let body =
        serde_json::to_string(&request).map_err(|e| format!("failed to serialize json: {e}"))?;

    let token = profile
        .api_key
        .clone()
        .ok_or("failed to get token (token is empty)")?;

    let mut request = ureq::post(URL_CHAT_COMPLETION)
        .set("Content-Type", "application/json")
        .set("Authorization", format!("Bearer {token}").as_ref());

    if let Some(organization_id) = &profile.organization_id {
        request = request.set("OpenAI-Organization", organization_id);
    }

    let response = request.send_string(&body);

    if let Err(err) = response {
        let e_message = match err {
            ureq::Error::Status(code, response) => format!(
                "code={}, body={}",
                code,
                response
                    .into_string()
                    .map_err(|e| format!("failed to parse response body: {e}"))?
            ),
            ureq::Error::Transport(e) => format!("failed to call chat completion: {e}"),
        };
        return Err(format!("failed to call chat completion: {e_message}"));
    }

    let text = response
        .unwrap()
        .into_string()
        .map_err(|e| format!("failed to parse response body: {e}"))?;
    let response: ResponseChatCompletion =
        serde_json::from_str(&text).map_err(|e| format!("failed to deserialize json: {e}"))?;

    if use_history {
        save_history(&profile.name, message, &response)?;
    }

    println!("\n{}\n", response.get_assistant_message());
    Ok(())
}
