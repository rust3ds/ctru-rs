extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let devkitpro_path = PathBuf::from(env::var("DEVKITPRO").unwrap());

    let bindings = bindgen::Builder::default()
        .use_core()
        .trust_clang_mangling(false)
        .generate_comments(false)
        .ctypes_prefix("libc")
        .header(devkitpro_path.join("libctru/include/3ds.h").to_str().unwrap())
        .hide_type("u8")
        .hide_type("u16")
        .hide_type("u32")
        .hide_type("u64")
        .clang_arg(format!("--sysroot={}/devkitARM/arm-none-eabi", devkitpro_path.display()))
        .clang_arg(format!("-I{}/libctru/include", devkitpro_path.display()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
