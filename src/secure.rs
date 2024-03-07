use regex::Regex;
use std::env;
use tokio::io;
use tokio::process::Command;

pub async fn add_generic_password(key: &str, content: &str) -> std::io::Result<()> {
    let user = env::var("USER").expect("Could not get current user.");

    Command::new("security")
        .args(&[
            "add-generic-password",
            "-a",
            &user,
            "-s",
            key,
            "-w",
            content,
            "-U",
        ])
        .output()
        .await?;

    // // Print stdout and stderr regardless of the command's success
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    //   if output.status.success() {
    //       println!("Password added successfully.");
    //   } else {
    //       eprintln!("Error adding password.");
    //   }

    Ok(())
}

pub async fn find_generic_password(key: &str) -> std::io::Result<String> {
    let user = env::var("USER").expect("Could not get current user.");

    let output = Command::new("security")
        .args(&[
            "find-generic-password",
            "-a",
            &user,
            "-s",
            key,
            "-w", // This option tells `security` to only output the password itself
        ])
        .output()
        .await?;

    // Check if the command was successful and parse the output
    if output.status.success() {
        let password = String::from_utf8_lossy(&output.stdout).trim().to_string();
        //   println!("Password found successfully.");
        Ok(password)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        //   eprintln!("Error finding password: {}", error_message);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Error finding password: {}", error_message),
        ))
    }
}

pub async fn dump_keys() -> std::io::Result<Vec<String>> {
    let output = Command::new("security")
        .args(&["dump-keychain"])
        .output()
        .await?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let regex = Regex::new(r#"^\s{4}"svce"<blob>="(.+)""#).expect("Invalid regex");

        let keys: Vec<String> = output_str
            .lines()
            .filter_map(|line| regex.captures(line))
            .filter_map(|caps| caps.get(1))
            .map(|match_| match_.as_str().to_string())
            .collect();

        Ok(keys)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Error dumping keychain: {}", error_message),
        ))
    }
}

pub async fn delete_generic_password(key: &str) -> std::io::Result<()> {
    let user = env::var("USER").expect("Could not get current user.");

    let output = Command::new("security")
        .args(&["delete-generic-password", "-a", &user, "-s", key])
        .output()
        .await?;

    if output.status.success() {
        // Uncomment this line if you want to print success confirmation
        // println!("Password removed successfully.");
    } else {
        // This part captures and returns any error message from stderr
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Error removing password: {}", error_message),
        ));
    }

    Ok(())
}
