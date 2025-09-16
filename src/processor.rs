/* src/processor.rs */

use crate::config::{get_rule_dir, get_target_dir, get_temp_dir};
use crate::gerber_modifier; // Import our new module
use crate::identifier::EDATool; // We need to know which EDA we're processing
use crate::log;
use fancy_regex::RegexBuilder;
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// The function signature now accepts the identified EDATool
pub fn process_files_with_rule(yaml_name: &str, eda_tool: &EDATool) -> Result<(), Box<dyn Error>> {
    let rule_path = get_rule_dir().join(yaml_name);
    let rule_content = fs::read_to_string(&rule_path)?;
    let rules: HashMap<String, String> = serde_yaml::from_str(&rule_content)?;

    let temp_dir = get_temp_dir();
    let target_dir = get_target_dir();

    for (logical_name, pattern) in rules {
        let regex = RegexBuilder::new(&pattern).case_insensitive(true).build()?;

        for entry in fs::read_dir(&temp_dir)? {
            let path = entry?.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if regex.is_match(file_name)? {
                        process_single_file(&path, &target_dir, &logical_name, eda_tool)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_single_file(
    src_path: &Path,
    target_dir: &Path,
    logical_name: &str,
    eda_tool: &EDATool,
) -> Result<(), Box<dyn Error>> {
    let dest_name = get_final_filename(logical_name);
    let dest_path = target_dir.join(dest_name);

    // Drill files are copied directly without modification
    if logical_name.contains("Drill") {
        fs::copy(src_path, &dest_path)?;
        log::log(&format!(
            "+ Copied '{}' -> '{}'",
            src_path.file_name().unwrap().to_str().unwrap(),
            dest_path.display()
        ));
        return Ok(());
    }

    // --- CORE LOGIC ---

    // 1. Read the original file content
    let mut content = fs::read_to_string(src_path)?;

    // 2. Inject the dynamic "EasyEDA" header
    let now = chrono::Local::now();
    content = format!(
        "G04 EasyEDA Pro v2.2.42.2, {}*\nG04 Gerber Generator version 0.3*\n{}",
        now.format("%Y-%m-%d %H:%M:%S"),
        content.replace("\r\n", "\n")
    );

    // 3. If it's from KiCad, apply the specific syntax fix
    if matches!(eda_tool, EDATool::KiCad) {
        content = gerber_modifier::convert_kicad_aperture_format(content);
    }

    // 4. Inject the unique MD5 hash aperture
    content = gerber_modifier::add_hash_aperture_to_gerber(content, false)?;

    // 5. Write the fully modified content to the destination file ONCE
    fs::write(&dest_path, content)?;

    log::log(&format!(
        "+ Processed '{}' -> '{}'",
        src_path.file_name().unwrap().to_str().unwrap(),
        dest_path.display()
    ));

    Ok(())
}

// Helper function with filenames corrected to EXACTLY match TransJLC's JLC_STYLE
fn get_final_filename(logical_name: &str) -> String {
    match logical_name {
        "Gerber_TopSolderMaskLayer" => "Gerber_TopSolderMaskLayer.GTS".to_string(),
        "Gerber_TopSilkscreenLayer" => "Gerber_TopSilkscreenLayer.GTO".to_string(),
        "Gerber_TopPasteMaskLayer" => "Gerber_TopPasteMaskLayer.GTP".to_string(),
        "Gerber_TopLayer" => "Gerber_TopLayer.GTL".to_string(),
        "Gerber_InnerLayer1" => "Gerber_InnerLayer1.G1".to_string(),
        "Gerber_InnerLayer2" => "Gerber_InnerLayer2.G2".to_string(),
        "Gerber_InnerLayer3" => "Gerber_InnerLayer3.G3".to_string(),
        "Gerber_InnerLayer4" => "Gerber_InnerLayer4.G4".to_string(),
        "Gerber_InnerLayer5" => "Gerber_InnerLayer5.G5".to_string(),
        "Gerber_InnerLayer6" => "Gerber_InnerLayer6.G6".to_string(),
        "Gerber_BottomSolderMaskLayer" => "Gerber_BottomSolderMaskLayer.GBS".to_string(),
        "Gerber_BottomSilkscreenLayer" => "Gerber_BottomSilkscreenLayer.GBO".to_string(),
        "Gerber_BottomPasteMaskLayer" => "Gerber_BottomPasteMaskLayer.GBP".to_string(),
        "Gerber_BottomLayer" => "Gerber_BottomLayer.GBL".to_string(),
        "Gerber_BoardOutlineLayer" => "Gerber_BoardOutlineLayer.GKO".to_string(),
        "Drill_PTH_Through" => "Drill_PTH_Through.DRL".to_string(),
        "Drill_PTH_Through_Via" => "Drill_PTH_Through_Via.DRL".to_string(),
        "Drill_NPTH_Through" => "Drill_NPTH_Through.DRL".to_string(),
        // Fallback for any other layer types, though unlikely to be hit with strict rules
        _ => format!("{}.{}", logical_name, "gbr"),
    }
}
