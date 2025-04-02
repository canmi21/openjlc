use openjlc::config::{get_temp_dir, get_target_dir, check_and_download_rule_files};
use openjlc::cli::get_input_file_path;
use openjlc::log;
use openjlc::extractor::extract_zip_to_temp;
use openjlc::identifier::{identify_eda_files, EDATool};
use openjlc::processor::process_rule_yaml;
use lazy_static::lazy_static;
use std::sync::Mutex;
use openjlc::utils::{create_pcb_order_file, create_header_yaml};

lazy_static! {
    static ref EDA_TOOL: Mutex<EDATool> = Mutex::new(EDATool::Unknown);
}

fn eda_tool_to_str(tool: &EDATool) -> &'static str {
    match tool {
        EDATool::Altium => "Altium",
        EDATool::KiCad => "KiCad",
        EDATool::LCEDA => "LCEDA",
        EDATool::Unknown => "Unknown",
    }
}

#[tokio::main]
async fn main() {
    let temp_dir = get_temp_dir();
    if !temp_dir.exists() {
        log::log("! Temp directory not found");
        std::fs::create_dir_all(&temp_dir).unwrap();
        log::log(&format!("+ Created temp at {:?}", temp_dir));
    } else {
        log::log(&format!("- Temp directory already exists at {:?}", temp_dir));
    }

    let target_dir = get_target_dir();
    if !target_dir.exists() {
        log::log("! Target directory not found");
        std::fs::create_dir_all(&target_dir).unwrap();
        log::log(&format!("+ Created target at {:?}", target_dir));
    } else {
        log::log(&format!("- Target directory already exists at {:?}", target_dir));
    }

    if let Err(e) = check_and_download_rule_files().await {
        log::log(&format!("! Failed to download rule files: {}", e));
        return;
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
                log::log(&format!("+ Identified {} Gerber file: {:?}", eda_tool_to_str(&tool), gerber_file));
                *EDA_TOOL.lock().unwrap() = tool;

                match *EDA_TOOL.lock().unwrap() {
                    EDATool::Altium => process_rule_yaml("altium_designer.yaml").unwrap(),
                    EDATool::KiCad => process_rule_yaml("kicad.yaml").unwrap(),
                    EDATool::LCEDA => {
                        log::log("! LCEDA tool detected, stopping further processing.");
                        return;
                    }
                    _ => log::log("! Unknown EDA tool detected, skipping rule processing."),
                }

                if let Err(e) = create_pcb_order_file(&target_dir) {
                    log::log(&format!("! Failed to create PCB order file: {}", e));
                }

                if let Err(e) = create_header_yaml(&target_dir) {
                    log::log(&format!("! Failed to create header.yaml: {}", e));
                }
            }
            Err(e) => log::log(&format!("! Failed to identify EDA file: {}", e)),
        }
    } else {
        log::log("! No valid file path provided");
    }
}
