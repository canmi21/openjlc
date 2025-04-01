use std::env;
use std::path::{Path, PathBuf};

pub fn get_input_file_path() -> Option<PathBuf> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input_path = &args[1];
        let path = Path::new(input_path);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }
    None
}
