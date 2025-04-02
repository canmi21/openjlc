use std::collections::HashMap;
use std::error::Error;
use std::fs;
use regex::RegexBuilder;
use serde_yaml;
use crate::log;
use crate::config::{get_rule_dir, get_temp_dir, get_target_dir};

pub fn process_files_with_rule(yaml_name: &str) -> Result<(), Box<dyn Error>> {
    let rule_path = get_rule_dir().join(yaml_name);
    let rule_content = fs::read_to_string(&rule_path).map_err(|e| format!("Failed to read rule file {}: {}", rule_path.display(), e))?;
    
    let rules: HashMap<String, String> = serde_yaml::from_str(&rule_content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;

    let temp_dir = get_temp_dir();
    let target_dir = get_target_dir();

    for (name, pattern) in rules {
        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()
            .map_err(|e| format!("Invalid regex pattern '{}': {}", pattern, e))?;

        let mut found = None;
        for entry in fs::read_dir(&temp_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if regex.is_match(file_name) {
                        found = Some(path);
                        break;
                    }
                }
            }
        }

        if let Some(src_path) = found {
            let ext = src_path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            let dest_name = if ext.is_empty() {
                name
            } else {
                format!("{}.{}", name, ext)
            };
            
            let dest_path = target_dir.join(dest_name);
            fs::copy(&src_path, &dest_path)?;
            log::log(&format!("Copied '{}' to '{}'", 
                src_path.file_name().unwrap().to_str().unwrap(),
                dest_path.display()));
        }
    }
    
    Ok(())
}