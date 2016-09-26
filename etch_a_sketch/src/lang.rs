
use core;

#[lang = "panic_fmt"]
extern fn panic_fmt(_args: &core::fmt::Arguments,
                    _file: &str,
                    _line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() -> ! { loop {} }

// I'm not 100% sure what this function does, but references to it are compiled
// into the program by the Rust compiler. I think it would be called in the case
// of a program panic.
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {
    loop {}
}

