#!/bin/bash -ex

# Compile gba_crt0.s using gcc still.
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb \
    -c gba_crt0.s -o gba_crt0.o

mkdir -p libs
# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e libs/libcore.rlib ] ||
  rustc --target=target.json -Z no-landing-pads ../rust/src/libcore/lib.rs --out-dir libs

rustc -L libs -Z no-landing-pads --target=target.json rust-gba.rs -o rust-gba.elf

arm-none-eabi-objcopy -O binary rust-gba.elf rust-gba.gba
gbafix rust-gba.gba
open rust-gba.gba
