A work-in-progress port of the Rust Standard Library for the Nintendo 3DS, based on [ctrulib](https://github.com/smealum/ctrulib/) and the [devkitARM](http://devkitPro.org) toolchain.

## Structure

This library aims to mimick the Rust standard library's public interface as closely as possible, exposing functionality that is common between the 3DS and other platforms. System-specific functionality such as control input, save file management, GPU features, and so forth are implemented in `ctru-rs`.

## Implemented modules

# Stable modules
* `any`
* `ascii`
* `borrow`
* `boxed`
* `cell`
* `char`
* `clone`
* `cmp`
* `collections`
* `convert`
* `default`
* `error`
* `f32`
* `f64`
* `ffi`
* `fmt`
* `fs`        Both `sdmc:/` and `romfs:/` paths are supported in standard file operations
* `hash`
* `i8`
* `i16`
* `i32`
* `i64`
* `io`
* `isize`
* `iter`
* `marker`
* `mem`
* `net`       Anything not involving IPv6 should work after initializing the `Soc` service in `ctru-rs`
* `num`
* `ops`
* `option`
* `os`        The modules in here should work, but they aren't well-tested
* `panic`
* `path`
* `prelude`
* `ptr`
* `rc`
* `result`
* `slice`
* `str`
* `string`
* `sync`
* `thread`    Threads are able to be spawned, but without the ability to pin to a specific core or set thread priority
* `time`
* `u8`
* `u16`
* `u32`
* `u64`
* `usize`
* `vec`

# Nightly modules
* `heap`
* `i128`
* `intrinsics`
* `raw`
* `u128`

# Non-functional or partially functional modules
* `env`       argc/argv can be implemented but have not been yet
* `process`   Unable to be implemented
