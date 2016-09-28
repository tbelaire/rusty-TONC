use gfx::Color;
use types::*;

pub const MEM_IO : u32 = 0x04000000;
pub const MEM_PAL : u32 = 0x05000000;		// no 8bit write !!
pub const MEM_VRAM : u32 = 0x06000000;		// no 8bit write !!

pub const PAL_SIZE : u32 = 0x00400;
pub const VRAM_SIZE : u32 = 0x18000;

pub const VRAM_PAGE_SIZE : u32 = 0x0A000;

// Nice names for the most important banks...
// Copied from TONC directly, maybe I'll revisit later.
#[allow(non_upper_case_globals)]
pub const pal_bg_mem: *mut Color =  MEM_PAL as *mut Color;
#[allow(non_upper_case_globals)]
pub const vid_mem: *mut Color = MEM_VRAM as *mut Color;
#[allow(non_upper_case_globals)]
pub const tile_mem: *mut Charblock = MEM_VRAM as *mut Charblock;

#[allow(non_upper_case_globals)]
pub const se_mem: *mut Screenblock = MEM_VRAM as *mut Screenblock;



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
