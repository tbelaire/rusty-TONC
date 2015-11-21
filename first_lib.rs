#![feature(intrinsics, lang_items, no_std, no_core)]

#![no_core]

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

// // Declare some intrinsic functions that are provided to us by the compiler.
// extern "rust-intrinsic" {
//     fn overflowing_add<T>(a: T, b: T) -> T;
//     fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool);
// }

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

// // I'm not 100% sure what this function does, but references to it are compiled
// // into the program by the Rust compiler. I think it would be called in the case
// // of a program panic.
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


// // This is the top of the stack, as provided to us by the linker.
// extern {
//     static _estack: u32;
// }

#[no_mangle]
pub fn rust_main() -> u32 {
    42
}
