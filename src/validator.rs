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

pub static mut LAYER_COUNT: u32 = 0;

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
    let mut has_top_copper = false;
    let mut has_bottom_copper = false;
    let mut has_inner_layers = vec![false; 10];

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
            has_top_copper = true;
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
            has_bottom_copper = true;
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
                if n <= 10 {
                    has_inner_layers[n as usize - 1] = true;
                }
            }
        }
    }

    if has_top_copper && has_inner_layers[0] && !has_bottom_copper {
        log::log("! Missing Bottom Copper layer due to Top Copper and Inner Layer 1 presence");
        std::process::exit(0);
    }

    let mut layer_count = 0;
    if has_top_copper {
        layer_count += 1;
    }
    if has_bottom_copper {
        layer_count += 1;
    }
    layer_count += has_inner_layers.iter().filter(|&&x| x).count() as u32;

    unsafe {
        LAYER_COUNT = layer_count;
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
