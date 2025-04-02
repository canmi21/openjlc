use std::fs;
use crate::config::{get_rule_dir, get_temp_dir, get_target_dir};
use crate::log;
use regex::Regex;
use std::io::{self, BufRead};

pub fn process_rule_file(rule_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rule_file_path = get_rule_dir().join(rule_name);
    if !rule_file_path.exists() {
        log::log(&format!("! Rule file not found: {}", rule_name));
        return Ok(());
    }

    let file = fs::File::open(rule_file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = Regex::new(r#"^(?P<layer>.+):\s"(?P<regex>.+)"$"#)?.captures(&line) {
            let layer = captures.name("layer").unwrap().as_str();
            let pattern = captures.name("regex").unwrap().as_str();
            let re = Regex::new(pattern)?;

            let temp_dir = get_temp_dir();
            for entry in fs::read_dir(temp_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && re.is_match(path.to_str().unwrap_or_default()) {
                    let target_dir = get_target_dir();
                    let target_path = target_dir.join(format!("{}.{}", layer, path.extension().unwrap_or_default().to_str().unwrap_or_default()));

                    fs::copy(&path, &target_path)?;
                    log::log(&format!("+ Copied {} to {}", path.display(), target_path.display()));
                    break;
                }
            }
        }
    }

    Ok(())
}