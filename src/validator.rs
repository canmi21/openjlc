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

    for file in &files {
        if file.starts_with("Gerber_TopLayer") {
            if !files.iter().any(|f| f.starts_with("Gerber_TopSolderMaskLayer")) {
                log::log("! Missing Gerber_TopSolderMaskLayer");
            }
            if !files.iter().any(|f| f.starts_with("Gerber_TopSilkscreenLayer")) {
                log::log("! Missing Gerber_TopSilkscreenLayer");
            }
            if !files.iter().any(|f| f.starts_with("Gerber_TopPasteMaskLayer")) {
                log::log("! Missing Gerber_TopPasteMaskLayer, consider checking it");
            }
        }

        if file.starts_with("Gerber_BottomLayer") {
            if !files.iter().any(|f| f.starts_with("Gerber_BottomSolderMaskLayer")) {
                log::log("! Missing Gerber_BottomSolderMaskLayer");
            }
            if !files.iter().any(|f| f.starts_with("Gerber_BottomSilkscreenLayer")) {
                log::log("! Missing Gerber_BottomSilkscreenLayer");
            }
            if !files.iter().any(|f| f.starts_with("Gerber_BottomPasteMaskLayer")) {
                log::log("! Missing Gerber_BottomPasteMaskLayer, consider checking it");
            }
        }

        if file.starts_with("Gerber_InnerLayer") {
            let layer_number: Option<u32> = file
                .strip_prefix("Gerber_InnerLayer")
                .and_then(|s| s.parse().ok());
            if let Some(n) = layer_number {
                if n < 10 {
                    let next_layer = format!("Gerber_InnerLayer{}", n + 1);
                    if !files.iter().any(|f| f.starts_with(&next_layer)) {
                        log::log(&format!("! Missing {}, consider checking it", next_layer));
                    }
                }
            }
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
