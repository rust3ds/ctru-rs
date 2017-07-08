# ctru-rs

A Rust wrapper library for smealum's [ctrulib](https://github.com/smealum/ctrulib).

See the [3DS project template](https://github.com/rust3ds/rust3ds-template) for instructions on how to use this library.

## Structure

This repository is organized as follows:
* `ctru-rs`: Safe, idiomatic wrapper around `ctru-sys`.
* `ctru-sys`: Low-level, unsafe bindings to ctrulib
* `ctr-std`: A partial implementation of the Rust standard library for the 3DS.

## License

Copyright (C) Rust 3DS Project authors, 2015-2016

See AUTHORS.md.

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

