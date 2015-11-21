use core::slice;

pub struct Mode3;

impl Mode3 {
    pub fn new () -> Mode3 {
        unsafe {
            *(0x04000000 as *mut u32)= 0x0403;
        }
        Mode3
    }

    pub fn dot(&mut self, x: u32, y: u32, color: Color) {

        assert!(x < 240);
        assert!(y < 160);

        unsafe{
            let b : &mut [u16] = slice::from_raw_parts_mut(0x06000000 as *mut u16, 240 * 160);
            b[(x+y*240) as usize] = color.0;
        }
    }
}

pub struct Color (u16);

impl Color {
    pub fn rgb15(red: u32, green: u32, blue: u32) -> Color {
        Color((red | (green << 5) | (blue << 10)) as u16)
    }
}


