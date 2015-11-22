#!/bin/bash -ex

mkdir -p out
LIBS=~/.multirust/toolchains/nightly/lib/rustlib/gba.json/lib
mkdir -p $LIBS

# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e $LIBS/libcore.rlib ] ||
  rustc --target=gba.json -Z no-landing-pads ../rust/src/libcore/lib.rs \
    --out-dir $LIBS

cargo build --target=gba.json --release

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/first out/first.gba
gbafix out/first.gba
open out/first.gba
