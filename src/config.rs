use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub hash: String,
    pub file_path: PathBuf,
    pub key: String,
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
