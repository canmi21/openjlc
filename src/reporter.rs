use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::time::Duration;
use chrono::Utc;
use zip::write::{FileOptions, ZipWriter};
use crate::config::{get_temp_dir, get_target_dir, get_rule_dir, get_report_dir};
use crate::cleaner::clear_directories;
use notify_rust::Notification;

pub fn generate_report(input_zip: PathBuf) {
    let report_dir = get_report_dir();
    if report_dir.exists() {
        fs::remove_dir_all(&report_dir).unwrap();
    }
    fs::create_dir_all(&report_dir).unwrap();

    fs::copy(&input_zip, report_dir.join(input_zip.file_name().unwrap())).unwrap();

    let temp_dir = get_temp_dir();
    let temp_target = report_dir.join("temp");
    fs::create_dir_all(&temp_target).unwrap();
    for entry in fs::read_dir(&temp_dir).unwrap() {
        let entry = entry.unwrap();
        let dest = temp_target.join(entry.file_name());
        if entry.path().is_dir() {
            fs::create_dir_all(&dest).unwrap();
            fs::copy(entry.path(), dest).unwrap();
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }

    let target_dir = get_target_dir();
    let target_target = report_dir.join("target");
    fs::create_dir_all(&target_target).unwrap();
    for entry in fs::read_dir(&target_dir).unwrap() {
        let entry = entry.unwrap();
        let dest = target_target.join(entry.file_name());
        if entry.path().is_dir() {
            fs::create_dir_all(&dest).unwrap();
            fs::copy(entry.path(), dest).unwrap();
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }

    let rule_dir = get_rule_dir();
    let rule_target = report_dir.join("rule");
    fs::create_dir_all(&rule_target).unwrap();
    for entry in fs::read_dir(&rule_dir).unwrap() {
        let entry = entry.unwrap();
        let dest = rule_target.join(entry.file_name());
        if entry.path().is_dir() {
            fs::create_dir_all(&dest).unwrap();
            fs::copy(entry.path(), dest).unwrap();
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }

    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    let log_dir = PathBuf::from(home_dir).join(".canmi/openjlc/logs");
    let log_target = report_dir.join("logs");
    fs::create_dir_all(&log_target).unwrap();
    for entry in fs::read_dir(&log_dir).unwrap() {
        let entry = entry.unwrap();
        let dest = log_target.join(entry.file_name());
        if entry.path().is_dir() {
            fs::create_dir_all(&dest).unwrap();
            fs::copy(entry.path(), dest).unwrap();
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }

    let timestamp = Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let zip_name = format!("report-{}.zip", timestamp);
    let zip_path = report_dir.join(&zip_name);
    let file = File::create(&zip_path).unwrap();
    let mut zip = ZipWriter::new(file);

    for entry in fs::read_dir(&report_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        if path.is_file() && name != zip_name {
            let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Deflated);
            zip.start_file(name.to_string(), options).unwrap();
            let mut f = File::open(path).unwrap();
            std::io::copy(&mut f, &mut zip).unwrap();
        } else if path.is_dir() {
            for sub_entry in fs::read_dir(&path).unwrap() {
                let sub_entry = sub_entry.unwrap();
                let sub_path = sub_entry.path();
                let sub_name = sub_path.file_name().unwrap().to_string_lossy();
                let relative_name = format!("{}/{}", name, sub_name);
                let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Deflated);
                zip.start_file(relative_name, options).unwrap();
                let mut f = File::open(sub_path).unwrap();
                std::io::copy(&mut f, &mut zip).unwrap();
            }
        }
    }

    zip.finish().unwrap();

    for entry in fs::read_dir(&report_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() != "zip" {
            fs::remove_file(path).unwrap();
        } else if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    clear_directories();

    Notification::new()
        .summary("Process Error")
        .body("Here encountered some issues")
        .timeout(Duration::from_millis(2100))
        .show()
        .unwrap();

    if cfg!(target_os = "windows") {
        std::process::Command::new("explorer").arg(report_dir).spawn().unwrap();
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(report_dir).spawn().unwrap();
    } else {
        std::process::Command::new("xdg-open").arg(report_dir).spawn().unwrap();
    }

    std::process::exit(0);
}