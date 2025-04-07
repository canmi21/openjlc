use std::fs::{create_dir_all, read_dir, remove_dir_all, rename, File};
use std::io::{self};
use std::path::{Path, PathBuf};
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

    let entries: Vec<PathBuf> = read_dir(temp_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    if entries.len() == 1 && entries[0].is_dir() {
        let inner_dir = &entries[0];
        for entry in read_dir(inner_dir)? {
            let entry = entry?;
            let from = entry.path();
            let file_name = entry.file_name();
            let to = temp_dir.join(file_name);
            if to.exists() {
                if to.is_dir() {
                    remove_dir_all(&to)?;
                } else {
                    std::fs::remove_file(&to)?;
                }
            }
            rename(from, to)?;
        }
        remove_dir_all(inner_dir)?;
    }

    Ok(())
}