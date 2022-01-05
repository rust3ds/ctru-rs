use std::env;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();

    if env::var("DEBUG").is_ok() {
        println!("cargo:rustc-link-lib=static=ctrud");
    } else {
        println!("cargo:rustc-link-lib=static=ctru");
    }
    println!("cargo:rustc-link-search=native={}/libctru/lib", dkp_path);
}
