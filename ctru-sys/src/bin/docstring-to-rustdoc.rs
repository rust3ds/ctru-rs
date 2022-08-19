//! This script transforms _some_ Boxygen comments to Rustdoc format
//!
//! # Usage
//!
//! `cargo run --bin docstring-to-rustdoc -- [location of the bindings.rs]`
//! Example: `cargo run --bin docstring-to-rustdoc -- src/bindings.rs`
//!
//! # Transformations
//!
//! The following are _completely_ removed, but _its contents are kept_:
//! * `@brief`
//! * `@ref`
//! * `@note`
//! * `@return`
//! * `@sa`
//! * `<`
//! * `[out]` and `[in]`
//!
//! The followings are _partially_ transformed to Rustdoc format:
//! * `@param`

use std::path::Path;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let bindings_path = Path::new(args.get(1).expect("bindings.rs not provided in the args"));
    let bindings_string: String = fs::read_to_string(bindings_path)?;

    let parsed = bindings_string
        .lines()
        .map(|v| {
            // Only modify lines with the following structure: `` #[doc ... ] ``
            if v.trim_start().starts_with("#[doc") && v.trim_end().ends_with(']') {
                v.replace("@brief", "")
                    // Example: ``@param offset Offset of the RomFS...`` -> ``- offset Offset of the RomFS...``
                    // Will improve in the future
                    .replace("@param", "* ")
                    .replace("@ref", "")
                    .replace("@note", "")
                    .replace("@return", "")
                    .replace("@sa", "")
                    .replace("< ", "")
                    // Remove things like ``@param[out]``
                    .replace("[out]", "")
                    .replace("[in]", "")
                    // Trim start of the Rustdoc: ``...= " ...`` -> ``...= "...``
                    .replace("= \" ", "= \"")
                    // Double pass because _most_ annotations are at the start
                    .replace("= \" ", "= \"")
            } else {
                String::from(v)
            }
        })
        .map(|v| v + "\n")
        .collect::<String>();

    let old_bindings_path = bindings_path.to_str().unwrap().to_owned() + ".old";

    // If something fails, the original bindings are available at ``bindings.rs.old``
    fs::rename(bindings_path, &old_bindings_path)?;
    fs::write(bindings_path, parsed)?;
    fs::remove_file(&old_bindings_path)?;

    Ok(())
}
