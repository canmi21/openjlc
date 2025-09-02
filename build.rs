/* build.rs */

use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows"
        && env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64"
    {
        let out = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        println!(
            "cargo:rustc-link-arg-bin=openjlc={}",
            out.join("icon.res").display()
        );
    }
}
