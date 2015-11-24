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
/*&[
    0x1F,0x7C,0x8C,0x69,0x08,0x41,0x73,0x3A,0x08,0x25,0x18,0x5B,0x18,0x5F,0x18,0x63,
    0x18,0x67,0x18,0x6B,0x18,0x6F,0xB6,0x72,0x18,0x77,0x18,0x7B,0x18,0x7F,0x7B,0x77];
    */
// Where the buttons start in the palette.
const BTN_PAL : isize = 5;




#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    unsafe{
    tonc_stolen::tonccpy(
         memmap::vid_mem as *mut u8, GBA_IMG as *const u8, GBA_IMG.len());
    tonc_stolen::tonccpy(
         memmap::pal_bg_mem as *mut u8, GBA_PAL as *const u8, GBA_PAL.len());


    }
    // Sets the mode.
    let mut mode4 = gfx::Mode4::new();

    let mut keys = input::Input::new();

    let mut frame = 0;
    for i in 0..160 {
        mode4.horz_line(0, 4, i, gfx::PaletteIx((i / 8) as u8));
    }
    loop{
        gfx::vid_vsync();
        keys.poll();
        if keys.hit(Keys::Start) {
            break;
        }
    }
    loop{
        gfx::vid_vsync();

        if frame & 7 == 0 {
            keys.poll();
        }

        for i in 0..10 {
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

