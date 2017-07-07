use std::env;
use std::path::PathBuf;

fn main() {
    let dkp_path = PathBuf::from(env::var("DEVKITPRO").unwrap());
    println!("cargo:rustc-link-search=native={}", dkp_path.join("libctru/lib").display());
}
