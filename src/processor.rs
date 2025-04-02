use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::log;
use crate::config::{get_temp_dir, get_target_dir, get_rule_dir};

pub fn process_rule_yaml(rule_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rule_file_path = get_rule_dir().join(rule_name);
    if !rule_file_path.exists() {
        log::log(&format!("! Rule file {:?} not found", rule_file_path));
        return Ok(());
    }

    let rule_content = fs::read_to_string(rule_file_path)?;
    let temp_dir = get_temp_dir();
    let target_dir = get_target_dir();

    for line in rule_content.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let layer_name = parts[0];
        let regex_str = parts[1].trim_matches('"');
        let regex = Regex::new(regex_str)?;

        log::log(&format!("> Using regex for layer '{}': {}", layer_name, regex_str));

        let mut matched_file: Option<PathBuf> = None;
        for entry in fs::read_dir(&temp_dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.display().to_string();

            log::log(&format!("> Checking file: {}", path_str));

            if path.is_file() && regex.is_match(&path_str) {
                matched_file = Some(path);
                break;
            }
        }

        if let Some(matched_path) = matched_file {
            let target_path = target_dir.join(format!("{}.{}", layer_name, matched_path.extension().unwrap_or_default().to_str().unwrap_or("")));
            fs::copy(&matched_path, &target_path)?;
            log::log(&format!("+ Copied and renamed {:?} to {:?}", matched_path, target_path));
        } else {
            log::log(&format!("! No matching file found for rule: {}", layer_name));
        }
    }

    Ok(())
}