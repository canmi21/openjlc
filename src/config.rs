use std::env;
use std::path::PathBuf;

pub fn get_temp_dir() -> PathBuf {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".canmi/openjlc/temp")
}
