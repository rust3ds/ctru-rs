use bindgen::callbacks::ParseCallbacks;
use bindgen::{Builder, RustTarget};
use itertools::Itertools;

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
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rustc-link-search=native={devkitpro}/libctru/lib");

    // https://github.com/rust3ds/cargo-3ds/issues/14#issuecomment-1783991872
    // To link properly, this must be the same as the library linked by cargo-3ds when building
    // the standard library, so if `-lctru[d]` is found in RUSTFLAGS we always defer to that
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let cargo_rustflags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
    let rustflags_libctru = cargo_rustflags
        .split('\x1F')
        // Technically this  could also be `-l ctru`, or `-lstatic=ctru` etc.
        // but for now we'll just rely on cargo-3ds implementation to pass it like this
        .find(|flag| flag.starts_with("-lctru"))
        .and_then(|flag| flag.strip_prefix("-l"));

    let linked_libctru = rustflags_libctru.unwrap_or_else(|| {
        let debuginfo = env::var("DEBUG").unwrap();
        match debuginfo.as_str() {
            // Normally this should just be "true" or "false", but just in case,
            // we don't support all the different options documented in
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            // so just default to linking with debuginfo if it wasn't disabled
            "0" | "false" | "none" => "ctru",
            _ => "ctrud",
        }
    });

    println!("cargo:rustc-link-lib=static={linked_libctru}");

    detect_and_track_libctru();

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
        .layout_tests(true)
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

fn detect_and_track_libctru() {
    let pacman = match which::which("dkp-pacman")
        .or_else(|err1| which::which("pacman").map_err(|err2| format!("{err1}; {err2}")))
    {
        Ok(pacman) => pacman,
        Err(err) => {
            println!("cargo:warning=unable to find `pacman` or `dkp-pacman`: {err}");
            return;
        }
    };

    match get_libctru_version(&pacman) {
        Ok((maj, min, patch, rel)) => {
            let version = format!("{maj}.{min}.{patch}-{rel}");
            eprintln!("using libctru version {version}");
            // These are exported as build script output variables, accessible
            // via `env::var("DEP_CTRU_<key>")` in other crates' build scripts.
            // https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
            println!("cargo:VERSION={version}");
            println!("cargo:MAJOR_VERSION={maj}");
            println!("cargo:MINOR_VERSION={min}");
            println!("cargo:PATCH_VERSION={patch}");
            println!("cargo:RELEASE={rel}");
        }
        Err(err) => println!("cargo:warning=unknown libctru version: {err}"),
    }

    if let Err(err) = track_libctru_files(&pacman) {
        println!("cargo:warning=unable to track `libctru` files for changes: {err}");
    }
}

fn get_libctru_version(pacman: &Path) -> Result<(String, String, String, String), Box<dyn Error>> {
    let Output { stdout, .. } = Command::new(pacman)
        .args(["--query", "libctru"])
        .stderr(Stdio::inherit())
        .output()?;

    let output_str = String::from_utf8_lossy(&stdout);

    let (_pkg, lib_version) = output_str
        .split_once(char::is_whitespace)
        .ok_or_else(|| format!("unexpected pacman output format: {output_str:?}"))?;

    let lib_version = lib_version.trim();

    parse_libctru_version(lib_version).map_err(Into::into)
}

fn parse_libctru_version(version: &str) -> Result<(String, String, String, String), String> {
    version
        .split(|c| c == '.' || c == '-')
        .map(String::from)
        .collect_tuple()
        .ok_or_else(|| format!("unexpected number of version segments: {version:?}"))
}

fn track_libctru_files(pacman: &Path) -> Result<(), String> {
    let stdout = match Command::new(pacman)
        .args(["--query", "--list", "libctru"])
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(Output { stdout, status, .. }) if status.success() => stdout,
        Ok(Output { status, .. }) => {
            return Err(format!("pacman query failed with status {status}"));
        }
        Err(err) => {
            return Err(format!("pacman query failed: {err}"));
        }
    };

    for line in String::from_utf8_lossy(&stdout).trim().split('\n') {
        let Some((_pkg, file)) = line.split_once(char::is_whitespace) else {
            println!("cargo:warning=unexpected line from pacman query: {line:?}");
            continue;
        };

        println!("cargo:rerun-if-changed={file}");
    }

    Ok(())
}
