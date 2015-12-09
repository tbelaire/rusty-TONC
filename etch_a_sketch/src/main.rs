#![feature(lang_items, no_std, core, core_slice_ext)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

use gba::gfx;
use gba::gfx::Color;
use gba::input;
use gba::input::Keys;


#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut m = gfx::Mode3::new();
    let mut keys = input::Input::new();
    let mut x : i32 = 120;
    let mut y : i32 = 80;

    // Avoid repeated typecasts.
    let width = gfx::Mode3::WIDTH as i32;
    let height = gfx::Mode3::HEIGHT as i32;

    let colors = [Color::rgb15(31, 0, 0),
                  Color::rgb15(0, 31, 0),
                  Color::rgb15(0, 0, 31)];
    let mut color_ix = 0;
    loop {
        gfx::vid_vsync();

        keys.poll();
        x += keys.tri_horz();
        y += keys.tri_vert();

        if keys.hit(Keys::L) {
            // % is rem, not mod, so need to keep it positive.
            color_ix -= 1;
        } else if keys.hit(Keys::R) {
            color_ix += 1;
        }
        color_ix = (colors.len() + color_ix) % colors.len();

        // This keeps everything positive.
        x = (x + width) % width;
        y = (y + height) % height;


        m.dot(x, y, colors[color_ix]);
        m.dot((x+1) % width,
                y,
                colors[(color_ix + 1) % colors.len()]);
        m.dot((x+1) % width,
              (y+1) % height,
              colors[(color_ix + 2) % colors.len()]);
    }
}

