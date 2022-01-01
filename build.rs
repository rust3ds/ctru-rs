use std::env;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();
    
    if let Ok(_) = env::var("DEBUG") {
        println!("cargo:rustc-link-lib=static=ctrud");
    } else {
        println!("cargo:rustc-link-lib=static=ctru");
    }
    println!("cargo:rustc-link-search=native={}/libctru/lib", dkp_path);
    
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=pthread_3ds");
}
