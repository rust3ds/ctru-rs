use bindgen::callbacks::ParseCallbacks;
use bindgen::{Builder, RustTarget};
use std::path::PathBuf;

#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }
}

fn main() {
    let devkitpro = std::env::var("DEVKITPRO").expect("DEVKITPRO not set in environment");
    let devkitarm = std::env::var("DEVKITARM").expect("DEVKITARM not set in environment");

    let include_path = PathBuf::from_iter([devkitpro.as_str(), "libctru", "include"]);
    let ctru_header = include_path.join("3ds.h");

    let sysroot = PathBuf::from(devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let errno_header = system_include.join("errno.h");

    let bindings = Builder::default()
        .header(ctru_header.to_str().unwrap())
        .header(errno_header.to_str().unwrap())
        .rust_target(RustTarget::Nightly)
        .use_core()
        .trust_clang_mangling(false)
        .must_use_type("Result")
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .blocklist_type("u(8|16|32|64)")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__va_list")
        .opaque_type("MiiData")
        .derive_default(true)
        .clang_args([
            "--target=arm-none-eabi",
            "--sysroot",
            sysroot.to_str().unwrap(),
            "-isystem",
            system_include.to_str().unwrap(),
            "-I",
            include_path.to_str().unwrap(),
            "-mfloat-abi=hard",
            "-march=armv6k",
            "-mtune=mpcore",
            "-mfpu=vfp",
            "-DARM11",
            "-D__3DS__",
        ])
        .parse_callbacks(Box::new(CustomCallbacks))
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write(Box::new(std::io::stdout()))
        .expect("failed to write bindings");
}
