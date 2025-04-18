use std::collections::HashMap;
use std::error::Error;
use std::fs;
use fancy_regex::RegexBuilder;
use serde_yaml;
use crate::log;
use crate::error::report_error;
use crate::config::{get_rule_dir, get_temp_dir, get_target_dir};

pub fn process_files_with_rule(yaml_name: &str) -> Result<(), Box<dyn Error>> {
    let rule_path = get_rule_dir().join(yaml_name);
    let rule_content = fs::read_to_string(&rule_path).map_err(|e| format!("! Failed to read rule file '{}': {}", rule_path.display(), e))?;
    
    let rules: HashMap<String, String> = serde_yaml::from_str(&rule_content)
        .map_err(|e| format!("! Failed to parse YAML: {}", e))?;

    let temp_dir = get_temp_dir();
    let target_dir = get_target_dir();

    for (name, pattern) in rules {
        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()
            .map_err(|e| format!("! Invalid regex pattern '{}': {}", pattern, e))?;

        let mut found_paths = Vec::new();
        for entry in fs::read_dir(&temp_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if regex.is_match(file_name)? {
                        found_paths.push(path);
                    }
                }
            }
        }

        if found_paths.len() > 1 {
            log::log(&format!("! Regex pattern '{}' matched multiple files: {:?}", pattern, found_paths));
            report_error();
        }

        if !found_paths.is_empty() {
            let name_clone = name.clone();
            for src_path in found_paths {
                let ext = src_path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_uppercase();
                let dest_name = if ext.is_empty() {
                    name_clone.clone()
                } else {
                    format!("{}.{}", name_clone, ext)
                };
                
                let dest_path = target_dir.join(&dest_name);
                fs::copy(&src_path, &dest_path)?;
                log::log(&format!("+ Linked '{}' -> '{}'", 
                    src_path.file_name().unwrap().to_str().unwrap(),
                    dest_path.display()));
                    
                let new_name = match name_clone.as_str() {
                    "Gerber_TopSolderMaskLayer" => "Gerber_TopSolderMaskLayer.GTS",
                    "Gerber_TopSilkscreenLayer" => "Gerber_TopSilkscreenLayer.GTO",
                    "Gerber_TopPasteMaskLayer" => "Gerber_TopPasteMaskLayer.GTP",
                    "Gerber_TopLayer" => "Gerber_TopLayer.GTL",
                    "Gerber_InnerLayer2" => "Gerber_InnerLayer2.G2",
                    "Gerber_InnerLayer1" => "Gerber_InnerLayer1.G1",
                    "Gerber_InnerLayer3" => "Gerber_InnerLayer3.G3",
                    "Gerber_InnerLayer4" => "Gerber_InnerLayer4.G4",
                    "Gerber_InnerLayer5" => "Gerber_InnerLayer5.G5",
                    "Gerber_InnerLayer6" => "Gerber_InnerLayer6.G6",
                    "Gerber_BottomSolderMaskLayer" => "Gerber_BottomSolderMaskLayer.GBS",
                    "Gerber_BottomSilkscreenLayer" => "Gerber_BottomSilkscreenLayer.GBP",
                    "Gerber_BottomPasteMaskLayer" => "Gerber_BottomPasteMaskLayer.GPB",
                    "Gerber_BottomLayer" => "Gerber_BottomLayer.GBL",
                    "Gerber_BoardOutlineLayer" => "Gerber_BoardOutlineLayer.GM13",
                    "Drill_PTH_Through" => "Drill_PTH_Through.DRL",
                    "Drill_PTH_Through_Via" => "Drill_PTH_Through_Via.DRL",
                    "Drill_NPTH_Through" => "Drill_NPTH_Through.DRL",
                    _ => dest_name.as_str(),
                };
                
                if new_name != dest_name.as_str() {
                    let new_dest_path = target_dir.join(new_name);
                    fs::rename(&dest_path, &new_dest_path)?;
                    log::log(&format!("+ Match '{}' -> '{}'", 
                        dest_path.display(),
                        new_dest_path.display()));
                }
            }
        }
    }
    
    Ok(())
}