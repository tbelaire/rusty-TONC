#![feature(lang_items, no_std, core, core_slice_ext)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};


use gba::*;
use gba::input::Keys;

use core::mem;

const GBA_IMG : &'static [u8; 38400] = include_bytes!("../resources/gba.img.bin");
const GBA_PAL : &'static [u8; 32]    = include_bytes!("../resources/gba.pal.bin");
// Where the buttons start in the palette.
const BTN_PAL : isize = 5;




#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    unsafe{
    tonc_stolen::tonccpy(
         memmap::vid_mem as *mut u8, GBA_IMG as *const u8, GBA_IMG.len());
    tonc_stolen::tonccpy(
         memmap::pal_bg_mem as *mut u8, GBA_PAL as *const u8, GBA_IMG.len());

    volatile_store(memmap::REG_DISPCNT,
                   memdef::DCNT_MODE4 | memdef::DCNT_BG2);
    }

    let mut keys = input::Input::new();

    let mut frame = 0;
    loop{
        gfx::vid_vsync();

        if frame & 7 == 0 {
            keys.poll();
        }

        for i in 0..9 {
            let k: Keys = unsafe{ mem::transmute(1 << i) };

            let color = if keys.hit(k) {
                gfx::Color::RED
            } else if keys.released(k) {
                gfx::Color::YELLOW
            } else if keys.held(k) {
                gfx::Color::LIME
            } else {
                gfx::Color::rgb15(27,27,29) // Grey
            };

            unsafe {
                *memmap::pal_bg_mem.offset(BTN_PAL + i) = color;
            }
        }

        frame += 1;
    }
}

