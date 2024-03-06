mod secure;
mod git;
mod fs;
mod flags;
mod config;

use pathdiff::diff_paths;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use git::run_git_command;
use fs::{file_exists, read_file_contents, to_absolute_path};
use secure::{add_generic_password, find_generic_password};
use flags::get_flags;
use config::Config;

#[tokio::main]
async fn main() {
    let matches = get_flags();

    let (subcommand, sub_m) = matches.subcommand().unwrap(); // It's safe due to subcommand_required(true)
    let file_path = sub_m
        .get_one::<String>("file")
        .expect("File path is required")
        .to_string();

    run_git_command(&["-v"])
        .await
        .expect("`git` not installed.");

    let abs_path = to_absolute_path(&file_path).expect("Invalid file path.");

    let git_toplevel_future = run_git_command(&["rev-parse", "--show-toplevel"]);
    let git_hash_future = run_git_command(&["rev-list", "--max-parents=0", "HEAD"]);

    let (
        git_toplevel_result, 
        git_hash_result,
    ) = tokio::join!(git_toplevel_future, git_hash_future);

    let toplevel = git_toplevel_result.expect("Current path is not a git repository.");
    let toplevel = Path::new(&toplevel);
    let hash = git_hash_result.expect("No First commit detected, please commit.");
    let relative_path: PathBuf =
        diff_paths(abs_path, toplevel).expect("Could not get relative path of file.");
    
    let key = format!("secure-store-{}-{}", hash, relative_path.display());

    let config = Config {
        hash,
        file_path: relative_path,
        key,
    };

    match subcommand {
        "store" => store_file(config).await,
        "cat" => cat_file(config).await,
        "write" => {
            let force = sub_m.get_flag("force");
            write_file(config, force).await;
        }
        _ => unreachable!("Exhaustive subcommand pattern matching; cannot happen"),
    }
}

async fn store_file(config: Config) {
    let file = read_file_contents(config.file_path.clone()).await.expect("Could not read file contents.");
    add_generic_password(&config.key, &file).await.expect("Error storing file.");
}

async fn cat_file(config: Config) {
    let file = find_generic_password(&config.key).await.expect("Error finding file.");
    print!("{}", file);
}

async fn write (config: Config) {
    let content = find_generic_password(&config.key).await.expect("Error finding file.");
    let mut file = File::create(config.file_path.clone()).await.expect("Error creating file.");
    file.write_all(content.as_bytes()).await.expect("Error writing file.");
}

async fn write_file(config: Config, force: bool) {
    let exists = file_exists(config.file_path.clone()).await;
    if exists {
        if !force {
            eprintln!("File already exists. Use --force to overwrite.");
        } else {
            write(config).await;
        }
    } else {
        write(config).await;
    }
}
