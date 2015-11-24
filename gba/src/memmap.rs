use gfx::Color;

pub const MEM_IO : u32 = 0x04000000;
pub const MEM_PAL : u32 = 0x05000000;		// no 8bit write !!
pub const MEM_VRAM : u32 = 0x06000000;		// no 8bit write !!

pub const PAL_SIZE : u32 = 0x00400;
pub const VRAM_SIZE : u32 = 0x18000;

pub const M3_SIZE : u32 = 0x12C00;
pub const M4_SIZE : u32 = 0x09600;
pub const M5_SIZE : u32 = 0x0A000;
pub const VRAM_PAGE_SIZE : u32 = 0x0A000;


pub const MEM_VRAM_BACK : u32 = (MEM_VRAM + VRAM_PAGE_SIZE);

// pal_bg_mem[y] = Color (color y)
pub const pal_bg_mem: *mut Color =  MEM_PAL as *mut Color;

// vid_mem[a] = Color
pub const vid_mem: *mut Color = MEM_VRAM as *mut Color;

pub const vid_mem_front: *mut Color = MEM_VRAM as *mut Color;
pub const vid_mem_back: *mut Color = MEM_VRAM_BACK as *mut Color;


pub const REG_BASE: u32 = MEM_IO;

// Must use volatile!
pub const REG_DISPCNT: *mut u32 = (REG_BASE + 0x0000) as *mut u32;	// display control
pub const REG_DISPSTAT: *mut u16 = (REG_BASE + 0x0004) as *mut u16;	// display interupt status
pub const REG_VCOUNT: *mut u16 = (REG_BASE + 0x0006) as *mut u16;	// vertical count

pub const REG_KEYINPUT: *mut u16 = (REG_BASE + 0x0130) as *mut u16;	// Key status
