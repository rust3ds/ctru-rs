use std::env;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();

    println!("cargo:rustc-link-search=native={}/libctru/lib", dkp_path);
    println!("cargo:rustc-link-lib=static={}", match env::var("PROFILE").unwrap().as_str() {
        "debug" => "ctrud",
        _       => "ctru",
    });
}
