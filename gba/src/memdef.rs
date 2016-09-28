#![allow(non_upper_case_globals, non_snake_case)]
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


// REG_BGxCNT
//
pub const BG_MOSAIC: u32 = 0x0040;  //< Enable Mosaic
pub const BG_4BPP: u32 = 0;         //< 4bpp (16 color) bg (no effect on affine bg)
pub const BG_8BPP: u32 = 0x0080;    //< 8bpp (256 color) bg (no effect on affine bg)
pub const BG_WRAP: u32 = 0x2000;    //< Wrap around edges of affine bgs
pub const BG_SIZE0: u32 = 0;
pub const BG_SIZE1: u32 = 0x4000;
pub const BG_SIZE2: u32 = 0x8000;
pub const BG_SIZE3: u32 = 0xC000;
pub const BG_REG_32x32: u32 = 0;    //< reg bg, 32x32 (256x256 px)
pub const BG_REG_64x32: u32 = 0x4000;    //< reg bg, 64x32 (512x256 px)
pub const BG_REG_32x64: u32 = 0x8000;    //< reg bg, 32x64 (256x512 px)
pub const BG_REG_64x64: u32 = 0xC000;    //< reg bg, 64x64 (512x512 px)
pub const BG_AFF_16x16: u32 = 0;    //< affine bg, 16x16 (128x128 px)
pub const BG_AFF_32x32: u32 = 0x4000;    //< affine bg, 32x32 (256x256 px)
pub const BG_AFF_64x64: u32 = 0x8000;    //< affine bg, 64x64 (512x512 px)
pub const BG_AFF_128x128: u32 = 0xC000;  //< affine bg, 128x128 (1024x1024 px)

pub const BG_PRIO_MASK: u32 = 0x0003;
pub const BG_PRIO_SHIFT: u32 = 0;
#[inline]
pub fn BG_PRIO(n:u32) -> u32 {
    n << BG_PRIO_SHIFT
}

pub const BG_CBB_MASK: u32 = 0x000C;
pub const BG_CBB_SHIFT: u32 = 2;
#[inline]
pub fn BG_CBB(n:u32) -> u32 {
    n << BG_CBB_SHIFT
}

pub const BG_SBB_MASK: u32 = 0x1F00;
pub const BG_SBB_SHIFT: u32 = 8;
#[inline]
pub fn BG_SBB(n:u32) -> u32 {
    n << BG_SBB_SHIFT
}

pub const BG_SIZE_MASK: u32 = 0xC000;
pub const BG_SIZE_SHIFT: u32 = 14;
#[inline]
pub fn BG_SIZE(n:u32) -> u32 {
    n << BG_SIZE_SHIFT
}


/*
#define BG_BUILD(cbb, sbb, size, bpp, prio, mos, wrap)		\
(															\
	   ((size)<<14)  | (((wrap)&1)<<13) | (((sbb)&31)<<8	\
	| (((bpp)&8)<<4) | (((mos)&1)<<6)   | (((cbb)&3)<<2)	\
	| ((prio)&3)											\
)
*/
