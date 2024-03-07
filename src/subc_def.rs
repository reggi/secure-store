use crate::flags::{file_arg, force_arg, Commands};
use clap::Command as ClapCommand;

pub fn cat_command() -> ClapCommand {
    ClapCommand::new(Commands::CAT)
        .about("Outputs the specified file content to stdout from the macOS Keychain")
        .arg(file_arg())
        .arg_required_else_help(true)
}

pub fn delete_command() -> ClapCommand {
    ClapCommand::new(Commands::DELETE)
        .about("Delete a file from the macOS Keychain.")
        .arg(file_arg())
        .arg_required_else_help(true)
}

pub fn delete_all_command() -> ClapCommand {
    ClapCommand::new(Commands::DELETE_ALL)
        .about("Deletes all files for this repo from the macOS Keychain.")
}

pub fn hash_command() -> ClapCommand {
    ClapCommand::new(Commands::HASH).about("Prints this repository's hash.")
}

pub fn list_command() -> ClapCommand {
    ClapCommand::new(Commands::LIST)
        .about("Lists all the files for this repo that are in macOS Keychain.")
}

pub fn store_command() -> ClapCommand {
    ClapCommand::new(Commands::STORE)
        .alias("add")
        .alias("create")
        .about("Adds the specified file in the macOS Keychain")
        .arg(file_arg())
        .arg_required_else_help(true)
}

pub fn write_command() -> ClapCommand {
    ClapCommand::new(Commands::WRITE)
        .about("Writes the specified file content to disk from the macOS Keychain")
        .arg(file_arg())
        .arg(force_arg())
        .arg_required_else_help(true)
}

pub fn write_all_command() -> ClapCommand {
    ClapCommand::new(Commands::WRITE_ALL)
        .about("Writes all files for this repo from the macOS Keychain.")
        .arg(force_arg())
}
