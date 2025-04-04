use std::env;
use sys_info;
use openjlc::log;
use std::sync::Mutex;
use std::time::Instant;
use lazy_static::lazy_static;
use openjlc::cli::get_input_file_path;
use openjlc::config::{get_temp_dir, get_target_dir, get_report_dir, check_and_download_rule_files};
use openjlc::extractor::extract_zip_to_temp;
use openjlc::identifier::{identify_eda_files, EDATool};
use openjlc::utils::{create_pcb_order_file, create_header_yaml};
use openjlc::processor::process_files_with_rule;
use openjlc::validator::validate_target_directory;
use openjlc::injector::inject_headers;
use openjlc::packager::package_target_dir;
use openjlc::cleaner::clear_directories;
use openjlc::error::report_error;
use rfd::MessageDialog;
use webbrowser;

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

fn get_os_and_arch() -> (String, String) {
    let os = sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string());
    let arch = env::consts::ARCH.to_string();
    (os, arch)
}

#[tokio::main]
async fn main() {
    clear_directories();
    let version = env!("CARGO_PKG_VERSION");
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let (os, arch) = get_os_and_arch();

    log::log(&format!("+ OpenJLC v{} {}", version, timestamp));
    log::log(&format!("- OS: {} & Arch: {}", os, arch));

    let temp_dir = get_temp_dir();
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).unwrap();
        log::log(&format!("+ Created temp at '{}'", temp_dir.display()));
    } else {
        log::log(&format!("- Temp directory already exists at '{}'", temp_dir.display()));
    }

    let target_dir = get_target_dir();
    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir).unwrap();
        log::log(&format!("+ Created target at '{}'", target_dir.display()));
    } else {
        log::log(&format!("- Target directory already exists at '{}'", target_dir.display()));
    }

    let report_dir = get_report_dir();
    if !report_dir.exists() {
        log::log("! Report directory not found");
        std::fs::create_dir_all(&report_dir).unwrap();
        log::log(&format!("+ Created report at '{}'", report_dir.display()));
    } else {
        log::log(&format!("- Report directory already exists at '{}'", report_dir.display()));
    }

    if let Err(e) = check_and_download_rule_files().await {
        log::log(&format!("! Failed to download rule files: {}", e));
        report_error();
    }

    let processing_start_time = Instant::now();

    if let Some(file_path) = get_input_file_path() {
        log::log(&format!("> Processing file: '{}'", file_path.display()));

        if let Err(e) = extract_zip_to_temp(&temp_dir, &file_path) {
            log::log(&format!("! Failed to extract zip file: {}", e));
            report_error();
        }

        log::log(&format!("+ Successfully extracted zip file to '{}'", temp_dir.display()));

        match identify_eda_files(&temp_dir) {
            Ok((gerber_file, tool)) => {
                log::log(&format!("+ Identified {} Gerber file: '{}'", eda_tool_to_str(&tool), gerber_file.display()));
                *EDA_TOOL.lock().unwrap() = tool.clone();

                if let Err(e) = create_pcb_order_file(&target_dir) {
                    log::log(&format!("! Failed to create PCB order file: {}", e));
                    report_error();
                }

                if let Err(e) = create_header_yaml(&target_dir) {
                    log::log(&format!("! Failed to create header.yaml: {}", e));
                    report_error();
                }

                let eda_type = match tool {
                    EDATool::Altium => "AD",
                    EDATool::KiCad => "Ki",
                    _ => {
                        log::log("! Unsupported EDA tool, exiting");
                        report_error();
                        "Unknown"
                    }
                };

                match tool {
                    EDATool::Altium => {
                        if let Err(e) = process_files_with_rule("altium_designer.yaml") {
                            log::log(&format!("! Altium processing failed: {}", e));
                            report_error();
                        }
                    }
                    EDATool::KiCad => {
                        if let Err(e) = process_files_with_rule("kicad.yaml") {
                            log::log(&format!("! KiCad processing failed: {}", e));
                            report_error();
                        }
                    }
                    _ => {}
                }

                validate_target_directory();
                inject_headers();
                let output_path = package_target_dir(eda_type);
                clear_directories();

                let processing_time = processing_start_time.elapsed().as_millis();
                let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();
                log::log(&format!("> Finished patch {}, took {}ms, dump at '{}'", file_name, processing_time, output_path.display()));
                
                let log_path = log::get_log_file_path();
                if let Ok(entries) = std::fs::read_dir(log_path.parent().unwrap()) {
                    let log_count = entries.filter(|e| e.is_ok()).count();
                    if log_count > 0 && log_count % 30 == 0 {
                        let result = MessageDialog::new()
                            .set_title("Support OpenJLC")
                            .set_description(&format!("You've used OpenJLC {} times. If you find it helpful, would you like to give the project a star on GitHub?", log_count))
                            .set_buttons(rfd::MessageButtons::YesNo)
                            .show();
                        if result == rfd::MessageDialogResult::Yes {
                            webbrowser::open("https://github.com/canmi21/openjlc").unwrap_or_default();
                        }
                    }
                }
            }
            Err(e) => {
                if !e.to_string().is_empty() {
                    log::log(&format!("! Failed to identify EDA file: {}", e));
                    report_error();
                }
            },
        }
    } else {
        log::log("! No valid file path provided");
        report_error();
    }
}