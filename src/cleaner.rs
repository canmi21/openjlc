use std::fs;
use crate::log;
use crate::config::{get_temp_dir, get_target_dir};

pub fn clear_directories() {
    let temp_dir = get_temp_dir();
    let target_dir = get_target_dir();

    if temp_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&temp_dir) {
            log::log(&format!("! Failed to clear temp directory: {}", e));
        } else {
            log::log(&format!("+ Cleared temp directory: {:?}", temp_dir));
        }
    }

    if target_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&target_dir) {
            log::log(&format!("! Failed to clear target directory: {}", e));
        } else {
            log::log(&format!("+ Cleared target directory: {:?}", target_dir));
        }
    }
}