#![feature(no_std, intrinsics, const_fn, associated_consts)]

#![no_std]

pub mod gfx;
pub mod memmap;
pub mod memdef;
pub mod input;
pub mod tonc_stolen;

extern "rust-intrinsic" {
    pub fn volatile_load<T>(src: *const T) -> T;
    pub fn volatile_store<T>(dst: *mut T, val: T);
}

