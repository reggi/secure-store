use std::fmt;
use std::path::PathBuf;

use crate::git::{run_git_command, GIT_HASH_COMMAND};
use crate::secure::{dump_keys, find_generic_password};
use clap::ArgMatches;
use path_clean::PathClean;
use pathdiff::diff_paths;
use std::env;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Config {
    pub hash: String,
    pub file_path: PathBuf,
    pub key: String,
    pub relative_path: String,
}

impl Config {
    pub fn new(hash: String, file_path: PathBuf) -> Self {
        let key = format!("{}-{}-{}", KEY_PREFIX, &hash, file_path.display());
        let relative_path = file_path.display().to_string();
        Self {
            hash,
            file_path,
            key,
            relative_path,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hash: {}, File Path: {}, Key: {}",
            self.hash,
            self.file_path.display(),
            self.key
        )
    }
}

pub const KEY_PREFIX: &str = "secure-store";

pub async fn get_config(matches: &ArgMatches) -> Config {
    let file_path = matches
        .get_one::<String>("file")
        .expect("File path is required")
        .to_string();

    run_git_command(&["-v"])
        .await
        .expect("`git` not installed.");

    let abs_path = to_absolute_path(&file_path);

    let git_toplevel_future = run_git_command(&["rev-parse", "--show-toplevel"]);
    let git_hash_future = run_git_command(GIT_HASH_COMMAND);

    let (git_toplevel_result, git_hash_result) = tokio::join!(git_toplevel_future, git_hash_future);

    let toplevel = git_toplevel_result.expect("Current path is not a git repository.");
    let toplevel = Path::new(&toplevel);
    let hash = git_hash_result.expect("No First commit detected, please commit.");
    let relative_path: PathBuf =
        diff_paths(abs_path, toplevel).expect("Could not get relative path of file.");

    Config::new(hash, relative_path)
}

async fn get_files(hash: String) -> Vec<String> {
    let repo_key = format!("{}-{}-", KEY_PREFIX, hash);

    let keys = dump_keys().await.expect("Error dumping keys.");
    let filtered_keys = keys
        .into_iter()
        .filter(|key| key.starts_with(&repo_key))
        .collect::<Vec<_>>();

    filtered_keys
        .into_iter()
        .map(|key| key.strip_prefix(&repo_key).unwrap_or(&key).to_string())
        .collect::<Vec<String>>()
}

pub async fn get_configs() -> Vec<Config> {
    let hash = run_git_command(GIT_HASH_COMMAND)
        .await
        .expect("No First commit detected, please commit.");
    let files = get_files(hash.clone()).await;
    let mut configs = Vec::new();
    for key in files {
        let config = Config::new(hash.clone(), Path::new(&key).to_path_buf());
        configs.push(config);
    }
    configs
}

pub async fn write_file(file_path: PathBuf, key: String, force: bool) {
    // let config = get_config(matches).await;
    // let force = matches.get_flag("force");
    let exists = file_exists(file_path.clone()).await;
    if exists && !force {
        eprintln!("File already exists. Use --force to overwrite.");
        return;
    }
    let content = find_generic_password(&key)
        .await
        .expect("Error finding file.");
    let mut file = File::create(file_path.clone())
        .await
        .expect("Error creating file.");
    file.write_all(content.as_bytes())
        .await
        .expect("Error writing file.");
}

pub fn to_absolute_path(input_path: &str) -> PathBuf {
    let path = Path::new(input_path);
    if path.is_absolute() {
        // Path is already absolute, return it
        path.to_path_buf()
    } else {
        // Path is relative, construct an absolute path
        let current_dir = env::current_dir()
            .map_err(|e| e.to_string())
            .expect("Could not get current directory");
        current_dir.join(path).clean()
    }
}

pub async fn read_file_contents(file_path: PathBuf) -> std::io::Result<String> {
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

pub async fn file_exists(file_path: PathBuf) -> bool {
    File::open(file_path).await.is_ok()
}
