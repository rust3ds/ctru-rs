# ctru-sys

Raw Rust bindings over the [`libctru`](https://github.com/devkitPro/libctru) C library.

## Requirements

To use the bindings provided by this crate you will need to link against the [`libctru`](https://github.com/devkitPro/libctru) library.
Consult the [`ctru-rs` wiki](https://github.com/rust3ds/ctru-rs/wiki/Getting-Started) to learn how to obtain the required packages
to use this library.

## Version

Crate bindings are generated at build time, so the available APIs will depend on the
installed version of `libctru` when the crate is built. If you want to check
what version of `libctru` is being built, you can examine these environment
variables with [`env!`](https://doc.rust-lang.org/std/macro.env.html):

* `LIBCTRU_VERSION`: full version string (e.g. `"2.3.1-4"`)
* `LIBCTRU_MAJOR`: major version (e.g. `"2"` for version `2.3.1-4`)
* `LIBCTRU_MINOR`: minor version (e.g. `"3"` for version `2.3.1-4`)
* `LIBCTRU_PATCH`: patch version (e.g. `"1"` for version `2.3.1-4`)
* `LIBCTRU_RELEASE`: release version (e.g. `"4"` for version `2.3.1-4`)

## License

This project is distributed under the Zlib license.
