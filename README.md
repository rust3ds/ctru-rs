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
    [dependencies.ctru-rs]
    git="https://github.com/Furyhunter/ctru-rs"
```

**It is highly recommended to use the [template
project.](https://github.com/Furyhunter/rust3ds-template)**

## Contributing

PR's are welcome. Organization of rust specific features and wrapper
functionality has not been decided on yet.
