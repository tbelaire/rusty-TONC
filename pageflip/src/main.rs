#![feature(lang_items, no_std, core, core_slice_ext, core_intrinsics)]

#![no_std]
#![no_main]

extern crate gba;

// These need to be exported symbols...
mod lang;
pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};


use core::intrinsics::{volatile_store, volatile_load};
use gba::*;
use gba::input::Keys;


const IMG_HEIGHT : usize = 16;
const IMG_WIDTH  : usize = 144;

// One byte per pixel in mode 4.
const IMG_BYTES : usize = IMG_HEIGHT * IMG_WIDTH * 1;

const FRAME_1_IMG : &'static [u8; IMG_BYTES] =
    include_bytes!("../resources/page_pic_1.img.bin");
const FRAME_2_IMG : &'static [u8; IMG_BYTES] =
    include_bytes!("../resources/page_pic_2.img.bin");

// 16 Entries of Color (u16).
const FRAME_PAL : &'static [u8; 16 * 2] =
    include_bytes!("../resources/page_pic.pal.bin");

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    let vid_mem_front = memmap::MEM_VRAM as *mut u8;
    let vid_mem_back = (memmap::MEM_VRAM + memmap::VRAM_PAGE_SIZE) as *mut u8;

    for ii in 0..16 as isize { // 16 is the height of the image.
        unsafe{
            // The image isn't a full screen wide, copy lines at a time.
            tonc_stolen::tonccpy(
                vid_mem_front.offset(ii * gfx::Mode4::WIDTH as isize),
                (FRAME_1_IMG as *const u8).offset(ii * IMG_WIDTH as isize),
                // IMG_WIDTH in bytes, but it's one byte per pixel, so we are ok.
                IMG_WIDTH);
            tonc_stolen::tonccpy(
                vid_mem_back.offset(ii * gfx::Mode4::WIDTH as isize),
                (FRAME_2_IMG as *const u8).offset(ii * IMG_WIDTH as isize),
                IMG_WIDTH);
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
    let mut stripe_color = gfx::PaletteIx(1);
    let mut other_color = gfx::PaletteIx(2);
    loop{
        gfx::vid_vsync();
        keys.poll();

        if keys.held(Keys::Start) {
            continue;
        }
        frame += 1;
        mode4.dot(10 + frame, 32, stripe_color);
        if frame == 60 {
            frame = 0;
            // Clear the line.
            mode4.horz_line(10, 71, 32, gfx::PaletteIx(0));
            core::mem::swap(&mut stripe_color, &mut other_color);
            mode4.page_flip();
        }
    }
}

