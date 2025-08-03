use bindgen::callbacks::ParseCallbacks;
use bindgen::{Builder, RustTarget};
use itertools::Itertools;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

// This allows us to have a directory layout of build/*.rs which is a little
// cleaner than having all the submodules as siblings to build.rs.
mod build {
    #[cfg(feature = "layout-tests")]
    pub mod test_gen;
}

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

    let bin_dir = Path::new(devkitarm.as_str()).join("bin");
    let cc = bin_dir.join("arm-none-eabi-gcc");
    let ar = bin_dir.join("arm-none-eabi-ar");

    #[cfg(feature = "layout-tests")]
    let cpp = bin_dir.join("arm-none-eabi-g++");

    let include_path = Path::new(devkitpro.as_str()).join("libctru/include");
    let ctru_header = include_path.join("3ds.h");

    let sysroot = Path::new(&devkitarm).join("arm-none-eabi");
    let system_include = sysroot.join("include");
    let errno_header = system_include.join("errno.h");

    let gcc_version = get_gcc_version(&cc);
    let gcc_include = Path::new(&devkitarm)
        .join("lib/gcc/arm-none-eabi")
        .join(gcc_version)
        .join("include");

    let mut cc_build = cc::Build::new();
    cc_build
        .compiler(cc)
        .archiver(ar)
        .include(&include_path)
        .define("ARM11", None)
        .define("__3DS__", None)
        .flag("-march=armv6k")
        .flag("-mtune=mpcore")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=vfp")
        .flag("-mtp=soft")
        .flag("-Wno-deprecated-declarations");

    let clang = cc_build
        .clone()
        .compiler("clang")
        // bindgen uses clang, so we need to tell it where devkitARM sysroot / libs are:
        .flag("--sysroot")
        .flag(sysroot.to_str().unwrap())
        .flag("-isystem")
        .flag(system_include.to_str().unwrap())
        .flag("-isystem")
        .flag(gcc_include.to_str().unwrap())
        // Fun fact: C compilers are allowed to represent enums as the smallest
        // integer type that can hold all of its variants, meaning that enums are
        // allowed to be the size of a `c_short` or a `c_char` rather than the size
        // of a `c_int`. Some of libctru's structs contain enums that depend on
        // this narrowing property for size and alignment purposes.
        //
        // Passing this flag to clang gives approximately the same behavior as
        // gcc, so bindgen will generate enums with the proper sizes.
        .flag("-fshort-enums")
        .get_compiler();

    // Build libctru bindings
    let binding_builder = Builder::default()
        .header(ctru_header.to_str().unwrap())
        .header(errno_header.to_str().unwrap())
        .rust_target(RustTarget::nightly())
        .use_core()
        .trust_clang_mangling(false)
        .must_use_type("Result")
        .layout_tests(true)
        .ctypes_prefix("::libc")
        .prepend_enum_name(false)
        .allowlist_file(include_path.join("3ds[.]h").to_string_lossy())
        .allowlist_file(include_path.join("3ds/.*").to_string_lossy())
        .allowlist_function("__errno")
        .blocklist_function("gethost(id|name)")
        .blocklist_type("u(8|16|32|64)")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("__va_list")
        .blocklist_type("timeval")
        .blocklist_type("in_addr")
        .blocklist_type("sockaddr_storage")
        .blocklist_type("(in_addr|wchar|socklen|suseconds|sa_family|time)_t")
        .blocklist_item("SOL_CONFIG")
        //.opaque_type("MiiData") Looks like MiiData can be built by the latest bindgen
        .derive_default(true)
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("libctru_statics_wrapper"))
        .wrap_unsafe_ops(true)
        .clang_args(clang.args().iter().map(|s| s.to_str().unwrap()))
        .parse_callbacks(Box::new(CustomCallbacks));

    #[cfg(feature = "layout-tests")]
    let (test_callbacks, test_generator) = build::test_gen::LayoutTestCallbacks::new();
    #[cfg(feature = "layout-tests")]
    let binding_builder = binding_builder.parse_callbacks(Box::new(test_callbacks));

    binding_builder
        .generate()
        .expect("unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc_build
        .file(out_dir.join("libctru_statics_wrapper.c"))
        .compile("ctru_statics_wrapper");

    #[cfg(feature = "layout-tests")]
    {
        let gen_test_file = out_dir.join("generated_layout_test.rs");
        generate_layout_tests(&gen_test_file, &test_generator)
            .unwrap_or_else(|err| panic!("Failed to generate layout tests: {err}"));

        cpp_build::Config::from(cc_build)
            .compiler(cpp)
            .build(gen_test_file);
    }
}

fn get_gcc_version(path_to_gcc: &Path) -> String {
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
        .split(['.', '-'])
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

    for line in String::from_utf8_lossy(&stdout).lines() {
        let Some((_pkg, file)) = line.split_once(char::is_whitespace) else {
            println!("cargo:warning=unexpected line from pacman query: {line:?}");
            continue;
        };

        println!("cargo:rerun-if-changed={file}");
    }

    Ok(())
}

#[cfg(feature = "layout-tests")]
fn generate_layout_tests(
    output_file: &Path,
    test_generator: &build::test_gen::LayoutTestGenerator,
) -> Result<(), Box<dyn Error>> {
    // There are several bindgen-generated types/fields that we can't check:
    test_generator
            // Opaque types:
            .blocklist_type("MiiData")
            .blocklist_type("FriendInfo")
            .blocklist_type("DecryptedApproachContext")
            .blocklist_type("CFLStoreData")
            .blocklist_type("AccountInfo")
            .blocklist_type("ExistentServerAccountData")
            // Bitfields:
            .blocklist_field(
                "ExHeader_SystemInfoFlags",
                "compress_exefs_code|is_sd_application",
            )
            .blocklist_field(
                "ExHeader_Arm11StorageInfo",
                "reserved|no_romfs|use_extended_savedata_access",
            )
            .blocklist_field(
                "ExHeader_Arm11CoreInfo",
                "use_cpu_clockrate_804MHz|enable_l2c|flag[12]_unused|[no]3ds_system_mode|ideal_processor|affinity_mask",
            )
            .blocklist_field(
                "Y2RU_ConversionParams",
                "(input|output)_format|rotation|block_alignment|standard_coefficient",
            )
            .blocklist_field(
                "FS_(Program|(System|Ext)SaveData)Info",
                "mediaType"
            )
            // Variable-length arrays:
            .blocklist_field("romfs_(dir|file)", "name")
            // Bindgen anonymous types (and their associated fields):
            .blocklist_type(".*__bindgen.*")
            .blocklist_field(".*", "__bindgen.*")
            // Bindgen mangles `type` (a Rust keyword) to `type_`:
            .rename_field("type", "type_")
            .generate_layout_tests(output_file)
}
