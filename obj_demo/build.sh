#!/bin/bash -ex

target=obj_demo

mkdir -p out

# "Compile images"
cd resources
# -gt graphic tile
# -gB 4 bits per pixel (index into 16 element palette)
# -m  Include a map
# -mRtfp reduce
# -mLs use screenblocks
# -p use palette
# -ft binary (for include_bytes!)
# -fh C header, to inspect the final settings and sizes
# grit metr.png -g -gt -gB4 -m -mRtfp -mLs -p -fh -ft bin
cd ..



xargo build --target=gba --release --verbose

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/$target out/${target}.gba
gbafix out/${target}.gba
open out/${target}.gba
