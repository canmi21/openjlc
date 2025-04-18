use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use crate::error::report_error;
use crate::log;

#[derive(Debug, Clone, Copy)]
pub enum EDATool {
    Altium,
    KiCad,
    LCEDA,
    Unknown,
}

fn check_file_signature(path: &Path, keyword: &str) -> bool {
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);
        return reader
            .lines()
            .take(100)
            .filter_map(|line| line.ok())
            .any(|line| line.to_lowercase().contains(&keyword.to_lowercase()));
    }
    false
}

fn has_no_exclusion(path: &Path, excludes: &[&str]) -> bool {
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);
        return !reader
            .lines()
            .take(100)
            .filter_map(|line| line.ok())
            .any(|line| {
                let line_lower = line.to_lowercase();
                excludes.iter().any(|&ex| line_lower.contains(&ex.to_lowercase()))
            });
    }
    true
}

pub fn identify_eda_files(temp_dir: &Path) -> io::Result<(PathBuf, EDATool)> {
    let candidates = ["gto", "gtl", "gbl"];
    let kicad_names = ["Edge_Cuts", "F_Cu", "F_Mask"];
    let keywords = ["Altium", "KiCad", "EasyEDA", "OpenJLC"];

    for entry in temp_dir.read_dir()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if candidates.contains(&ext_lower.as_str()) || kicad_names.iter().any(|&name| file_name.contains(name)) {
                    if check_file_signature(&path, "Altium") && has_no_exclusion(&path, &["EasyEDA", "OpenJLC"]) {
                        return Ok((path, EDATool::Altium));
                    }
                    if check_file_signature(&path, "KiCad") && has_no_exclusion(&path, &["EasyEDA", "OpenJLC"]) {
                        return Ok((path, EDATool::KiCad));
                    }
                    if check_file_signature(&path, "EasyEDA") && has_no_exclusion(&path, &["KiCad", "Altium", "OpenJLC"]) {
                        return Ok((path, EDATool::LCEDA));
                    }
                    if keywords.iter().any(|&kw| check_file_signature(&path, kw)) {
                        log::log("! File appears to have been injected");
                        return Err(io::Error::new(io::ErrorKind::InvalidData, ""));
                    } else {
                        log::log("- No valid EDA signature found");
                        report_error();
                        return Err(io::Error::new(io::ErrorKind::NotFound, ""));
                    }
                }
            }
        }
    }
    log::log("- No valid EDA signature found");
    report_error();
    Err(io::Error::new(io::ErrorKind::NotFound, ""))
}