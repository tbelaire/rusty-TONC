#!/bin/bash -e
set -x
FLAGS=" -nostdlib -target arm-none-eabi  -mcpu=arm7tdmi "
clang $FLAGS -c second.c -o second.o
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb -c gba_crt0.s -o gba_crt0.o
rustc --target=target.json --crate-type staticlib --emit obj -o first_lib.o first_lib.rs
clang -nostdlib -target arm-none-eabi -Tgba_cart.ld second.o gba_crt0.o crti.o first_lib.o -o first.rustc.elf
arm-none-eabi-objcopy -O binary first.rustc.elf first.rustc.gba
gbafix first.rustc.gba
open first.rustc.gba
