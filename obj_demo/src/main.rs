#![feature(lang_items, static_in_const, core_intrinsics)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};


use gba::*;
use gba::oam::*;
use gba::memdef::*;

use core::intrinsics::volatile_store;
use core::mem;

const METR_TILES: &[u8; 4096] = include_bytes!("../resources/metr.img.bin");
const METR_PAL: &[u8; 512] = include_bytes!("../resources/metr.pal.bin");

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {

    unsafe {
        tonc_stolen::tonccpy(memmap::tile_mem.offset(4) as *mut u8,
                             METR_TILES as *const u8,
                             METR_TILES.len());
        tonc_stolen::tonccpy(memmap::pal_obj_mem as *mut u8,
                             METR_PAL as *const u8,
                             METR_PAL.len());
    }
    let mut obj_buffer: [obj_attr; 128] = [obj_attr{attr0: 0, attr1: 0, attr2: 0, fill:0}; 128];
    oam_init(&mut obj_buffer[..]);
    unsafe {
        use gba::memdef::*;
        volatile_store(gba::memmap::REG_DISPCNT, DCNT_MODE0 | DCNT_OBJ | DCNT_OBJ_1D);
    }
    obj_test(&mut obj_buffer);

    loop{}
}

fn obj_test(obj_buffer: &mut [obj_attr; 128]) {

    let mut oam_mem: &mut [obj_attr] = unsafe {
        core::slice::from_raw_parts_mut(
            memmap::oam_mem,
            memmap::OAM_SIZE as usize / mem::size_of::<obj_attr>())
    };
    let mut keys = input::Input::new();

    let mut x: i32 = 96;
    let mut y: i32 = 32;
    let mut tid: i32 = 0;
    let mut pb: i32 = 0;

    {
        let metr: &mut obj_attr = &mut obj_buffer[0];
        metr.set_attr( ATTR0_SQUARE,
                       ATTR1_SIZE_64,
                       ATTR2_PALBANK(pb as u16) | tid as u16);
        metr.set_pos(x, y);
    }

    loop {
        gfx::vid_vsync();
        keys.poll();

        x += 2 * keys.tri_horz();
        y += 2 * keys.tri_vert();

        tid += keys.tri_shoulder_hit();
        pb  += keys.tri_fire_hit();

        {
            let metr: &mut obj_attr = &mut obj_buffer[0];
            metr.set_pos(x, y);
            metr.set_attr2(tid, pb, 0);
        }
        oam_copy(oam_mem, &obj_buffer[..1]);  // only need to update one


    }
}
