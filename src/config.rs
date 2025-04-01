use std::env;
use std::path::PathBuf;
use reqwest;

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

async fn download_rule_file(url: &str, dest_path: &std::path::Path) -> reqwest::Result<()> {
    let content = reqwest::get(url).await?.text().await?;
    std::fs::write(dest_path, content)?;
    Ok(())
}

pub async fn check_and_download_rule_files() -> Result<(), reqwest::Error> {
    let rule_dir = get_rule_dir();
    if !rule_dir.exists() {
        std::fs::create_dir_all(&rule_dir)?;
    }

    let altium_path = rule_dir.join("altium_designer.yaml");
    if !altium_path.exists() {
        let url = "https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/rule/altium_designer.yaml";
        download_rule_file(url, &altium_path).await?;
    }

    let kicad_path = rule_dir.join("kicad.yaml");
    if !kicad_path.exists() {
        let url = "https://raw.githubusercontent.com/canmi21/openjlc/refs/heads/main/rule/kicad.yaml";
        download_rule_file(url, &kicad_path).await?;
    }

    Ok(())
}
