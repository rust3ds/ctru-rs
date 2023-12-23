# ctru-sys

Raw Rust bindings over the [`libctru`](https://github.com/devkitPro/libctru) C library.

Documentation for the latest devkitPro release
[on Docker Hub](https://hub.docker.com/r/devkitpro/devkitarm/)
can be found [here](https://rust3ds.github.io/ctru-rs/crates/ctru_sys).

## Requirements

To use the bindings provided by this crate you will need to link against the [`libctru`](https://github.com/devkitPro/libctru) library.
Consult the [`ctru-rs` wiki](https://github.com/rust3ds/ctru-rs/wiki/Getting-Started) to learn how to obtain the required packages
to use this library.

## Version

Crate bindings are generated at build time, so the available APIs will depend on the
installed version of `libctru` when the crate is built. If you want to check
what version of `libctru` is being built, you can examine these environment
variables from your crate's build script via to its
[`links` variables](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key):

* `DEP_CTRU_VERSION`: full version string (e.g. `"2.3.1-4"`)
* `DEP_CTRU_MAJOR_VERSION`: major version (e.g. `"2"` for version `2.3.1-4`)
* `DEP_CTRU_MINOR_VERSION`: minor version (e.g. `"3"` for version `2.3.1-4`)
* `DEP_CTRU_PATCH_VERSION`: patch version (e.g. `"1"` for version `2.3.1-4`)
* `DEP_CTRU_RELEASE`: release version (e.g. `"4"` for version `2.3.1-4`)

## License

This project is distributed under the Zlib license.
