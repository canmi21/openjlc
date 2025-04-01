use openjlc::config::get_temp_dir;
use openjlc::cli::get_input_file_path;
use openjlc::log;
use openjlc::extractor::extract_zip_to_temp;

fn main() {
    let temp_dir = get_temp_dir();
    if !temp_dir.exists() {
        log::log(&format!("! Temp directory not found"));
        std::fs::create_dir_all(&temp_dir).unwrap();
        log::log(&format!("+ Created temp at {:?}", temp_dir));
    } else {
        log::log(&format!("+ Temp directory already exists at {:?}", temp_dir));
    }

    if let Some(file_path) = get_input_file_path() {
        log::log(&format!("> Processing file: {:?}", file_path));

        if let Err(e) = extract_zip_to_temp(&temp_dir, &file_path) {
            log::log(&format!("! Failed to extract zip file: {}", e));
        } else {
            log::log(&format!("+ Successfully extracted zip file to {:?}", temp_dir));
        }
    } else {
        log::log(&format!("! No valid file path provided"));
    }
}
