#![allow(non_upper_case_globals)]
use gfx::Color;
use types::*;
use oam::obj_attr;

pub const MEM_IO:   u32 = 0x04000000;
pub const MEM_PAL:  u32 = 0x05000000;  // no 8bit write !!
pub const MEM_VRAM: u32 = 0x06000000;  // no 8bit write !!
pub const MEM_OAM:  u32 = 0x07000000;  // no 8bit write !!

pub const PAL_SIZE:  u32 = 0x00400;
pub const VRAM_SIZE: u32 = 0x18000;
pub const OAM_SIZE:  u32 = 0x00400;

pub const PAL_BG_SIZE:     u32 = 0x00200;
pub const PAL_OBJ_SIZE:    u32 = 0x00200;
pub const VRAM_BG_SIZE:    u32 = 0x10000;
pub const VRAM_OBJ_SIZE:   u32 = 0x08000;
pub const M3_SIZE:         u32 = 0x12C00;
pub const M4_SIZE:         u32 = 0x09600;
pub const M5_SIZE:         u32 = 0x0A000;
pub const VRAM_PAGE_SIZE : u32 = 0x0A000;


// Nice names for the most important banks...
// Copied from TONC directly, maybe I'll revisit later.
pub const pal_bg_mem:  *mut Color       = MEM_PAL as *mut Color;
pub const pal_obj_mem: *mut Color       = (MEM_PAL + PAL_BG_SIZE) as *mut Color;
pub const vid_mem:     *mut Color       = MEM_VRAM as *mut Color;
pub const tile_mem:    *mut Charblock   = MEM_VRAM as *mut Charblock;

pub const se_mem:      *mut Screenblock = MEM_VRAM as *mut Screenblock;
pub const oam_mem:     *mut obj_attr    = MEM_OAM as *mut obj_attr;



pub const REG_BASE: u32 = MEM_IO;

// Must use volatile!
pub const REG_DISPCNT: *mut u32 = (REG_BASE + 0x0000) as *mut u32;	// display control
pub const REG_DISPSTAT: *mut u16 = (REG_BASE + 0x0004) as *mut u16;	// display interupt status
pub const REG_VCOUNT: *mut u16 = (REG_BASE + 0x0006) as *mut u16;	// vertical count

pub const REG_KEYINPUT: *mut u16 = (REG_BASE + 0x0130) as *mut u16;	// Key status

// Background
pub const REG_BG0CNT: *mut u16 = (REG_BASE + 0x0008) as *mut u16; // bg 0-3 control
pub const REG_BG1CNT: *mut u16 = (REG_BASE + 0x000A) as *mut u16;
pub const REG_BG2CNT: *mut u16 = (REG_BASE + 0x000C) as *mut u16;
pub const REG_BG3CNT: *mut u16 = (REG_BASE + 0x000E) as *mut u16;

pub const REG_BG0HOFS: *mut u16 = (REG_BASE+0x0010) as *mut u16;  // bg 0-3 origins
pub const REG_BG0VOFS: *mut u16 = (REG_BASE+0x0012) as *mut u16;
pub const REG_BG1HOFS: *mut u16 = (REG_BASE+0x0014) as *mut u16;
pub const REG_BG1VOFS: *mut u16 = (REG_BASE+0x0016) as *mut u16;
pub const REG_BG2HOFS: *mut u16 = (REG_BASE+0x0018) as *mut u16;
pub const REG_BG2VOFS: *mut u16 = (REG_BASE+0x001A) as *mut u16;
pub const REG_BG3HOFS: *mut u16 = (REG_BASE+0x001C) as *mut u16;
pub const REG_BG3VOFS: *mut u16 = (REG_BASE+0x001E) as *mut u16;
