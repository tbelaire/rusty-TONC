#!/bin/bash -ex

target=key_demo

mkdir -p out

# "Compile images"
cd resources
# Cut the palette off at 16 entries
grit gba.png -gb -gB8 -ft bin -pe16
cd ..


xargo build --target=gba --release --verbose

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/$target out/${target}.gba
gbafix out/${target}.gba
open out/${target}.gba
