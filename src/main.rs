use openjlc::config::get_temp_dir;
use openjlc::cli::get_input_file_path;
use openjlc::log;
use openjlc::extractor::extract_zip_to_temp;
use openjlc::identifier::{identify_eda_files, EDATool};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fmt;

lazy_static! {
    static ref EDA_TOOL: Mutex<EDATool> = Mutex::new(EDATool::Unknown);
}

impl fmt::Debug for EDATool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            EDATool::Altium => "Altium",
            EDATool::KiCad => "KiCad",
            EDATool::Unknown => "Unknown",
        })
    }
}

fn main() {
    let temp_dir = get_temp_dir();
    if !temp_dir.exists() {
        log::log("! Temp directory not found");
        std::fs::create_dir_all(&temp_dir).unwrap();
        log::log(&format!("+ Created temp at {:?}", temp_dir));
    } else {
        log::log(&format!("+ Temp directory already exists at {:?}", temp_dir));
    }

    if let Some(file_path) = get_input_file_path() {
        log::log(&format!("> Processing file: {:?}", file_path));

        if let Err(e) = extract_zip_to_temp(&temp_dir, &file_path) {
            log::log(&format!("! Failed to extract zip file: {}", e));
            return;
        }

        log::log(&format!("+ Successfully extracted zip file to {:?}", temp_dir));

        match identify_eda_files(&temp_dir) {
            Ok((gerber_file, tool)) => {
                log::log(&format!("+ Identified {} Gerber file: {:?}", tool, gerber_file));
                *EDA_TOOL.lock().unwrap() = tool;
            }
            Err(e) => log::log(&format!("! Failed to identify EDA file: {}", e)),
        }
    } else {
        log::log("! No valid file path provided");
    }
}
