use std::env;
use std::path::PathBuf;

pub fn get_temp_dir() -> PathBuf {
    env::temp_dir().join("openjlc")
}
