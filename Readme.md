# Rusty TONC

This is a fun little project run some Rust code on the GBA.
It's all inspired by the [TONC][tonc] GBA tutorial, and
I'm writing some [blog posts][blog] to go along with it.

## Getting Started.
What you will require to run these examples is [DevKitARM][devkitarm],
as well as the cross compiling took [`xargo`][xargo].

* Install DevKitARM,
* Add the binaries to your path, (i.e. `arm-none-eabi-gcc` works).
* `cd first && ./build.sh` should work.

(Oh, it calls `open first.gba` at the end, which opens up
an emulator for me, and is OSX specific.  Sorry :/)

[tonc]: http://www.coranac.com/tonc/text/
[blog]: http://csclub.uwaterloo.ca/~tbelaire/blog/
[devkitarm]: http://sourceforge.net/projects/devkitpro/
[rust]: https://github.com/rust-lang/rust
[xargo]: https://github.com/japaric/xargo

## Why?
[TONC][tonc] is where I first did any C development, so it seemed like
fun to go back to that.  Maybe, if I have far too much free time, I'll even
get this working on the nds too.

Currently, it's looking like Rust is nicer than C in some places
(I quite like my `Input` module, but is not so great in others.
Why so many `as isize` casts, and I still don't know the best
way to have pointers into raw memory.).


