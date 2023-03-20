use crate::models::config::Config;
use crate::traits::command_definition::CommandDefinition;
use clap::{arg, ArgMatches, Command};

pub struct CmdPrompt;

impl CmdPrompt {
    const ID_PROFILE: &'static str = "profile";
    const ID_MESSAGE: &'static str = "message";
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
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let profile_name: &String = args.get_one(Self::ID_PROFILE).unwrap();
        let message: Option<&String> = args.get_one(Self::ID_MESSAGE);

        let config = Config::load()?.ok_or("not found config file")?;
        let profile = config
            .get_profile(profile_name, true)?
            .ok_or(format!("Profile \"{profile_name}\" is not exists"))?;

        let message = if let Some(message) = message {
            message.clone()
        } else {
            get_message_from_editor()?
        };

        crate::api::call_chat_completion(&profile, &message)?;

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
