#!/bin/bash -ex
# -ex will stop on the first error, and print every command
# as it gets run.

clang -nostdlib -target arm-none-eabi -mcpu=arm7tdmi -c first.c -o first.clang.o
# Clang is a cross compiler, so we don't need to rebuild it from source.
# We do need to pass it the target triple (arm-none-eabi), and what cpu
# we are using.
# Also, since we aren't using libc, we pass -nostdlib

arm-none-eabi-gcc -nostdlib -mthumb-interwork -mthumb \
    -c gba_crt0.s -o gba_crt0.o
# Compile gba_crt0.s using gcc still.

clang -nostdlib -target arm-none-eabi -Tgba_cart.ld \
    first.clang.o gba_crt0.o crti.o -o first.clang.elf

# Call clang to link everything.  Actually, clang will just call out to
# arm-none-eabi-gcc to link everything properly.  It needs to be in the path.

arm-none-eabi-objcopy -O binary first.clang.elf first.clang.gba
gbafix first.clang.gba

open first.clang.gba
