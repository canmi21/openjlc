use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug)]
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

pub fn identify_eda_files(temp_dir: &Path) -> io::Result<(PathBuf, EDATool)> {
    let candidates = ["gto", "gtl", "gbl"];

    for entry in temp_dir.read_dir()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if candidates.contains(&ext_lower.as_str()) {
                    if check_file_signature(&path, "Altium") {
                        return Ok((path, EDATool::Altium));
                    }
                    if check_file_signature(&path, "KiCad") {
                        return Ok((path, EDATool::KiCad));
                    }
                    if check_file_signature(&path, "EasyEDA") {
                        return Ok((path, EDATool::LCEDA));
                    }
                }
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "- No valid EDA signature found"))
}
