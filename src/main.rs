use openjlc::config::get_temp_dir;
use openjlc::log;

fn main() {
    let temp_dir = get_temp_dir();
    if !temp_dir.exists() {
        log::log(&format!("! Temp directory not found"));
        std::fs::create_dir_all(&temp_dir).unwrap();
        log::log(&format!("+ Created temp at {:?}", temp_dir));
    }
}
