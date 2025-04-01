use openjlc::config::get_temp_dir;

fn main() {
    let temp_dir = get_temp_dir();
    println!("temp: {:?}", temp_dir);
}
