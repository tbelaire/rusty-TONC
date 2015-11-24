// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();
    config.compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar");
    config.file("src/tonc_core.c");
    config.compile("libtonc.a");
}
