use chrono::Local;
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn get_log_file_path() -> PathBuf {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    let log_dir = PathBuf::from(home_dir).join(".canmi/openjlc/log");

    if let Err(e) = create_dir_all(&log_dir) {
        eprintln!("Permission Denied: {}", e);
    }

    let date = Local::now().format("%Y-%m-%d").to_string();
    log_dir.join(format!("{}.log", date))
}

fn log_message(level: &str, message: &str) {
    let timestamp = Local::now().format("[%H:%M:%S]").to_string();
    let formatted_message = format!("{} [{}] {}", timestamp, level, message);

    println!("{}", formatted_message);

    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(get_log_file_path()) {
        writeln!(file, "{}", formatted_message).ok();
    }
}

pub fn info(message: &str) {
    log_message("INFO", message);
}

pub fn warn(message: &str) {
    log_message("WARN", message);
}

pub fn error(message: &str) {
    log_message("ERROR", message);
}
