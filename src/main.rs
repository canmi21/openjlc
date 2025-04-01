use openjlc::config::get_temp_dir;
use openjlc::log;

fn main() {
    let temp_dir = get_temp_dir();
    log::log(&format!("Temp directory: {:?}", temp_dir));
}
