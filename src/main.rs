use openjlc::log;
use std::sync::Mutex;
use lazy_static::lazy_static;
use openjlc::cli::get_input_file_path;
use openjlc::config::{get_temp_dir, get_target_dir, check_and_download_rule_files};
use openjlc::extractor::extract_zip_to_temp;
use openjlc::identifier::{identify_eda_files, EDATool};
use openjlc::utils::{create_pcb_order_file, create_header_yaml};
use openjlc::processor::process_files_with_rule;
use openjlc::validator::validate_target_directory;
use openjlc::injector::inject_headers;
use openjlc::packager::package_target_dir;
use openjlc::cleaner::clear_directories;

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
                *EDA_TOOL.lock().unwrap() = tool.clone();

                if let Err(e) = create_pcb_order_file(&target_dir) {
                    log::log(&format!("! Failed to create PCB order file: {}", e));
                }

                if let Err(e) = create_header_yaml(&target_dir) {
                    log::log(&format!("! Failed to create header.yaml: {}", e));
                }

                let eda_type = match tool {
                    EDATool::Altium => "AD",
                    EDATool::KiCad => "Ki",
                    _ => {
                        log::log("! Unsupported EDA tool, exiting");
                        std::process::exit(0);
                    }
                };

                match tool {
                    EDATool::Altium => {
                        if let Err(e) = process_files_with_rule("altium_designer.yaml") {
                            log::log(&format!("! Altium processing failed: {}", e));
                        }
                    }
                    EDATool::KiCad => {
                        if let Err(e) = process_files_with_rule("kicad.yaml") {
                            log::log(&format!("! KiCad processing failed: {}", e));
                        }
                    }
                    _ => {}
                }

                validate_target_directory();
                inject_headers();
                package_target_dir(eda_type);
                clear_directories();
            }
            Err(e) => {
                if !e.to_string().is_empty() {
                    log::log(&format!("! Failed to identify EDA file: {}", e));
                }
            },
        }
    } else {
        log::log("! No valid file path provided");
    }
}