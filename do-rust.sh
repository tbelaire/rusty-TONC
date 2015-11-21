#!/bin/bash -e
set -x
FLAGS=" -nostdlib -target arm-none-eabi  -mcpu=arm7tdmi " # -march=armv7a "
clang $FLAGS -c first.c -o first.clang.o
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb -c gba_crt0.gcc.s -o gba_crt0.o
rustc --target=target.json --crate-type staticlib --emit obj -o first_lib.o first_lib.rs
clang -nostdlib -target arm-none-eabi -Tgba_cart.ld first.clang.o gba_crt0.o crti.o first_lib.o -o first.clang.elf
arm-none-eabi-objcopy -O binary first.clang.elf first.clang.gba
gbafix first.clang.gba
open first.clang.gba
