use crate::git::{run_git_command, GIT_HASH_COMMAND};
use crate::secure::{add_generic_password, delete_generic_password, find_generic_password};
use crate::utils::{get_config, get_configs, read_file_contents, write_file};
use clap::ArgMatches;

pub async fn cat(matches: &ArgMatches) {
    let config = get_config(matches).await;
    let file = find_generic_password(&config.key)
        .await
        .expect("Error finding file.");
    print!("{}", file);
}

pub async fn delete(matches: &ArgMatches) {
    let config = get_config(matches).await;
    delete_generic_password(&config.key)
        .await
        .expect("Error deleting file.");
}

pub async fn delete_all() {
    for config in get_configs().await {
        delete_generic_password(&config.key)
            .await
            .expect("Error deleting file.");
    }
}

pub async fn hash() {
    let hash = run_git_command(GIT_HASH_COMMAND)
        .await
        .expect("No First commit detected, please commit.");
    println!("{}", hash);
}

pub async fn list_files() {
    let configs = get_configs().await;
    for config in configs {
        println!("{}", config.relative_path);
    }
}

pub async fn store(matches: &ArgMatches) {
    let config = get_config(matches).await;
    let file = read_file_contents(config.file_path.clone())
        .await
        .expect("Could not read file contents.");
    add_generic_password(&config.key, &file)
        .await
        .expect("Error storing file.");
}

pub async fn write(matches: &ArgMatches) {
    let config = get_config(matches).await;
    let force = matches.get_flag("force");
    write_file(config.file_path, config.key, force).await;
}

pub async fn write_all(matches: &ArgMatches) {
    let force = matches.get_flag("force");
    for config in get_configs().await {
        write_file(config.file_path, config.key, force).await;
    }
}
