
use path_clean::PathClean;
use std::env;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub fn to_absolute_path(input_path: &str) -> Result<PathBuf, String> {
  let path = Path::new(input_path);
  if path.is_absolute() {
      // Path is already absolute, return it
      Ok(path.to_path_buf())
  } else {
      // Path is relative, construct an absolute path
      let current_dir = env::current_dir().map_err(|e| e.to_string())?;
      Ok(current_dir.join(path).clean())
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
