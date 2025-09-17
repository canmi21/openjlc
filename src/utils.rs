/* src/utils.rs */

use chrono::Local;
use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn create_pcb_order_file(target_dir: &Path) -> io::Result<()> {
    let file_path = target_dir.join("PCB下单必读.txt");
    let mut file = File::create(file_path)?;

    let content =
        "如何进行PCB下单\n\n请查看：\nhttps://prodocs.lceda.cn/cn/pcb/order-order-pcb/index.html";
    file.write_all(content.as_bytes())?;

    Ok(())
}

// move to process job
pub fn create_header_yaml(target_dir: &Path) -> io::Result<()> {
    let file_path = target_dir.join("header.yaml");
    let mut file = File::create(file_path)?;

    let mut rng = rand::rng();
    let version = format!(
        "v2.{}.{}.{}",
        rng.random_range(1..=2),
        rng.random_range(1..=37),
        rng.random_range(1..=3)
    );
    let name = if rng.random_bool(0.5) {
        "EasyEDA"
    } else {
        "EasyEDA Pro"
    };
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let content = format!(
        "header: |-\n  G04 {} {} {}*\n  G04 Gerber Generator version 0.3*",
        name, version, timestamp
    );
    file.write_all(content.as_bytes())?;

    Ok(())
}
