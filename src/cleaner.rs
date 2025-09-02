/* src/cleaner.rs */

use crate::config::{get_target_dir, get_temp_dir};
use crate::log;
use std::fs;

pub fn clear_directories() {
    let temp_dir = get_temp_dir();
    let target_dir = get_target_dir();

    if temp_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&temp_dir) {
            log::log(&format!("! Failed to clear temp directory: {}", e));
        } else {
            log::log(&format!(
                "- Cleared temp directory: '{}'",
                temp_dir.display()
            ));
        }
    }

    if target_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&target_dir) {
            log::log(&format!("! Failed to clear target directory: {}", e));
        } else {
            log::log(&format!(
                "- Cleared target directory: '{}'",
                target_dir.display()
            ));
        }
    }
}
