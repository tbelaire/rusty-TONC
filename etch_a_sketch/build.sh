#!/bin/bash -ex
target=etch_a_sketch

xargo build --target=gba --release

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/$target out/${target}.gba
gbafix out/${target}.gba
open out/${target}.gba
