use crate::input::{get_confirm, get_input_number, get_input_with_null};
use crate::models::config::{Config, Profile, DEFAULT_MODEL};
use crate::traits::command_definition::CommandDefinition;
use clap::{arg, ArgMatches, Command};
use std::fmt::Display;
use std::str::FromStr;

pub struct  CmdConfigure;

impl CmdConfigure {
    const ID_PROFILE: &str = "profile";
}

impl CommandDefinition for CmdConfigure {
    const NAME: &'static str = "configure";

    fn command() -> Command {
        Command::new(Self::NAME).about("configure profile").arg(
            arg!(<PROFILE_NAME>)
                .id(Self::ID_PROFILE)
                .long(Self::ID_PROFILE)
                .short('p')
                .required(false)
                .default_value("default"),
        )
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let profile_name: &String = args.get_one(Self::ID_PROFILE).unwrap();

        let mut config = if let Some(config) = Config::load()? {
            config
        } else {
            Config::new()
        };

        let mut profile = if let Some(profile) = config.get_profile(profile_name, false)? {
            profile
        } else {
            Profile {
                name: profile_name.to_string(),
                source_profile: None,
                token: None,
                organization_id: None,
                use_history: None,
                model: None,
                temperature: None,
                top_p: None,
                max_tokens: None,
                presence_penalty: None,
                frequency_penalty: None,
                user: None,
            }
        };

        println!("profile name: {profile_name}");
        profile.token = input_str_with_null_and_default("OpenAI API key", &profile.token, true)?;
        profile.organization_id = input_str_with_null_and_default(
            "OpenAI Organization ID",
            &profile.organization_id,
            true,
        )?;
        profile.use_history = input_use_history(&profile.use_history)?;

        let default_model = if let Some(model) = &profile.model {
            Some(model.to_string())
        } else {
            Some(DEFAULT_MODEL.to_string())
        };
        profile.model = input_str_with_null_and_default("ChatGPT model", &default_model, false)?;

        let is_default_option = input_is_default_option()?;

        if is_default_option {
            profile.temperature =
                input_number_with_default("temperature", "float", &profile.temperature)?;
            profile.top_p = input_number_with_default("top_p", "float", &profile.top_p)?;
            profile.max_tokens =
                input_number_with_default("max_tokens", "integer", &profile.max_tokens)?;
            profile.presence_penalty =
                input_number_with_default("presence_penalty", "float", &profile.presence_penalty)?;
            profile.frequency_penalty = input_number_with_default(
                "frequency_penalty",
                "float",
                &profile.frequency_penalty,
            )?;
            profile.user = input_str_with_null_and_default("user", &profile.user, false)?;
        }

        config.upsert_profile(profile);
        config.save()
    }
}

fn input_str_with_null_and_default<T: Display>(
    message_without_separator: T,
    default_value: &Option<String>,
    is_masking_default: bool,
) -> Result<Option<String>, String> {
    let prefix = if let Some(default_value) = default_value {
        let masked_value = if is_masking_default {
            format!(
                "xxxxxxxxxx{}",
                default_value
                    .chars()
                    .enumerate()
                    .filter(|&(i, _)| default_value.len() - 5 <= i)
                    .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c))
            )
        } else {
            default_value.to_string()
        };
        format!(" [{masked_value}]")
    } else {
        "".to_string()
    };

    let mut value = get_input_with_null(format!("{message_without_separator}{prefix}: "))?;

    if value.is_none() && default_value.is_some() {
        value = default_value.clone();
    }
    Ok(value)
}

fn input_use_history(default_flag: &Option<bool>) -> Result<Option<bool>, String> {
    let default_flag = default_flag.unwrap_or(true);
    let choices = if default_flag { "[Y/n]" } else { "[y/N]" };

    let result = get_confirm(format!("use history {choices}: "), default_flag)?;
    if result.is_some() {
        Ok(result)
    } else {
        println!("\ninvalid input\n");
        input_use_history(&Some(default_flag))
    }
}

fn input_is_default_option() -> Result<bool, String> {
    let flg = get_confirm("change Chat Completion Option [y/N]: ", false)?;
    if let Some(flg) = flg {
        Ok(flg)
    } else {
        println!("\ninvalid input\n");
        input_is_default_option()
    }
}

fn input_number_with_default<T: Display + Copy, U: Display + Copy, V: FromStr + Display + Copy>(
    message_without_separator: T,
    type_name: U,
    default_value: &Option<V>,
) -> Result<Option<V>, String> {
    let prefix = if let Some(default_value) = default_value {
        format!(" [{default_value}]")
    } else {
        "".to_string()
    };
    let message = format!("{message_without_separator}{prefix}: ");
    let (err, value) = get_input_number(&message, type_name)?;
    if let Some(err) = err {
        println!("\n{err}\n");
        input_number_with_default(message_without_separator, type_name, default_value)
    } else if value.is_none() && default_value.is_some() {
        Ok(Some(default_value.unwrap()))
    } else {
        Ok(value)
    }
}
