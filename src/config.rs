use std::env;
use std::fs;
use std::path::PathBuf;
use reqwest;
use crate::log;

pub fn get_temp_dir() -> PathBuf {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".canmi/openjlc/temp")
}

pub fn get_target_dir() -> PathBuf {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".canmi/openjlc/target")
}

pub fn get_rule_dir() -> PathBuf {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".canmi/openjlc/rule")
}

async fn download_rule_file(url: &str, dest_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    log::log(&format!("> Downloading rule file from {}", url));
    let content = reqwest::get(url).await?.text().await?;
    fs::write(dest_path, content)?;
    log::log(&format!("+ Downloaded rule file to {:?}", dest_path));
    Ok(())
}

pub async fn check_and_download_rule_files() -> Result<(), Box<dyn std::error::Error>> {
    let rule_dir = get_rule_dir();
    if !rule_dir.exists() {
        log::log("! Rule directory not found");
        fs::create_dir_all(&rule_dir)?;
        log::log(&format!("+ Created rule directory at {:?}", rule_dir));
    } else {
        log::log(&format!("- Rule directory already exists at {:?}", rule_dir));
    }

    let altium_path = rule_dir.join("altium_designer.yaml");
    if !altium_path.exists() {
        log::log("! Missing rule file: altium_designer.yaml");
        let url = "https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/rule/altium_designer.yaml";
        download_rule_file(url, &altium_path).await?;
    } else {
        log::log(&format!("- Rule file already exists: {:?}", altium_path));
    }

    let kicad_path = rule_dir.join("kicad.yaml");
    if !kicad_path.exists() {
        log::log("! Missing rule file: kicad.yaml");
        let url = "https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/rule/kicad.yaml";
        download_rule_file(url, &kicad_path).await?;
    } else {
        log::log(&format!("- Rule file already exists: {:?}", kicad_path));
    }

    Ok(())
}
