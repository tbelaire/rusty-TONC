#!/bin/bash -ex

clang -nostdlib -target arm-none-eabi -mcpu=arm7tdmi -c second.c -o second.o

# Compile gba_crt0.s using gcc still.
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb \
    -c gba_crt0.s -o gba_crt0.o

# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e libs/libcore.rlib ] ||
  rustc --target=target.json -Z no-landing-pads ../rust/src/libcore/lib.rs --out-dir libs/ --emit link,obj

# I don't want to use anything in here, but it needs to exist.
[ -e libs/libcompiler-rt.a ] ||
    arm-none-eabi-ar m libcompiler-rt.a

# Call rustc to compile our first Rust object
rustc -L libs -Z no-landing-pads --target=target.json --emit obj  rust-gba.rs

clang -nostdlib -target arm-none-eabi -Tgba_cart.ld \
    -Wl,--gc-sections \
    rust-gba.o libs/core.o \
    gba_crt0.o crti.o -o rust-gba.elf

arm-none-eabi-objcopy -O binary rust-gba.elf rust-gba.gba
gbafix rust-gba.gba
open rust-gba.gba
