use crate::log;
use regex::Regex;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn load_yaml_rules(rule_path: &Path) -> HashMap<String, Regex> {
    let content = fs::read_to_string(rule_path).unwrap();
    let yaml: HashMap<String, Value> = serde_yaml::from_str(&content).unwrap();
    yaml.into_iter()
        .filter_map(|(k, v)| v.as_str().map(|s| (k, Regex::new(s).unwrap())))
        .collect()
}

fn capitalize_extension(filename: &str) -> String {
    let mut parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        parts[0] = &parts[0].to_uppercase();
        format!("{}.{}", parts[1], parts[0])
    } else {
        filename.to_string()
    }
}

fn process_files(temp_dir: &Path, target_dir: &Path, rules: &HashMap<String, Regex>) {
    let mut matched_files: HashMap<String, PathBuf> = HashMap::new();
    let mut missing_layers = vec![];

    for entry in fs::read_dir(temp_dir).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy();

        for (layer, regex) in rules {
            if regex.is_match(&file_name) {
                if matched_files.contains_key(layer) {
                    log::log(&format!("! Multiple matches for layer {}", layer));
                    return;
                }
                matched_files.insert(layer.clone(), file_path.clone());
            }
        }
    }

    let required_layers = ["TopLayer", "TopSolderMaskLayer", "BoardOutlineLayer"];
    let conditional_layers = [
        ("BottomLayer", "BottomSolderMaskLayer"),
        ("InnerLayer1", "InnerLayer2"),
    ];

    for layer in required_layers {
        if !matched_files.contains_key(layer) {
            log::log(&format!("! Missing required layer {}", layer));
            return;
        }
    }

    for (main, dependent) in conditional_layers {
        if matched_files.contains_key(main) && !matched_files.contains_key(dependent) {
            log::log(&format!("! {} found but {} is missing", main, dependent));
        }
    }

    for (layer, file_path) in matched_files {
        let prefix = if layer.contains("PTH") || layer.contains("NPTH") {
            "Drill_"
        } else {
            "Gerber_"
        };
        let new_name = format!("{}{}", prefix, layer);
        let new_path = target_dir.join(capitalize_extension(&new_name));
        fs::copy(file_path, new_path).unwrap();
    }

    log::log("+ File processing completed");
}

pub fn process_eda_files(temp_dir: &Path, target_dir: &Path, eda_tool: &str) {
    let rule_path = match eda_tool {
        "Altium" => target_dir.join("rule/altium_designer.yaml"),
        "KiCad" => target_dir.join("rule/kicad.yaml"),
        _ => return,
    };

    if !rule_path.exists() {
        log::log("! Rule file not found");
        return;
    }

    let rules = load_yaml_rules(&rule_path);
    process_files(temp_dir, target_dir, &rules);
}
