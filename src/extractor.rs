use std::fs::{create_dir_all, File};
use std::io::{self};
use std::path::Path;
use zip::read::ZipArchive;

pub fn extract_zip_to_temp(temp_dir: &Path, zip_file: &Path) -> io::Result<()> {
    let file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = temp_dir.join(file.name());

        if file.name().ends_with('/') {
            create_dir_all(&file_path)?;
        } else {
            if let Some(parent) = file_path.parent() {
                create_dir_all(parent)?;
            }

            let mut output_file = File::create(file_path)?;
            io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}