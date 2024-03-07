mod flags;
mod git;
mod secure;
mod subc_def;
mod subc_op;
mod utils;

use flags::{get_flags, Commands};
use subc_op::{cat, delete, delete_all, hash, list_files, store, write, write_all};

#[tokio::main]
async fn main() {
    let matches: clap::ArgMatches = get_flags();
    let (subcommand, arg_matches) = matches.subcommand().unwrap(); // It's safe due to subcommand_required(true)
    match subcommand {
        Commands::CAT => cat(arg_matches).await,
        Commands::DELETE => delete(arg_matches).await,
        Commands::DELETE_ALL => delete_all().await,
        Commands::HASH => hash().await,
        Commands::LIST => list_files().await,
        Commands::STORE => store(arg_matches).await,
        Commands::WRITE => write(arg_matches).await,
        Commands::WRITE_ALL => write_all(arg_matches).await,
        _ => unreachable!("Exhaustive subcommand pattern matching; cannot happen"),
    }
}
