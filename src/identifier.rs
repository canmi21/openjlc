use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn identify_altium_files(temp_dir: &Path) -> io::Result<PathBuf> {
    let candidates = ["gto", "gtl", "gbl"];
    let mut detected_file = None;

    for entry in temp_dir.read_dir()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if candidates.contains(&ext.to_lowercase().as_str()) {
                    if let Ok(file) = File::open(&path) {
                        let reader = io::BufReader::new(file);
                        if reader
                            .lines()
                            .take(100)
                            .filter_map(|line| line.ok())
                            .any(|line| line.contains("Altium"))
                        {
                            detected_file = Some(path);
                            break;
                        }
                    }
                }
            }
        }
    }

    detected_file.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "- No Altium signature found"))
}
