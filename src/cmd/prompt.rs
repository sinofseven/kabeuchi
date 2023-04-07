use crate::models::config::Config;
use crate::traits::command_definition::CommandDefinition;
use clap::{arg, ArgMatches, Command};

pub struct CmdPrompt;

impl CmdPrompt {
    const ID_PROFILE: &'static str = "profile";
    const ID_MESSAGE: &'static str = "message";
    const ID_TEMPERATURE: &'static str = "temperature";
    const ID_TOP_P: &'static str = "top-p";
    const ID_MAX_TOKENS: &'static str = "max-tokens";
    const ID_PRESENCE_PENALTY: &'static str = "presence-penalty";
    const ID_FREQUENCY_PENALTY: &'static str = "frequency-penalty";
    const ID_USER: &'static str = "user";
}

impl CommandDefinition for CmdPrompt {
    const NAME: &'static str = "prompt";

    fn command() -> Command {
        Command::new(Self::NAME)
            .about("call ChatGPT API")
            .arg(
                arg!(<PROFILE_NAME>)
                    .id(Self::ID_PROFILE)
                    .long(Self::ID_PROFILE)
                    .short('p')
                    .required(false)
                    .default_value("default"),
            )
            .arg(
                arg!(<MESSAGE>)
                    .id(Self::ID_MESSAGE)
                    .long(Self::ID_MESSAGE)
                    .short('m')
                    .required(false),
            )
            .arg(
                arg!(<TEMPERATURE>)
                    .id(Self::ID_TEMPERATURE)
                    .long(Self::ID_TEMPERATURE)
                    .required(false),
            )
            .arg(
                arg!(<TOP_P>)
                    .id(Self::ID_TOP_P)
                    .long(Self::ID_TOP_P)
                    .required(false),
            )
            .arg(
                arg!(<MAX_TOKENS>)
                    .id(Self::ID_MAX_TOKENS)
                    .long(Self::ID_MAX_TOKENS)
                    .required(false),
            )
            .arg(
                arg!(<PRESENCE_PENALTY>)
                    .id(Self::ID_PRESENCE_PENALTY)
                    .long(Self::ID_PRESENCE_PENALTY)
                    .required(false),
            )
            .arg(
                arg!(<FREQUENCY_PENALTY>)
                    .id(Self::ID_FREQUENCY_PENALTY)
                    .long(Self::ID_FREQUENCY_PENALTY)
                    .required(false),
            )
            .arg(
                arg!(<USER>)
                    .id(Self::ID_USER)
                    .long(Self::ID_USER)
                    .required(false),
            )
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let profile_name: &String = args.get_one(Self::ID_PROFILE).unwrap();
        let message: Option<&String> = args.get_one(Self::ID_MESSAGE);
        let temperature: Option<&f32> = args.get_one(Self::ID_TEMPERATURE);
        let top_p: Option<&f32> = args.get_one(Self::ID_TOP_P);
        let max_token: Option<&u64> = args.get_one(Self::ID_MAX_TOKENS);
        let presence_penalty: Option<&f32> = args.get_one(Self::ID_PRESENCE_PENALTY);
        let frequency_penalty: Option<&f32> = args.get_one(Self::ID_FREQUENCY_PENALTY);
        let user: Option<&String> = args.get_one(Self::ID_USER);

        let config = Config::load()?.ok_or("not found config file")?;
        let mut profile = config
            .get_profile(profile_name, true)?
            .ok_or(format!("Profile \"{profile_name}\" is not exists"))?;

        let message = if let Some(message) = message {
            message.clone()
        } else {
            get_message_from_editor()?
        };

        if let Some(temperature) = temperature {
            profile.temperature = Some(temperature.clone());
        }

        if let Some(top_p) = top_p {
            profile.top_p = Some(top_p.clone());
        }

        if let Some(max_token) = max_token {
            profile.max_tokens = Some(max_token.clone());
        }

        if let Some(presence_penalty) = presence_penalty {
            profile.presence_penalty = Some(presence_penalty.clone());
        }

        if let Some(frequency_penalty) = frequency_penalty {
            profile.frequency_penalty = Some(frequency_penalty.clone());
        }

        if let Some(user) = user {
            profile.user = Some(user.to_string());
        }

        // crate::api::call_chat_completion(&profile, &message)?;

        println!("{:?}", profile);

        Ok(())
    }
}

fn get_message_from_editor() -> Result<String, String> {
    let path = crate::path::get_path_editting_message_file()?;

    let editor = std::env::var("EDITOR")
        .map_err(|e| format!("failed to get environment variable \"EDITOR\": {e}"))?;

    crate::fs::save_text(&path, "")?;

    std::process::Command::new(editor)
        .arg(&path)
        .spawn()
        .map_err(|e| format!("failed to spawn editor: {e}"))?
        .wait()
        .map_err(|e| format!("failed to edit message: {e}"))?;

    let text = crate::fs::load_text(&path)?;

    if text.is_empty() {
        Err("Aborting prompt due to empty message".to_string())
    } else {
        Ok(text.trim().to_string())
    }
}
