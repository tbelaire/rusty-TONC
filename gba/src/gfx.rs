use core::slice;
use memmap;
use memdef;
use core::ptr::{read_volatile, write_volatile};

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
pub const SCREEN_WIDTH: i32 = 240;
pub const SCREEN_HEIGHT: i32 = 160;

pub fn vid_vsync(){
    unsafe{
    while read_volatile(memmap::REG_VCOUNT) >= 160 {}
    while read_volatile(memmap::REG_VCOUNT) <  160 {}
    }
}

/// Check out http://www.coranac.com/tonc/text/bitmaps.htm for more details on
/// different bitmap modes.
///
/// Mode3 is a one buffer, with width 240, height 160, and 16 bits per pixel (bpp).
/// Is it the most basic of modes.
pub struct Mode3;
pub struct Mode4  {
    // This is u16 since you can't write in single bytes anyways.
    current_page: &'static mut [u16],
}
pub struct Mode5;

impl Mode3 {
    pub const WIDTH : usize = 240;
    pub const HEIGHT : usize = 160;
    pub const BIT_DEPTH : usize = 16;
    pub const SIZE : usize = 0x12C00; // WIDTH * HEIGHT * BIT_DEPTH/8

    // Doesn't actually compile yet.  But I like the idea, if not the name.
    // pub type Entry = Color;

    /// Calling this invalidates all other modes and enters Mode3.
    pub fn new () -> Mode3 {
        unsafe {
            write_volatile(
                memmap::REG_DISPCNT,
                memdef::DCNT_MODE3 | memdef::DCNT_BG2);
        }
        Mode3
    }

    /// Draw a dot at co-ordinates (x, y), and of color `color`.
    pub fn dot(&mut self, x: i32, y: i32, color: Color) {
        assert!(0 <= x && x < Self::WIDTH as i32);
        assert!(0 <= y && y < Self::HEIGHT as i32);

        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(0x06000000 as *mut u16, 240 * 160)
        };
        buff[(x+y*Self::WIDTH as i32) as usize] = color.0;
    }
}

impl Mode4 {
    /// The width of the screen (in px).
    pub const WIDTH : usize = 240;
    /// The height of the screen (in px).
    pub const HEIGHT : usize = 160;
    /// The number of bits per pixel.  Mode4 is indexed.
    pub const BIT_DEPTH : usize = 8;
    /// The number of bytes in a full screen.
    pub const SIZE : usize = 0x9600; // WIDTH * HEIGHT * BIT_DEPTH/8

    /// Initializes the screen to mode4, and enables BG_2.
    /// It starts out pointing to the first page in VRAM,
    /// but you can call page_flip() to swap the currently
    /// showing page as well as the target surface for drawing
    /// operations.  By default, it will draw to the currently
    /// showing page.
    /// You can toggle that by calling draw_page_flip().
    pub fn new() -> Mode4 {
        unsafe {
            write_volatile(
                memmap::REG_DISPCNT,
                memdef::DCNT_MODE4 | memdef::DCNT_BG2);
        }
        let current_page : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(
                memmap::MEM_VRAM as *mut u16,
                Mode4::WIDTH * Mode4::HEIGHT / 2) // u8 to u16
        };
        Mode4 {current_page: current_page}
    }

    pub fn horz_line(&mut self, l: i32, r: i32, y: i32, color: PaletteIx) {
        assert!(0 <= l && l < 240);
        assert!(0 <= r && r < 240);
        assert!(0 <= r && y < 160);
        assert!(l <= r);

        let double_color : u16 = (color.0 as u16) << 8 | (color.0 as u16);
        // TODO off by one errors, think about this when not tired.
        for i in (l / 2)..(r / 2) {
            self.current_page[(i + y * 120) as usize] = double_color;
        }
        // Might have missed the ends
        if l & 1 == 1 {
            self.dot(l, y, color);
        }
        if r & 1 == 1 {
            self.dot(r, y, color);
        }

    }

    pub fn dot(&mut self, x: i32, y: i32, color: PaletteIx) {
        assert!(0 <= x && x < 240);
        assert!(0 <= y && y < 160);

        // u16 *dst= &vid_page[(y*M4_WIDTH+x)/2];  // Division by 2 due to u8/u16 pointer mismatch!
        // if(x&1)
        //     *dst= (*dst& 0xFF) | (clrid<<8);    // odd pixel
        // else
        //     *dst= (*dst&~0xFF) |  clrid;        // even pixel
        //
        // TODO: /2 is correct?

        let c : u16 = color.0 as u16;
        let dest : &mut u16 = &mut self.current_page[(x + y * 240) as usize / 2];
        if x & 1 != 0 {
            *dest = (*dest & 0xFF) | (c << 8);
        } else {
            *dest = (*dest & !0xFF) | c;
        }

    }

    /// Changes the surface drawn to without changing the currently displayed
    /// page.  Only really needed to be called once.
    pub fn draw_page_flip(&mut self) {
        unsafe {
            let current_page = self.current_page.as_mut_ptr();
            let new_page = ((current_page as u32) ^ memmap::VRAM_PAGE_SIZE) as *mut u16;
            self.current_page = slice::from_raw_parts_mut(
                new_page,
                Mode4::WIDTH * Mode4::HEIGHT / 2); // u8 to u16
        }
    }

    // Also known as vid_flip() for those grepping.
    pub fn page_flip(&mut self) {
        unsafe {
            self.draw_page_flip();
            write_volatile(memmap::REG_DISPCNT,
                           read_volatile(memmap::REG_DISPCNT)
                           ^ memdef::DCNT_PAGE);
        }
    }
}

impl Mode5 {
    /// The width of the screen (in px).
    pub const WIDTH : usize = 160;
    /// The height of the screen (in px).
    pub const HEIGHT : usize = 128;
    /// The number of bits per pixel.  Mode4 is indexed.
    pub const BIT_DEPTH : usize = 16;
    /// The number of bytes in a full screen.
    pub const SIZE : usize = 0xA000; // WIDTH * HEIGHT * BIT_DEPTH/8
    pub fn new() -> Mode5 {
        unsafe {
            write_volatile(
                memmap::REG_DISPCNT,
                memdef::DCNT_MODE5 | memdef::DCNT_BG2);
        }
        Mode5
    }

    /// Draw a dot at co-ordinates (x, y), and of color `color`.
    pub fn dot(&mut self, x: i32, y: i32, color: Color) {

        assert!(0 <= x && x < Self::WIDTH as i32);
        assert!(0 <= y && y < Self::HEIGHT as i32);

        let buff : &mut [u16] = unsafe {
            slice::from_raw_parts_mut(0x06000000 as *mut u16, 160 * 128)
        };
        buff[(x+y*Self::WIDTH as i32) as usize] = color.0;
    }
}

