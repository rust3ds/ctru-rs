//! This script transforms _some_ Boxygen comments to Rustdoc format
//!
//! # Usage
//!
//! `cargo docstring-to-rustdoc [location of the bindings.rs]`
//! Example: `cargo docstring-to-rustdoc src/bindings.rs`
//!
//! # Transformations
//!
//! Check [doxygen-rs docs](https://techie-pi.github.io/doxygen-rs/doxygen_rs/)

use std::path::Path;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let bindings_path = Path::new(args.get(1).expect("bindings.rs not provided in the args"));
    let bindings = fs::read_to_string(bindings_path)?;

    let parsed = doxygen_rs::transform_bindgen(bindings.as_str());

    let old_bindings_path = bindings_path.to_str().unwrap().to_owned() + ".old";

    // If something fails, the original bindings are available at ``bindings.rs.old``
    fs::rename(bindings_path, &old_bindings_path)?;
    fs::write(bindings_path, parsed)?;
    fs::remove_file(&old_bindings_path)?;

    Ok(())
}
