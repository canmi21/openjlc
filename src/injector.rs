use std::fs;
use fancy_regex::Regex;
use crate::config::get_target_dir;
use crate::error::report_error;
use crate::log;

pub fn inject_headers() {
    let target_dir = get_target_dir();
    let header_path = target_dir.join("header.yaml");
    if !header_path.exists() {
        log::log("! header.yaml not found");
        report_error();
    }

    let header_content = match fs::read_to_string(&header_path) {
        Ok(content) => content.lines().skip(1).map(|line| line.trim_start()).collect::<Vec<_>>().join("\n") + "\n",
        Err(e) => {
            log::log(&format!("! Failed to read header.yaml: {}", e));
            report_error();
        }
    };

    let re = Regex::new(r"(?i)(KiCad|Altium Designer|Altium)").unwrap();
    
    for entry in fs::read_dir(&target_dir).unwrap() {
        let path = entry.unwrap().path();

        if !path.is_file() {
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();
        if filename == "PCB下单必读.txt" || filename == "header.yaml" {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let modified_content = header_content.clone() + &content;
        let modified_content = re
            .replace_all(&modified_content, |caps: &fancy_regex::Captures| {
                match caps.get(0).unwrap().as_str().to_lowercase().as_str() {
                    "kicad" | "altium" => "OpenJLC".to_string(),
                    "altium designer" => "Ki Designer".to_string(),
                    s => s.to_string(),
                }
            })
            .to_string();

        if let Err(e) = fs::write(&path, modified_content) {
            log::log(&format!("! Failed to inject header into {:?}: {}", path, e));
            report_error();
        } else {
            log::log(&format!("> Inject '{:?}'", path));
        }
    }
}