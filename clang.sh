#!/bin/bash -e
set -x
FLAGS=" -nostdlib -target arm-none-eabi  -mcpu=arm7tdmi "
clang $FLAGS -c first.c -o first.clang.o
# clang $FLAGS -c gba_crt0.s -o gba_crt0.o
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb -c gba_crt0.gcc.s -o gba_crt0.o
clang -nostdlib -target arm-none-eabi -Tgba_cart.ld first.clang.o gba_crt0.o crti.o -o first.clang.elf
arm-none-eabi-objcopy -O binary first.clang.elf first.clang.gba
gbafix first.clang.gba
open first.clang.gba
