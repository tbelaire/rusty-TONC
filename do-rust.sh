#!/bin/bash -ex

clang -nostdlib -target arm-none-eabi -mcpu=arm7tdmi -c second.c -o second.o

# Compile gba_crt0.s using gcc still.
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb \
    -c gba_crt0.s -o gba_crt0.o

# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e libs/libcore.rlib ] ||
  rustc --target=target.json -o libs/libcore.rlib ../../rust/src/libcore/lib.rs

# Call rustc to compile our first Rust object
rustc -L libs --target=target.json --crate-type staticlib --emit obj -o first_lib.o first_lib.rs

clang -nostdlib -target arm-none-eabi -Tgba_cart.ld \
    second.o gba_crt0.o crti.o first_lib.o -o first.rustc.elf

arm-none-eabi-objcopy -O binary first.rustc.elf first.rustc.gba
gbafix first.rustc.gba
open first.rustc.gba
