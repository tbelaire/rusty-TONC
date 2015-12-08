#![feature(lang_items, no_std, core, core_slice_ext)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};


use gba::*;
use gba::input::Keys;

use core::mem;

const FRAME_1_IMG : &'static [u8; 2304] =
    include_bytes!("../resources/page_pic_1.img.bin");
const FRAME_2_IMG : &'static [u8; 2304] =
    include_bytes!("../resources/page_pic_2.img.bin");
const FRAME_PAL : &'static [u8; 32] =
    include_bytes!("../resources/page_pic.pal.bin");

const MEM_VRAM_BACK : u32 = (memmap::MEM_VRAM + memmap::VRAM_PAGE_SIZE);

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    let vid_mem_front = memmap::MEM_VRAM as *mut u8;
    let vid_mem_back = MEM_VRAM_BACK as *mut u8;

    for ii in 0..16 { // 16 is the height of the image.
        unsafe{
            // The image isn't a full screen wide, copy lines at a time.
            tonc_stolen::tonccpy(
                vid_mem_front.offset((ii * gfx::M4_WIDTH) as isize),
                FRAME_1_IMG as *const u8, FRAME_1_IMG.len());
            tonc_stolen::tonccpy(
                vid_mem_back.offset((ii * gfx::M4_WIDTH) as isize),
                FRAME_2_IMG as *const u8, FRAME_2_IMG.len());
        }
    }
    unsafe{
        tonc_stolen::tonccpy(
            memmap::pal_bg_mem as *mut u8,
            FRAME_PAL as *const u8, FRAME_PAL.len());
    }

    // Sets the mode.
    let mut mode4 = gfx::Mode4::new();

    let mut keys = input::Input::new();

    let mut frame = 0;
    loop{
        gfx::vid_vsync();
        keys.poll();

        if !keys.held(Keys::Start) {
            frame += 1;
            if frame == 60 {
                frame = 0;
                unsafe {
                    volatile_store(memmap::REG_DISPCNT,
                                   volatile_load(memmap::REG_DISPCNT)
                                   ^ memdef::DCNT_PAGE);
                }
            }
        }
    }
}

