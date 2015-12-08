# Rusty TONC

This is a fun little project run some Rust code on the GBA.
It's all inspired by the [TONC][tonc] GBA tutorial, and
I'm writing some [blog posts][blog] to go along with it.

## Getting Started.
What you will require to run these examples is [DevKitARM][devkitarm],
as well as a copy of the [Rust source code][rust] to build libcore.

I'm also assuming that you're using [multirust][multirust] to run a nightly version,
and there's a few places where that assumptions creeps into some paths, but you
can just run it without multirust (But why?).

* Install DevKitARM,
* Add the binaries to your path, (i.e. `arm-none-eabi-gcc` works).
* From the root: `git clone git@github.com:rust-lang/rust.git rust`
  (Sorry, no, I'm not adding it as a git submodule, just clone it yourself.)
* `cd first && ./do-rust.sh` should work.

(Oh, it calls `open first.gba` at the end, which opens up
an emulator for me, and is sad on non-OSX.  Sorry :/)

[tonc]: http://www.coranac.com/tonc/text/
[blog]: http://csclub.uwaterloo.ca/~tbelaire/blog/
[devkitarm]: http://sourceforge.net/projects/devkitpro/
[rust]: https://github.com/rust-lang/rust
[multirust]: https://github.com/brson/multirust

## Why?
I plan on writing an OS for my University's Real Time Operating systems class
in Rust, and I want to see if it's nuts to try and do some embedded development
at Rust's current stage of development.

And [TONC][tonc] is where I first did any C development, so it seemed like
fun to go back to that.  Maybe, if I have far too much free time, I'll even
get this working on the nds too.

Currently, it's looking like Rust is winning in some places
(I quite like my `Input` module, but is not so great in others.
Why so many `as isize` casts, and I still don't know the best
way to have pointers into raw memory.).


