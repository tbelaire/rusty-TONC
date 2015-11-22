#!/bin/bash -ex

mkdir -p libs
mkdir -p out

# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e libs/libcore.rlib ] ||
  rustc --target=target.json -Z no-landing-pads ../rust/src/libcore/lib.rs --out-dir libs

rustc -L libs -Z no-landing-pads --target=target.json src/rust-gba.rs -o out/rust-gba.elf

arm-none-eabi-objcopy -O binary out/rust-gba.elf out/rust-gba.gba
gbafix out/rust-gba.gba
open out/rust-gba.gba
