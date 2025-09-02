/* src/config.rs */

use crate::log;
use reqwest;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_temp_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(".canmi")
        .join("openjlc")
        .join("temp")
}

pub fn get_target_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(".canmi")
        .join("openjlc")
        .join("target")
}

pub fn get_rule_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(".canmi")
        .join("openjlc")
        .join("rule")
}

pub fn get_report_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    Path::new(&home_dir)
        .join(".canmi")
        .join("openjlc")
        .join("report")
}

async fn download_rule_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    log::log(&format!("> Downloading rule file from '{}'", url));
    let content = reqwest::get(url).await?.text().await?;
    fs::write(dest_path, content)?;
    log::log(&format!(
        "+ Downloaded rule file to '{}'",
        dest_path.display()
    ));
    Ok(())
}

pub async fn check_and_download_rule_files() -> Result<(), Box<dyn std::error::Error>> {
    let rule_dir = get_rule_dir();
    if !rule_dir.exists() {
        log::log("! Rule directory not found");
        fs::create_dir_all(&rule_dir)?;
        log::log(&format!(
            "+ Created rule directory at '{}'",
            rule_dir.display()
        ));
    } else {
        log::log(&format!(
            "- Rule directory already exists at '{}'",
            rule_dir.display()
        ));
    }

    let altium_path = rule_dir.join("altium_designer.yaml");
    if !altium_path.exists() {
        log::log("! Missing rule file: altium_designer.yaml");
        let url = "https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/rule/altium_designer.yaml";
        download_rule_file(url, &altium_path).await?;
    } else {
        log::log(&format!(
            "- Rule file already exists: '{}'",
            altium_path.display()
        ));
    }

    let kicad_path = rule_dir.join("kicad.yaml");
    if !kicad_path.exists() {
        log::log("! Missing rule file: kicad.yaml");
        let url =
            "https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/rule/kicad.yaml";
        download_rule_file(url, &kicad_path).await?;
    } else {
        log::log(&format!(
            "- Rule file already exists: '{}'",
            kicad_path.display()
        ));
    }

    let report_dir = get_report_dir();
    if !report_dir.exists() {
        log::log("! Report directory not found");
        fs::create_dir_all(&report_dir)?;
        log::log(&format!(
            "+ Created report directory at '{}'",
            report_dir.display()
        ));
    } else {
        log::log(&format!(
            "- Report directory already exists at '{}'",
            report_dir.display()
        ));
    }

    Ok(())
}
