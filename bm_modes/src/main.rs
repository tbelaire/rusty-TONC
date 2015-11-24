#![feature(lang_items, no_std, core)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};


use gba::*;
use gba::input::Keys;

use core::ptr;

const Modes_img : &'static [u8; 19200] = include_bytes!("../resources/modes.img.bin");
const Modes_pal : &'static [u8; 32]    = include_bytes!("../resources/modes.pal.bin");




#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    /*
    int mode= 3;
    REG_DISPCNT= mode | DCNT_BG2;

    // Copy the data and palette to the right
    // addresses
    memcpy(vid_mem, modesBitmap, modesBitmapLen);
    memcpy(pal_bg_mem, modesPal, modesPalLen);

    while(1)
    {
    // Wait till VBlank before doing anything
    vid_vsync();

    // Check keys for mode change
    key_poll();
    if(key_hit(KEY_LEFT) && mode>3)
    mode--;
    else if(key_hit(KEY_RIGHT) && mode<5)
    mode++;

    // Change the mode
    REG_DISPCNT= mode | DCNT_BG2;
    }
    */
    let mut mode : u32 = 3;
    unsafe{
    gba::volatile_store(gba::memmap::REG_DISPCNT, mode | gba::memdef::DCNT_BG2);

    // let src = Modes_img as *const u8 as *const u32;
    // let dst = memmap::vid_mem as *mut u32;

    // for i in 0..4800 {
    //     *dst.offset(i) = *src.offset(i);
    // }

    tonc_stolen::tonccpy(
         gba::memmap::vid_mem as *mut u8, Modes_img as *const u8, 19200);
    tonc_stolen::tonccpy(
        gba::memmap::pal_bg_mem as *mut u8, Modes_pal as *const u8, 32);
    }

    let mut keys = gba::input::Input::new();

    loop{
        gba::gfx::vid_vsync();

        keys.poll();
        if keys.hit(Keys::Left) && mode > 3 {
            mode -= 1;
        } else if keys.hit(Keys::Right) && mode < 5 {
            mode += 1;
        }


        unsafe {
            volatile_store(
                memmap::REG_DISPCNT,
                mode | memdef::DCNT_BG2);
        }
    }
}

