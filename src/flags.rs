use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};

use crate::subc_def::{
    cat_command, delete_all_command, delete_command, hash_command, list_command, store_command,
    write_all_command, write_command,
};

pub fn file_arg() -> Arg {
    Arg::new("file")
        .required(true)
        .action(ArgAction::Set)
        .index(1)
}

pub fn force_arg() -> Arg {
    Arg::new("force")
        .long("force")
        .short('f')
        .action(ArgAction::SetTrue)
        .help("Forces overwriting the file if it already exists")
}

pub struct Commands;

impl Commands {
    pub const CAT: &'static str = "cat";
    pub const WRITE: &'static str = "write";
    pub const LIST: &'static str = "list";
    pub const STORE: &'static str = "store";
    pub const DELETE: &'static str = "delete";
    pub const DELETE_ALL: &'static str = "delete-all";
    pub const WRITE_ALL: &'static str = "write-all";
    pub const HASH: &'static str = "hash";
}

pub fn get_flags() -> ArgMatches {
    ClapCommand::new("Secure Store")
        .version("1.0")
        .author("Your Name")
        .about("Stores, retrieves, and manages files with the macOS Keychain")
        .subcommand_required(true)
        .subcommand(cat_command())
        .subcommand(delete_all_command())
        .subcommand(delete_command())
        .subcommand(hash_command())
        .subcommand(list_command())
        .subcommand(store_command())
        .subcommand(write_all_command())
        .subcommand(write_command())
        .arg_required_else_help(true)
        .get_matches()
}
