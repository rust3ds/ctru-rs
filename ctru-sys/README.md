# ctru-sys

Raw Rust bindings over the [`libctru`](https://github.com/devkitPro/libctru) C library.

## Requirements

To use the bindings provided by this crate you will need to link against the [`libctru`](https://github.com/devkitPro/libctru) library.
Consult the [`ctru-rs` wiki](https://github.com/rust3ds/ctru-rs/wiki/Getting-Started) to learn how to obtain the required packages
to use this library.

## Version

This crate's version changes according to the version of `libctru`
used to generate the bindings, with the following convention:

  * [`libctru`](https://github.com/devkitPro/libctru) version `X.Y.Z-W`
  * `ctru-sys` version `XY.Z.P+X.Y.Z-W`

  where `P` is usually 0 but may be incremented for fixes in e.g.
  binding generation, `libc` dependency bump, etc.

It may be possible to build this crate against a different version of [`libctru`](https://github.com/devkitPro/libctru),
but you may encounter linker errors or ABI issues. A build-time Cargo warning
(displayed when built with `-vv`) will be issued if the build script detects
a mismatch or is unable to check the installed [`libctru`](https://github.com/devkitPro/libctru) version.

## Generating bindings

Bindings of new versions of [`libctru`](https://github.com/devkitPro/libctru) can be built using the integrated [`bindgen.sh`](./bindgen.sh) script.

## License

This project is distributed under the Zlib license.
