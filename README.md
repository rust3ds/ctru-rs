# ctru-rs

A Rust wrapper library for smealum's
[ctrulib](https://github.com/smealum/ctrulib). Intended for use only when
targeting CTR.

## How to build

1. Install the devkitARM toolchain for your system. Make sure `DEVKITPRO` is
   set in your environment.
2. Modify ~/.cargo/config and add the following lines:

```toml
    [target.3ds]
    ar = "/path/to/arm-none-eabi-ar"
```

3. Build with `cargo build --target 3ds.json`.
4. A dkA linkable .rlib (static library) will be generated. Read below for
instructions on using.

## How to use

You can build your homebrew projects through Cargo just as easily as any other
platform. Add this to your `Cargo.toml`:

```toml
    [dependencies.core]
    git="https://github.com/hackndev/rust-libcore"

    [dependencies.ctru-rs]
    git="https://github.com/Furyhunter/ctru-rs"
```

Copy the `3ds.json` target file to your project root and pass `--target
3ds.json` whenever building. The instructions above for cargo configuration and
environment still apply. It's recommended that you put the `bin` dir in your
dkA root to your path.

Your homebrew's crate must specify that it does not use `std`, because `std`
is not available for the 3ds. `core` is a small subset of `std`'s functionality
that is platform agnostic. You can `use core::prelude::*` to recover much of
std's prelude, after `extern crate core`. This library makes use of core only.

## Contributing

This is a thin wrapper library **only**, any additional Rust-centric support
code for the 3DS will be put into another package. However, the wrapper is
incomplete, so PR's to finish it are helpful.
