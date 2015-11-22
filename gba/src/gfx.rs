use core::slice;

pub struct Color (u16);

impl Color {
    /// Creates a color with 16 bits, 5 bits for each channel.
    pub fn rgb15(red: u32, green: u32, blue: u32) -> Color {
        Color((red | (green << 5) | (blue << 10)) as u16)
    }
}


/// Check out http://www.coranac.com/tonc/text/bitmaps.htm for more details on
/// different modes.
///
/// Mode3 is a one buffer, with width 240, height 160, and 16 bits per pixel (bpp).
/// Is it the most basic of modes.
pub struct Mode3;

impl Mode3 {
    /// Calling this invalidates all other modes and enters Mode3.
    pub fn new () -> Mode3 {
        unsafe {
            *(0x04000000 as *mut u32)= 0x0403;
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

