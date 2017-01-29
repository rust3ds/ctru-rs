A work-in-progress port of the Rust Standard Library for the Nintendo 3DS, based on [ctrulib](https://github.com/smealum/ctrulib/) and the [devkitARM](http://devkitPro.org) toolchain.

## Structure

This library aims to mimick the Rust standard library's public interface as closely as possible, exposing functionality that is common between the 3DS and other platforms. System-specific functionality such as control input, save file management, GPU features, and so forth are implemented in `ctru-rs`.
