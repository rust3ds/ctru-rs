use bindgen::callbacks::ParseCallbacks;
use bindgen::{Builder, RustTarget};

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }
}

fn main() {
    let devkitpro = env::var("DEVKITPRO").unwrap();
    let devkitarm = env::var("DEVKITARM").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rustc-link-search=native={devkitpro}/libctru/lib");
    println!(
        "cargo:rustc-link-lib=static={}",
        match profile.as_str() {
            "debug" => "ctrud",
            _ => "ctru",
        }
    );

    match check_libctru_version() {
        Ok((maj, min, patch)) => {
            eprintln!("using libctru version {maj}.{min}.{patch}");

            // These are accessible by the crate during build with `env!()`.
            // We might consider exporting some public constants or something.
            println!("cargo:rustc-env=LIBCTRU_VERSION={maj}.{min}.{patch}");
            println!("cargo:rustc-env=LIBCTRU_MAJOR={maj}");
            println!("cargo:rustc-env=LIBCTRU_MINOR={min}");
            println!("cargo:rustc-env=LIBCTRU_PATCH={patch}");
        }
        Err(err) => println!("cargo:warning=failed to check libctru version: {err}"),
    }

    let gcc_version = get_gcc_version(PathBuf::from(&devkitarm).join("bin/arm-none-eabi-gcc"));

    let include_path = PathBuf::from_iter([devkitpro.as_str(), "libctru", "include"]);
    let ctru_header = include_path.join("3ds.h");

    let sysroot = Path::new(&devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let gcc_include = PathBuf::from(format!(
        "{devkitarm}/lib/gcc/arm-none-eabi/{gcc_version}/include"
    ));
    let errno_header = system_include.join("errno.h");

    // Build libctru bindings
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
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("libctru_statics_wrapper"))
        .clang_args([
            "--target=arm-none-eabi",
            "--sysroot",
            sysroot.to_str().unwrap(),
            "-isystem",
            system_include.to_str().unwrap(),
            "-isystem",
            gcc_include.to_str().unwrap(),
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
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile static inline fns wrapper
    let cc = Path::new(devkitarm.as_str()).join("bin/arm-none-eabi-gcc");
    let ar = Path::new(devkitarm.as_str()).join("bin/arm-none-eabi-ar");

    cc::Build::new()
        .compiler(cc)
        .archiver(ar)
        .include(&include_path)
        .file(out_dir.join("libctru_statics_wrapper.c"))
        .flag("-march=armv6k")
        .flag("-mtune=mpcore")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=vfp")
        .flag("-mtp=soft")
        .flag("-Wno-deprecated-declarations")
        .compile("ctru_statics_wrapper");
}

fn get_gcc_version(path_to_gcc: PathBuf) -> String {
    let Output { stdout, .. } = Command::new(path_to_gcc)
        .arg("--version")
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    let stdout_str = String::from_utf8_lossy(&stdout);

    stdout_str
        .split(|c: char| c.is_whitespace())
        .nth(4)
        .unwrap()
        .to_string()
}

fn parse_libctru_version(version: &str) -> Result<(String, String, String), &str> {
    let versions: Vec<_> = version
        .split(|c| c == '.' || c == '-')
        .map(String::from)
        .collect();

    match &versions[..] {
        [major, minor, patch, _build] => Ok((major.clone(), minor.clone(), patch.clone())),
        _ => Err("unexpected number of version segments"),
    }
}

fn check_libctru_version() -> Result<(String, String, String), Box<dyn Error>> {
    let pacman = which::which("dkp-pacman").or_else(|_| which::which("pacman"))?;

    let Output { stdout, .. } = Command::new(&pacman)
        .args(["--query", "libctru"])
        .stderr(Stdio::inherit())
        .output()?;

    let output_str = String::from_utf8_lossy(&stdout);

    let (_pkg, lib_version) = output_str
        .split_once(char::is_whitespace)
        .ok_or("unexpected pacman output format")?;

    let lib_version = lib_version.trim();

    let cargo_pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let (_, crate_built_version) = cargo_pkg_version
        .split_once('+')
        .expect("crate version should have '+' delimeter");

    if lib_version != crate_built_version {
        Err(format!(
            "libctru version is {lib_version} but this crate was built for {crate_built_version}"
        )
        .into());
    }

    let Output { stdout, .. } = Command::new(pacman)
        .args(["--query", "--list", "libctru"])
        .stderr(Stdio::inherit())
        .output()?;

    for line in String::from_utf8_lossy(&stdout).split('\n') {
        let Some((_pkg, file)) = line.split_once(char::is_whitespace) else {
            continue;
        };

        println!("cargo:rerun-if-changed={file}");
    }

    let (lib_major, lib_minor, lib_patch) = parse_libctru_version(lib_version)?;
    Ok((lib_major, lib_minor, lib_patch))
}
