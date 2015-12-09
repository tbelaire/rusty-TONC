
pub const DCNT_MODE0:  u32 = 0;	///< Mode 0; bg 0-4: reg
pub const DCNT_MODE1:  u32 = 0x0001;	///< Mode 1; bg 0-1: reg; bg 2: affine
pub const DCNT_MODE2:  u32 = 0x0002;	///< Mode 2; bg 2-2: affine
pub const DCNT_MODE3:  u32 = 0x0003;	///< Mode 3; bg2: 240x160\@16 bitmap
pub const DCNT_MODE4:  u32 = 0x0004;	///< Mode 4; bg2: 240x160\@8 bitmap
pub const DCNT_MODE5:  u32 = 0x0005;	///< Mode 5; bg2: 160x128\@16 bitmap
pub const DCNT_GB:  u32 = 0x0008;	///< (R) GBC indicator
pub const DCNT_PAGE:  u32 = 0x0010;	///< Page indicator
pub const DCNT_OAM_HBL:  u32 = 0x0020;	///< Allow OAM updates in HBlank
pub const DCNT_OBJ_2D:  u32 = 0;	///< OBJ-VRAM as matrix
pub const DCNT_OBJ_1D:  u32 = 0x0040;	///< OBJ-VRAM as array
pub const DCNT_BLANK:  u32 = 0x0080;	///< Force screen blank
pub const DCNT_BG0:  u32 = 0x0100;	///< Enable bg 0
pub const DCNT_BG1:  u32 = 0x0200;	///< Enable bg 1
pub const DCNT_BG2:  u32 = 0x0400;	///< Enable bg 2
pub const DCNT_BG3:  u32 = 0x0800;	///< Enable bg 3
pub const DCNT_OBJ:  u32 = 0x1000;	///< Enable objects
pub const DCNT_WIN0:  u32 = 0x2000;	///< Enable window 0
pub const DCNT_WIN1:  u32 = 0x4000;	///< Enable window 1
pub const DCNT_WINOBJ:  u32 = 0x8000;	///< Enable object window

pub const DCNT_MODE_MASK:  u32 = 0x0007;
pub const DCNT_MODE_SHIFT:  u32 = 0;

// This is just ported directly from TONC,
// I'll get around to fixing it up sometime,
// till then just leave it matching.
#[allow(non_snake_case)]
pub fn DCNT_MODE(n : u32) -> u32 {
    n << DCNT_MODE_SHIFT
}


pub const DCNT_LAYER_MASK:  u32 = 0x1F00;
pub const DCNT_LAYER_SHIFT:  u32 = 8;

#[allow(non_snake_case)]
pub fn DCNT_LAYER(n : u32) -> u32 {
    n << DCNT_LAYER_SHIFT
}

pub const DCNT_WIN_MASK:  u32 = 0xE000;
pub const DCNT_WIN_SHIFT:  u32 = 13;

#[allow(non_snake_case)]
pub fn DCNT_WIN(n: u32) -> u32 {
    n << DCNT_WIN_SHIFT
}

#[allow(non_snake_case)]
pub fn DCNT_BUILD(mode: u32, layer: u32, win: u32, obj1d: u32, objhbl: u32) -> u32 {
    (((win)&7)<<13) | (((layer)&31)<<8) | (((obj1d)&1)<<6)
        | (((objhbl)&1)<<5) | ((mode)&7)
}

