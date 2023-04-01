use std::env;
use std::error::Error;
use std::process::{Command, Output, Stdio};

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();
    let profile = env::var("PROFILE").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rustc-link-search=native={dkp_path}/libctru/lib");
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
}

fn parse_version(version: &str) -> Result<(String, String, String), &str> {
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
        return Err(format!(
            "libctru version is {lib_version} but this crate was built for {crate_built_version}"
        ))?;
    }

    let Output { stdout, .. } = Command::new(pacman)
        .args(["--query", "--list", "libctru"])
        .stderr(Stdio::inherit())
        .output()?;

    for line in String::from_utf8_lossy(&stdout).split('\n') {
        let Some((_pkg, file)) = line.split_once(char::is_whitespace)
        else { continue };

        println!("cargo:rerun-if-changed={file}");
    }

    let (lib_major, lib_minor, lib_patch) = parse_version(lib_version)?;
    Ok((lib_major, lib_minor, lib_patch))
}
