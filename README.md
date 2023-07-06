# ctru-rs

A Rust wrapper around [libctru](https://github.com/devkitPro/libctru).

## Structure

This repository is organized as follows:

* `ctru-rs`: Safe, idiomatic wrapper around `ctru-sys`

* `ctru-sys`: Low-level, unsafe bindings to `libctru`.

  This crate's version changes according to the version of `libctru`
  used to generate the bindings, with the following convention:

  * `libctru` version `X.Y.Z-W`
  * `ctru-sys` version `XY.Z.P+X.Y.Z-W`

  where `P` is usually 0 but may be incremented for fixes in e.g.
  binding generation, `libc` dependency bump, etc.

  It may be possible to build this crate against a different version of `libctru`,
  but you may encounter linker errors or ABI issues. A build-time Cargo warning
  (displayed when built with `-vv`) will be issued if the build script detects
  a mismatch or is unable to check the installed `libctru` version.

## Original version

This project is based on the efforts the original authors:
 * [Eidolon](https://github.com/HybridEidolon)
 * [FenrirWolf](https://github.com/FenrirWolf)

The old version is archived [here](https://github.com/rust3ds/ctru-rs-old).
