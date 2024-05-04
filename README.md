# ctru-rs

This repository is home of the `ctru-rs` project, which aims to bring full control of the Nintendo 3DS console to homebrew developers using Rust.

## Structure

This repository is organized as follows:

* [`ctru-rs`](./ctru-rs) - Safe, idiomatic wrapper around [`ctru-sys`](./ctru-sys).
* [`ctru-sys`](./ctru-sys) - Low-level, unsafe bindings to [`libctru`](https://github.com/devkitPro/libctru).
* [`test-runner`](./test-runner) - A helper crate for running Rust tests on 3DS (hardware or emulator).

## Getting Started

Specific information about how to use the crates is present in the individual README for each package.
Have a look at `ctru-rs`' [README.md](./ctru-rs/README.md) for a broad overview.

## Documentation

Cargo-generated [documentation](https://rust3ds.github.io/ctru-rs/crates) is available
via GitHub Pages, because the <https://docs.rs> build environment does not have `libctru`
installed.

## Original version

This project is based on the efforts of the original authors:

* [Eidolon](https://github.com/HybridEidolon)
* [FenrirWolf](https://github.com/FenrirWolf)

The old version is archived [here](https://github.com/rust3ds/ctru-rs-old).

## License

This project is distributed under the Zlib license.
