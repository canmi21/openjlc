use std::fs;
use std::fs::File;
use zip::write::{FileOptions, ZipWriter};
use crate::config::get_target_dir;
use crate::cli::get_input_file_path;
use crate::log;

pub fn package_target_dir() {
    let target_dir = get_target_dir();
    let header_path = target_dir.join("header.yaml");
    if header_path.exists() {
        fs::remove_file(&header_path).unwrap();
    }

    let input_path = get_input_file_path().unwrap();
    let input_file_name = input_path.file_stem().unwrap().to_string_lossy();
    let output_zip_name = format!("{}_openjlc.zip", input_file_name);
    let output_path = input_path.parent().unwrap().join(output_zip_name);

    let file = File::create(&output_path).unwrap();
    let mut zip = ZipWriter::new(file);

    for entry in fs::read_dir(&target_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_file() {
            let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Deflated);
            zip.start_file(name.to_string(), options).unwrap();
            let mut f = File::open(path).unwrap();
            std::io::copy(&mut f, &mut zip).unwrap();
        }
    }

    zip.finish().unwrap();
    log::log(&format!("+ Packaged target directory into {:?}", output_path));
}