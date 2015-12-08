
use core::slice;
use ::memmap;
use ::memdef;
use core::intrinsics::{volatile_store, volatile_load};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Color (pub u16);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct PaletteIx (pub u8);

impl Color {
    /// Creates a color with 16 bits, 5 bits for each channel.
    pub const fn rgb15(red: u32, green: u32, blue: u32) -> Color {
        Color((red | (green << 5) | (blue << 10)) as u16)
    }

    pub const BLACK:  Color = Color(0x0000);
    pub const RED:    Color = Color(0x001F);
    pub const LIME:   Color = Color(0x03E0);
    pub const YELLOW: Color = Color(0x03FF);
    pub const BLUE:   Color = Color(0x7C00);
    pub const MAG:    Color = Color(0x7C1F);
    pub const CYAN:   Color = Color(0x7FE0);
    pub const WHITE:  Color = Color(0x7FFF);
}



// --- sizes ---
pub const SCREEN_WIDTH: u32 = 240;
pub const SCREEN_HEIGHT: u32 = 160;

pub const M3_WIDTH: u32 = SCREEN_WIDTH;
pub const M3_HEIGHT: u32 = SCREEN_HEIGHT;
pub const M4_WIDTH: u32 = SCREEN_WIDTH;
pub const M4_HEIGHT: u32 = SCREEN_HEIGHT;
pub const M5_WIDTH: u32 = 160;
pub const M5_HEIGHT: u32 = 128;

pub fn vid_vsync(){
    unsafe{
    while volatile_load(memmap::REG_VCOUNT) >= 160 {}
    while volatile_load(memmap::REG_VCOUNT) <  160 {}
    }
}
// fn vid_flip() -> *mut u16 {

	// vid_page= (COLOR*)((u32)vid_page ^ VRAM_PAGE_SIZE);
	// REG_DISPCNT ^= DCNT_PAGE;	// update control register	

	// return vid_page;
// }




/// Check out http://www.coranac.com/tonc/text/bitmaps.htm for more details on
/// different modes.
///
/// Mode3 is a one buffer, with width 240, height 160, and 16 bits per pixel (bpp).
/// Is it the most basic of modes.
pub struct Mode3;
pub struct Mode4;
pub struct Mode5;

impl Mode3 {
    /// Calling this invalidates all other modes and enters Mode3.
    pub fn new () -> Mode3 {
        unsafe {
            volatile_store(
                memmap::REG_DISPCNT,
                memdef::DCNT_MODE3 | memdef::DCNT_BG2);
        }
        Mode3
    }

    /// Draw a dot at co-ordinates (x, y), and of color `color`.
    pub fn dot(&mut self, x: u32, y: u32, color: Color) {

        assert!(x < 240);
        assert!(y < 160);

        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(0x06000000 as *mut u16, 240 * 160)
        };
        buff[(x+y*240) as usize] = color.0;
    }
}

impl Mode4 {
    pub fn new() -> Mode4 {
        unsafe {
            volatile_store(
                memmap::REG_DISPCNT,
                memdef::DCNT_MODE4 | memdef::DCNT_BG2);
        }
        Mode4
    }
    pub fn horz_line(&mut self, l: u32, r: u32, y: u32, color: PaletteIx) {
        assert!(l < 240);
        assert!(r < 240);
        assert!(y < 160);
        assert!(l <= r);

        let double_color : u16 = (color.0 as u16) << 8 | (color.0 as u16);
        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(
                memmap::MEM_VRAM as *mut u16, 240 * 160 / 2)
        };
        // TODO off by one errors, think about this when not tired.
        for i in (l / 2)..(r / 2) {
            buff[(i + y * 120) as usize] = double_color;
        }
        // Might have missed the ends
        if l & 1 == 1 {
            self.dot(l, y, color);
        }
        if r & 1 == 1 {
            self.dot(r, y, color);
        }

    }

    pub fn dot(&mut self, x: u32, y: u32, color: PaletteIx) {
        assert!(x < 240);
        assert!(y < 160);

        // u16 *dst= &vid_page[(y*M4_WIDTH+x)/2];  // Division by 2 due to u8/u16 pointer mismatch!
        // if(x&1)
        //     *dst= (*dst& 0xFF) | (clrid<<8);    // odd pixel
        // else
        //     *dst= (*dst&~0xFF) |  clrid;        // even pixel
        //
        // TODO: /2 is correct?

        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(
                memmap::MEM_VRAM as *mut u16, 240 * 160 / 2)
        };
        let c : u16 = color.0 as u16;
        let old : u16 = buff[(x + y * 240) as usize / 2];
        if x & 1 != 0 {
            buff[(x +y*240) as usize / 2] = (old & 0xFF) | (c << 8);
        } else {
            buff[(x +y*240) as usize / 2] = (old & !0xFF) | c;
        }

    }
}

impl Mode5 {
    pub fn new() -> Mode5 {
        unsafe {
            volatile_store(
                memmap::REG_DISPCNT,
                memdef::DCNT_MODE5 | memdef::DCNT_BG2);
        }
        Mode5
    }

    /// Draw a dot at co-ordinates (x, y), and of color `color`.
    pub fn dot(&mut self, x: u32, y: u32, color: Color) {

        assert!(x < 160);
        assert!(y < 126);

        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(0x06000000 as *mut u16, 160 * 128)
        };
        buff[(x+y*160) as usize] = color.0;
    }
}

