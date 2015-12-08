#!/bin/bash -ex

target=key_demo

mkdir -p out
LIBS=~/.multirust/toolchains/nightly/lib/rustlib/gba.json/lib
mkdir -p $LIBS

# "Compile images"
cd resources
grit page_pic.png -gb -ah16 -gB8 -o page_pic_1 -ftb -p'!'
grit page_pic.png -gb -at16 -ah16 -gB8 -o page_pic_2 -ftb -p'!'
grit page_pic.png -g'!' -p -ftb -pu32 -pn16 -o page_pic
cd ..


# Call rustc to compile libcore for us.
# This is slow, maybe switch to a makefile?
[ -e $LIBS/libcore.rlib ] ||
  rustc --target=gba.json -Z no-landing-pads ../rust/src/libcore/lib.rs \
    --out-dir $LIBS

cargo build --target=gba.json --release --verbose

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/$target out/${target}.gba
gbafix out/${target}.gba
open out/${target}.gba
