use std::fs;
use crate::config::get_target_dir;
use crate::log;

const REQUIRED_FILES: &[&str] = &[
    "PCB下单必读.txt",
    "header.yaml",
];

const REQUIRED_PREFIXES: &[&str] = &[
    "Gerber_BoardOutlineLayer",
    "Gerber_TopLayer",
    "Gerber_TopSolderMaskLayer",
];

pub fn validate_target_directory() -> bool {
    let target_dir = get_target_dir();
    if !target_dir.exists() {
        log::log(&format!("! Target directory not found: {:?}", target_dir));
        return false;
    }

    let files: Vec<String> = match fs::read_dir(&target_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
            .collect(),
        Err(e) => {
            log::log(&format!("! Failed to read target directory: {}", e));
            return false;
        }
    };

    let mut missing_files = Vec::new();

    for &required in REQUIRED_FILES {
        if !files.iter().any(|f| f == required) {
            missing_files.push(format!("! Missing {}", required));
        }
    }

    for &prefix in REQUIRED_PREFIXES {
        if !files.iter().any(|f| f.starts_with(prefix)) {
            missing_files.push(format!("! Missing {}.*", prefix));
        }
    }

    if missing_files.is_empty() {
        log::log("- Basic structure validation passed");
        true
    } else {
        for msg in missing_files {
            log::log(&msg);
        }
        false
    }
}