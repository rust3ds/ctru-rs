use std::env;
use std::path::PathBuf;
use std::fs;

use std::option::Option::{self, Some, None};

const ENV_DKP: &'static str = "DEVKITPRO";

fn find_libctru() -> Option<PathBuf> {
    if let Ok(value) = env::var(ENV_DKP) {
        let mut path = PathBuf::from(value);
        path.push("libctru/lib");
        // metadata returns Err if the dir does not exist
        if let Ok(metadata) = fs::metadata(path.as_path()) {
            if metadata.is_dir() {
                return Some(path);
            }
        }
    }
    return None;
}

fn main() {
    if let Some(path) = find_libctru() {
        if let Some(s) = path.to_str() {
            println!("cargo:rustc-link-lib=static=ctru");
            println!("cargo:rustc-link-search=native={}", s);
        } else {
            println!("path is not a valid string");
            std::process::exit(1);
        }
    } else {
        println!("could not find libctru");
        std::process::exit(1);
    }
}
