use std::env;
use std::path::{Path, PathBuf};

pub fn get_input_file_path() -> Option<PathBuf> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input_path = &args[1];
        let path = Path::new(input_path);
        if path.exists() {
            let absolute_path = if path.is_relative() {
                let current_dir = env::current_dir().unwrap();
                current_dir.join(path)
            } else {
                path.to_path_buf()
            };

            if let Some(extension) = absolute_path.extension() {
                if extension == "zip" {
                    return Some(absolute_path);
                } else {
                    return None;
                }
            }
        }
    }
    None
}
