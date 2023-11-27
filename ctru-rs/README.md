# ctru-rs

Safe and idiomatic Rust wrapper around [`libctru`](https://github.com/devkitPro/libctru).

Documentation for the `master` branch can be found [here](https://rust3ds.github.io/ctru-rs/crates/ctru).

## Getting Started

Thoroughly read the [`ctru-rs` wiki](https://github.com/rust3ds/ctru-rs/wiki/Getting-Started) to meet the requirements
and to understand what it takes to develop homebrew software on the Nintendo 3DS family of consoles.
After that, you can simply add the crate as a dependency to your project and build your final binary by using [`cargo-3ds`](https://github.com/rust3ds/cargo-3ds)
or by manually compiling for the `armv6k-nintendo-3ds` target.

## Examples

Many examples to demonstrate the `ctru-rs` functionality are available in the [`examples`](./examples/) folder. Simply run them via

```bash
cargo 3ds run --example <example-name>
```

## License

This project is distributed under the Zlib license.
