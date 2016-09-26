#!/bin/bash -ex

mkdir -p out

xargo build --target=gba --release

# I have no idea what it's in target/gba
arm-none-eabi-objcopy -O binary target/gba/release/first out/first.gba
gbafix out/first.gba
open out/first.gba
