use std::env;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();
    let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();

    println!("cargo:rustc-link-search=native={}/libctru/lib", dkp_path);
    println!("cargo:rustc-link-search=native={}", manifest_path);
    println!(
        "cargo:rustc-link-search=native={}/devkitARM/arm-none-eabi/lib/armv6k/fpu",
        dkp_path
    );
    println!(
        "cargo:rustc-link-search=native={}/devkitARM/lib/gcc/arm-none-eabi/11.1.0/armv6k/fpu",
        dkp_path
    );

    println!(
        "cargo:rustc-link-lib=static={}",
        match profile.as_str() {
            "debug" => "ctrud",
            _ => "ctru",
        }
    );
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=sysbase");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=pthread_3ds");
    println!("cargo:rustc-link-lib=static=linker_fix_3ds");
}
