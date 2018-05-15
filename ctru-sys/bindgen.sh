#!/usr/bin/env bash

clang_version=$1

if [ -z "$clang_version" ]; then
    echo "  usage: ./bindgen.sh <clang_version>"
    echo "example: ./bindgen.sh 5.0.0"
    echo "Check your current version with \`clang -v\`."
    exit 1
fi

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/3ds.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blacklist-type "u(8|16|32|64)" \
    --blacklist-type "__builtin_va_list" \
    --blacklist-type "__va_list" \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -isystem/usr/lib/clang/$clang_version/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D_3DS \
> src/bindings.rs
