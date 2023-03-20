use clap::{ArgMatches, Command};

pub trait CommandDefinition {
    const NAME: &'static str;
    fn command() -> Command;
    fn run(args: &ArgMatches) -> Result<(), String>;
}
