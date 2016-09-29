
/// VRAM-safe cpy.
/// This version mimics memcpy in functionality, with 
/// the benefit of working for VRAM as well. It is also 
/// slightly faster than the original memcpy, but faster 
/// implementations can be made.
/// \param dst	Destination pointer.
/// \param src	Source pointer.
/// \param size	Fill-length in bytes.
/// \return		\a dst.
/// \note	The pointers and size need not be word-aligned.
// void *tonccpy(void *dst, const void *src, uint size)
extern {
    #[no_mangle]
    pub fn tonccpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8;
    #[no_mangle]
    pub fn toncclr(dst: *mut u8, size: usize) -> *mut u8;
}
#[no_mangle]
pub fn __aeabi_memcpy8(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    unsafe {
        tonccpy(dst, src, size)
    }
}
#[no_mangle]
pub fn __aeabi_memcpy4(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    unsafe {
        tonccpy(dst, src, size)
    }
}
#[no_mangle]
pub fn __aeabi_memcpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    unsafe {
        tonccpy(dst, src, size)
    }
}

#[no_mangle]
pub fn __aeabi_memclr4(dst: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        toncclr(dst, size)
    }
}

#[no_mangle]
pub fn __aeabi_memclr8(dst: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        toncclr(dst, size)
    }
}

#[no_mangle]
pub fn __aeabi_memclr(dst: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        toncclr(dst, size)
    }
}
