mod api;
mod cmd;
mod fs;
mod input;
mod models;
mod path;
mod traits;

use crate::cmd::*;
use crate::traits::command_definition::CommandDefinition;

use clap::command;

fn main() -> Result<(), String> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(CmdPrompt::command())
        .subcommand(CmdPath::command())
        .subcommand(CmdConfigure::command())
        .get_matches();

    match matches.subcommand() {
        Some((CmdPrompt::NAME, args)) => CmdPrompt::run(args),
        Some((CmdPath::NAME, args)) => CmdPath::run(args),
        Some((CmdConfigure::NAME, args)) => CmdConfigure::run(args),
        _ => unreachable!(),
    }
}
