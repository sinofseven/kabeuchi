use crate::path;
use crate::traits::command_definition::CommandDefinition;
use clap::{arg, ArgMatches, Command};

pub struct CmdPath;
struct SubCmdConfigDir;
struct SubCmdConfigFile;
struct SubCmdPreMessagesDir;
struct SubCmdHistoryDir;

impl CommandDefinition for CmdPath {
    const NAME: &'static str = "path";

    fn command() -> Command {
        Command::new(Self::NAME)
            .about("show path")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(SubCmdConfigDir::command())
            .subcommand(SubCmdConfigFile::command())
            .subcommand(SubCmdPreMessagesDir::command())
            .subcommand(SubCmdHistoryDir::command())
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            Some((SubCmdConfigDir::NAME, sub_args)) => SubCmdConfigDir::run(sub_args),
            Some((SubCmdConfigFile::NAME, sub_args)) => SubCmdConfigFile::run(sub_args),
            Some((SubCmdPreMessagesDir::NAME, sub_args)) => SubCmdPreMessagesDir::run(sub_args),
            Some((SubCmdHistoryDir::NAME, sub_args)) => SubCmdHistoryDir::run(sub_args),
            _ => unreachable!("This is Bug."),
        }
    }
}

impl CommandDefinition for SubCmdConfigDir {
    const NAME: &'static str = "config-dir";

    fn command() -> Command {
        Command::new(Self::NAME).about("show path of config dir")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        println!("{}", path::get_path_config_dir()?.display());
        Ok(())
    }
}

impl CommandDefinition for SubCmdConfigFile {
    const NAME: &'static str = "config-file";

    fn command() -> Command {
        Command::new(Self::NAME).about("show path of config file")
    }

    fn run(_args: &ArgMatches) -> Result<(), String> {
        println!("{}", path::get_path_config_file()?.display());
        Ok(())
    }
}

impl SubCmdPreMessagesDir {
    const KEY_PROFILE_NAME: &'static str = "profile_name";
}

impl CommandDefinition for SubCmdPreMessagesDir {
    const NAME: &'static str = "pre-messages-dir";

    fn command() -> Command {
        Command::new(Self::NAME)
            .about("show path of pre messages dir")
            .long_about("show path of pre messages direcotry (pre messages = Messages to be passed in advance when hitting ChatGPT)")
            .arg(arg!(<PROFILE_NAME>).id(Self::KEY_PROFILE_NAME))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let profile_name: Option<&String> = args.get_one(Self::KEY_PROFILE_NAME);
        let mut directory = path::get_path_pre_messages_dir()?;
        if let Some(profile_name) = profile_name {
            directory = directory.join(profile_name);
        }
        println!("{}", directory.display());

        Ok(())
    }
}

impl SubCmdHistoryDir {
    const KEY_PROFILE_NAME: &'static str = "profile_name";
}

impl CommandDefinition for SubCmdHistoryDir {
    const NAME: &'static str = "history-dir";

    fn command() -> Command {
        Command::new(Self::NAME)
            .about("show path of history dir")
            .arg(arg!(<PROFILE_NAME>).id(Self::KEY_PROFILE_NAME))
    }

    fn run(args: &ArgMatches) -> Result<(), String> {
        let profile_name: Option<&String> = args.get_one(Self::KEY_PROFILE_NAME);
        let mut directory = path::get_path_history_dir()?;
        if let Some(profile_name) = profile_name {
            directory = directory.join(profile_name);
        }
        println!("{}", directory.display());

        Ok(())
    }
}
