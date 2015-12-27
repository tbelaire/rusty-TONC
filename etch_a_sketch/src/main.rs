#![feature(lang_items)]

#![no_std]
#![no_main]

extern crate gba;

mod lang;

pub use lang::{__aeabi_unwind_cpp_pr0, __aeabi_unwind_cpp_pr1};

use gba::gfx;
use gba::gfx::Color;
use gba::input;

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    let mut m = gfx::Mode3::new();
    let mut keys = input::Input::new();

    // Location of the cursor
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
        // Wait for vsync, so we only draw once per frame.
        gfx::vid_vsync();

        // Save the current state of the keys.
        // This keeps the previous state around,
        // so we can check for button *presses*,
        // and tell that apart from holding the button.
        keys.poll();

        // These are neat little helpers functions that encapsulate
        // that pressing Left increases x and Right decreases it.
        // tri_horz() will return -1, 0, or 1.
        x += keys.tri_horz();
        y += keys.tri_vert();


        // This keeps everything positive.
        // Note that just like as in C, -1 % 5 = -1, so we need to add width.
        x = (x + width) % width;
        y = (y + height) % height;

        color_ix = ((color_ix as i32 + keys.tri_shoulder_hit() + colors.len() as i32)
            % colors.len() as i32) as usize;


        m.dot(x, y, colors[color_ix]);
        m.dot((x+1) % width, y,
              colors[(color_ix + 1) % colors.len()]);
        m.dot((x+1) % width, (y+1) % height,
              colors[(color_ix + 2) % colors.len()]);
    }
}

