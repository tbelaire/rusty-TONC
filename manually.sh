#!/bin/bash
set -ex
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb -c first.c -o first.gcc.o
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb -c gba_crt0.gcc.s -o gba_crt0.o
arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb -T gba_cart.ld first.gcc.o gba_crt0.o crti.o -o first.gcc.elf
arm-none-eabi-objcopy -O binary first.gcc.elf first.gcc.gba
gbafix first.gcc.gba
open first.gcc.gba
