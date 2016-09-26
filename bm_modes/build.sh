#!/bin/bash -ex

target=bm_modes

# "Compile images"
cd resources
# This is a little strange, since we want to both have a palette
# AND use 16 bpp
# -gb graphic *b*itmap
# -gB 16 bits per pixel
# -ft binary (for include_bytes!)
# -fh! No C header.
grit modes.png -gb -gB16 -ft bin -fh!
# -g! No graphics
# -p yes palette
grit modes.png -g! -p -ft bin -fh!
cd ..



xargo build --target=gba --release --verbose

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/$target out/${target}.gba
gbafix out/${target}.gba
open out/${target}.gba
