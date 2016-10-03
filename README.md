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
    git="https://github.com/rust3ds/ctru-rs"
```

**It is highly recommended to use the [template
project.](https://github.com/rust3ds/rust3ds-template)**

## Contributing

PR's are welcome. Organization of rust specific features and wrapper
functionality has not been decided on yet.

## License

Copyright (C) Ronald Kinard, 2015-2016

As with the original ctrulib, this library is licensed under zlib. This
applies to every file in the tree, unless otherwise noted.

    This software is provided 'as-is', without any express or implied
    warranty.  In no event will the authors be held liable for any
    damages arising from the use of this software.

    Permission is granted to anyone to use this software for any
    purpose, including commercial applications, and to alter it and
    redistribute it freely, subject to the following restrictions:

    1. The origin of this software must not be misrepresented; you
       must not claim that you wrote the original software. If you use
       this software in a product, an acknowledgment in the product
       documentation would be appreciated but is not required.
    2. Altered source versions must be plainly marked as such, and
       must not be misrepresented as being the original software.
    3. This notice may not be removed or altered from any source
       distribution.

Rust is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See [LICENSE-APACHE](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE), [LICENSE-MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT), and [COPYRIGHT](https://github.com/rust-lang/rust/blob/master/COPYRIGHT) for details.
