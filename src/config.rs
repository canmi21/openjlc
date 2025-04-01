use std::env;
use std::path::PathBuf;
use openjlc::config::get_temp_dir;

pub fn get_temp_dir() -> PathBuf {
    env::temp_dir().join("openjlc")
}
