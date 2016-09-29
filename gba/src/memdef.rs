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


// --- OAM attribute 0 ---------------------------------------------

pub const ATTR0_REG:         u16 = 0;      // Regular object
pub const ATTR0_AFF:         u16 = 0x0100; // Affine object
pub const ATTR0_HIDE:        u16 = 0x0200; // Inactive object
pub const ATTR0_AFF_DBL:     u16 = 0x0300; // Double-size affine object
pub const ATTR0_AFF_DBL_BIT: u16 = 0x0200;
pub const ATTR0_BLEND:       u16 = 0x0400; // Enable blend
pub const ATTR0_WINDOW:      u16 = 0x0800; // Use for object window
pub const ATTR0_MOSAIC:      u16 = 0x1000; // Enable mosaic
pub const ATTR0_4BPP:        u16 = 0;      // Use 4bpp (16 color) tiles
pub const ATTR0_8BPP:        u16 = 0x2000; // Use 8bpp (256 color) tiles
pub const ATTR0_SQUARE:      u16 = 0;      // Sq     uare shape
pub const ATTR0_WIDE:        u16 = 0x4000; // Tall shape (height &gt; width)
pub const ATTR0_TALL:        u16 = 0x8000; // Wide shape (height &lt; width)

pub const ATTR0_Y_MASK:      u16 = 0x00FF;
pub const ATTR0_Y_SHIFT:     u16 = 0;

#[inline]
pub fn ATTR0_Y(n:u16) -> u16 {
    n << ATTR0_Y_SHIFT
}

pub const ATTR0_MODE_MASK:   u16 = 0x0300;
pub const ATTR0_MODE_SHIFT:  u16 = 8;
// #define ATTR0_MODE(n)        ((n)<<ATTR0_MODE_SHIFT)
#[inline]
pub fn ATTR0_MODE(n:u16) -> u16 {
    n << ATTR0_MODE_SHIFT
}

pub const ATTR0_SHAPE_MASK:  u16 = 0xC000;
pub const ATTR0_SHAPE_SHIFT: u16 = 14;
// #define ATTR0_SHAPE(n)    ((n)<<ATTR0_SHAPE_SHIFT)
#[inline]
pub fn ATTR0_SHAPE(n:u16) -> u16 {
    n << ATTR0_SHAPE_SHIFT
}

// --- OAM attribute 1 ---------------------------------------------

pub const ATTR1_HFLIP:         u16 = 0x1000;    // Horizontal flip (reg obj only)
pub const ATTR1_VFLIP:         u16 = 0x2000;    // Vertical flip (reg obj only)
// Base sizes
pub const ATTR1_SIZE_8:        u16 = 0;
pub const ATTR1_SIZE_16:       u16 = 0x4000;
pub const ATTR1_SIZE_32:       u16 = 0x8000;
pub const ATTR1_SIZE_64:       u16 = 0xC000;
// Square sizes
pub const ATTR1_SIZE_8x8:      u16 = 0;    // Size flag for  8x8 px object
pub const ATTR1_SIZE_16x16:    u16 = 0x4000;    // Size flag for 16x16 px object
pub const ATTR1_SIZE_32x32:    u16 = 0x8000;    // Size flag for 32x32 px object
pub const ATTR1_SIZE_64x64:    u16 = 0xC000;    // Size flag for 64x64 px object
// Tall sizes
pub const ATTR1_SIZE_8x16:     u16 = 0;    // Size flag for  8x16 px object
pub const ATTR1_SIZE_8x32:     u16 = 0x4000;    // Size flag for  8x32 px object
pub const ATTR1_SIZE_16x32:    u16 = 0x8000;    // Size flag for 16x32 px object
pub const ATTR1_SIZE_32x64:    u16 = 0xC000;    // Size flag for 32x64 px object
// Wide sizes
pub const ATTR1_SIZE_16x8:     u16 = 0;    // Size flag for 16x8 px object
pub const ATTR1_SIZE_32x8:     u16 = 0x4000;    // Size flag for 32x8 px object
pub const ATTR1_SIZE_32x16:    u16 = 0x8000;    // Size flag for 32x16 px object
pub const ATTR1_SIZE_64x32:    u16 = 0xC000;    // Size flag for 64x64 px object


pub const ATTR1_X_MASK:        u16 = 0x01FF;
pub const ATTR1_X_SHIFT:       u16 = 0;
#[inline]
pub fn ATTR1_X(n:u16) -> u16 {
    n << ATTR1_X_SHIFT
}

pub const ATTR1_AFF_ID_MASK:   u16 = 0x3E00;
pub const ATTR1_AFF_ID_SHIFT:  u16 = 9;
#[inline]
pub fn ATTR1_AFF_ID(n:u16) -> u16 {
    n << ATTR1_AFF_ID_SHIFT
}

pub const ATTR1_FLIP_MASK:     u16 = 0x3000;
pub const ATTR1_FLIP_SHIFT:    u16 = 12;
#[inline]
pub fn ATTR1_FLIP(n:u16) -> u16 {
    n << ATTR1_FLIP_SHIFT
}

pub const ATTR1_SIZE_MASK:     u16 = 0xC000;
pub const ATTR1_SIZE_SHIFT:    u16 = 14;
#[inline]
pub fn ATTR1_SIZE(n:u16) -> u16 {
    n << ATTR1_SIZE_SHIFT
}


// #define ATTR1_BUILDR(x, size, hflip, vflip)    \
// ( ((x)&511) | (((hflip)&1)<<12) | (((vflip)&1)<<13) | (((size)&3)<<14) )

// #define ATTR1_BUILDA(x, size, affid)            \
// ( ((x)&511) | (((affid)&31)<<9) | (((size)&3)<<14) )


// --- OAM attribute 2 -------------------------------------------------

pub const ATTR2_ID_MASK:       u16 = 0x03FF;
pub const ATTR2_ID_SHIFT:      u16 = 0;
#[inline]
pub fn ATTR2_ID(n:u16) -> u16 {
    n << ATTR2_ID_SHIFT
}

pub const ATTR2_PRIO_MASK:     u16 = 0x0C00;
pub const ATTR2_PRIO_SHIFT:    u16 = 10;
#[inline]
pub fn ATTR2_PRIO(n:u16) -> u16 {
    n << ATTR2_PRIO_SHIFT
}

pub const ATTR2_PALBANK_MASK:  u16 = 0xF000;
pub const ATTR2_PALBANK_SHIFT: u16 = 12;
#[inline]
pub fn ATTR2_PALBANK(n:u16) -> u16 {
    n << ATTR2_PALBANK_SHIFT
}

// #define ATTR2_BUILD(id, pbank, prio)			\
// ( ((id)&0x3FF) | (((pbank)&15)<<12) | (((prio)&3)<<10) )

#[inline]
pub fn ATTR2_BUILD(tile_id: u16, pbank: u16, prio:u16) -> u16 {
    (tile_id & 0x3FF)
     | ((pbank & 15)<<12)
     | ((prio & 3)<<10)
}

