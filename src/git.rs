use tokio::io;
use tokio::process::Command;

pub const GIT_HASH_COMMAND: &[&str] = &["rev-list", "--max-parents=0", "HEAD"];

pub async fn run_git_command(args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new("git").args(args).output().await?;

    if !output.status.success() {
        // Convert the command's stderr output to an io::Error
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(io::Error::new(io::ErrorKind::Other, stderr));
    }

    let output_string = String::from_utf8_lossy(&output.stdout)
        .into_owned()
        .trim()
        .to_string();

    Ok(output_string)
}
