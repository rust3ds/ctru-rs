use std::path::PathBuf;

fn main() {
    // Open Cargo.toml
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = format!("{manifest_dir}/Cargo.toml");
    let manifest_str = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("Could not open {manifest_path}: {e}"));
    let manifest_data: toml::Value =
        toml::de::from_str(&manifest_str).expect("Could not parse Cargo manifest as TOML");

    // Find the romfs setting and compute the path
    let romfs_dir_setting = manifest_data
        .as_table()
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("metadata"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("cargo-3ds"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("romfs_dir"))
        .and_then(toml::Value::as_str)
        .unwrap_or("romfs");
    let romfs_path = PathBuf::from(format!("{manifest_dir}/{romfs_dir_setting}"));

    // Check if the romfs path exists so we can compile the module
    if romfs_path.exists() {
        println!("cargo:rustc-cfg=romfs_exists");
    }

    println!("cargo:rerun-if-changed={manifest_dir}");
}
