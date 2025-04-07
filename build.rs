use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let out = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        println!(
            "cargo:rustc-link-arg-bin=openjlc={}",
            out.join("icon.res").display()
        );
    }
}