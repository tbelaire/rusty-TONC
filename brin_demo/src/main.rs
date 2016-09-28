#![feature(lang_items, static_in_const, core_intrinsics)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};


use gba::*;

use core::intrinsics::volatile_store;

const BRIN_TILES: &[u8; 992] = include_bytes!("../resources/brin-full.img.bin");
const BRIN_MAP: &[u8; 4096] = include_bytes!("../resources/brin-full.map.bin");
const BRIN_PAL: &[u8; 512] = include_bytes!("../resources/brin-full.pal.bin");

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    unsafe {
        tonc_stolen::tonccpy(memmap::pal_bg_mem as *mut u8,
                             BRIN_PAL as *const u8,
                             BRIN_PAL.len());
        tonc_stolen::tonccpy(memmap::tile_mem.offset(0) as *mut u8,
                             BRIN_TILES as *const u8,
                             BRIN_TILES.len());
        tonc_stolen::tonccpy(memmap::se_mem.offset(30) as *mut u8,
                             BRIN_MAP as *const u8,
                             BRIN_MAP.len());


    }
    unsafe {
        use gba::memdef::*;
        volatile_store(gba::memmap::REG_BG0CNT,
                       (BG_CBB(0) | BG_SBB(30) | BG_4BPP | BG_REG_64x32) as u16);
        volatile_store(gba::memmap::REG_DISPCNT, DCNT_MODE0 | DCNT_BG0);
    }

    let mut keys = input::Input::new();

    let mut x = 192;
    let mut y = 64;

    loop {
        gfx::vid_vsync();
        keys.poll();

        x += keys.tri_horz();
        y += keys.tri_vert();

        unsafe {
            volatile_store(memmap::REG_BG0HOFS, x as u16);
            volatile_store(memmap::REG_BG0VOFS, y as u16);
        }
    }
}
