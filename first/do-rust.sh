#!/bin/bash -ex

mkdir -p out
LIBS=.rust/lib/gba.json/
mkdir -p $LIBS

# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e $LIBS/libcore.rlib ] ||
  rustc --target=gba.json -Z no-landing-pads ../rust/src/libcore/lib.rs \
    --out-dir $LIBS

#rustc -L libs -Z no-landing-pads --target=target.json src/rust-gba.rs -o out/rust-gba.elf

cargo build --target=gba.json --release

arm-none-eabi-objcopy -O binary target/target/release/rust-gba out/rust-gba.gba
gbafix out/rust-gba.gba
open out/rust-gba.gba
