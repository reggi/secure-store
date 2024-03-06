use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};

pub fn file_arg() -> Arg {
    Arg::new("file")
        .required(true)
        .action(ArgAction::Set)
        .index(1)
}

pub fn get_flags() -> ArgMatches {
    ClapCommand::new("Secure Store")
        .version("1.0")
        .author("Your Name")
        .about("Stores, retrieves, and manages files with the macOS Keychain")
        .subcommand_required(true)
        .subcommand(
            ClapCommand::new("store")
                .about("Stores the specified file in the macOS Keychain")
                .arg(file_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            ClapCommand::new("cat")
                .about("Outputs the specified file content to stdout from the macOS Keychain")
                .arg(file_arg())
                .arg_required_else_help(true),
        )
        .subcommand(
            ClapCommand::new("write")
                .about("Writes the specified file content to disk from the macOS Keychain")
                .arg(file_arg())
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Forces overwriting the file if it already exists"),
                )
                .arg_required_else_help(true),
        )
        .arg_required_else_help(true)
        .get_matches()
}
