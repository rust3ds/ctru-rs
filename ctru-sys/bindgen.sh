#!/usr/bin/env bash

set -euo pipefail

echo "Determining libctru version..."
pacman=dkp-pacman
if ! command -v $pacman &>/dev/null; then
    pacman=pacman
    if ! command -v $pacman &>/dev/null; then
        echo >&2 "ERROR: Unable to automatically determine libctru version!"
        exit 1
    fi
fi

LIBCTRU_VERSION="$($pacman -Qi libctru | grep Version | cut -d: -f 2 | tr -d ' ')"

CTRU_SYS_VERSION="$(
    printf '%s' "$LIBCTRU_VERSION" |
    cut -d- -f1 |
    sed -E 's/^([0-9]+)\.([0-9.]+)$/\1\2/'
)"

echo "Generating bindings.rs..."
bindgen "$DEVKITPRO/libctru/include/3ds.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --must-use-type 'Result' \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --blocklist-type "__builtin_va_list" \
    --blocklist-type "__va_list" \
    --opaque-type "MiiData" \
    --with-derive-default \
    -- \
    --target=arm-none-eabi \
    --sysroot="$DEVKITARM/arm-none-eabi" \
    -isystem"$DEVKITARM/arm-none-eabi/include" \
    -I"$DEVKITPRO/libctru/include" \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D__3DS__ \
> src/bindings.rs

echo "Updating docstrings in bindings.rs..."
cargo run --quiet --package docstring-to-rustdoc -- src/bindings.rs

echo "Formatting generated files..."
cargo fmt --all

echo "Generated bindings for ctru-sys version \"${CTRU_SYS_VERSION}.x+${LIBCTRU_VERSION}\""
