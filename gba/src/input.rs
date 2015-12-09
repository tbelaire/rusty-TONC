use ::memmap;
use core::intrinsics::{volatile_load};

/// Keys also functions as the flags for the keys.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum Keys {
    A =      0x0001,
    B =      0x0002,
    Select = 0x0004,
    Start  = 0x0008,
    Right  = 0x0010,
    Left   = 0x0020,
    Up     = 0x0040,
    Down   = 0x0080,
    R      = 0x0100,
    L      = 0x0200,
}

/// The OR of all the keys.
pub const KEY_MASK: u32 = 0x03FF;


/// KeysIndex is what index the key is at, not what flag you need
/// to mask to get it.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum KeyIndex {
    A = 0x0000,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
    R,
    L,
}

#[derive(Debug)]
pub struct Input {
    prev: u32,
    curr: u32,
}
fn bit_tribool(bits: u32, negative : KeyIndex, positive : KeyIndex) -> i32{
    ((bits >> positive as u32) & 1) as i32
        - ((bits >> negative as u32) & 1) as i32
}

impl Input {
    /// You should only need one copy of this struct.
    pub fn new() -> Input {
        Input{ prev: 0, curr: 0}
    }

    /// This should be called once a frame.
    pub fn poll(&mut self) {
        self.prev = self.curr;
        self.curr = unsafe {!(volatile_load(memmap::REG_KEYINPUT) as u32)}
                    & KEY_MASK;
    }

    /// hit checks if the key is now pressed, but wasn't before.
    pub fn hit(&mut self, k: Keys) -> bool {
        (!self.prev & self.curr) & (k as u32) != 0
    }
    /// held checks that it's been pressed for 2 consecutive frames.
    pub fn held(&mut self, k: Keys) -> bool {
        (self.prev & self.curr) & (k as u32) != 0
    }
    /// released checks if it was held, but now is not.
    pub fn released(&mut self, k: Keys) -> bool {
        (self.prev & !self.curr) & (k as u32) != 0
    }

    /// is_down tests if the key is currently down.
    pub fn is_down(&mut self, k : Keys) -> bool {
        self.curr & (k as u32) != 0
    }
    /// is_up tests if the key is currently up.
    pub fn is_up(&mut self, k : Keys) -> bool {
        self.curr & (k as u32) == 0
    }

    /// These family of functions return -1, 0, or 1.
    /// tri_horz is 1 when Left is pressed, and -1 when Right is.
    pub fn tri_horz(&mut self) -> i32 {
        bit_tribool(self.curr, KeyIndex::Left, KeyIndex::Right)
    }
    /// tri_vert is 1 when Up is pressed, and -1 when Down is.
    pub fn tri_vert(&mut self) -> i32 {
        bit_tribool(self.curr, KeyIndex::Up, KeyIndex::Down)
    }
    /// tri_shoulder is 1 when L is pressed, and -1 when R is.
    pub fn tri_shoulder(&mut self) -> i32 {
        bit_tribool(self.curr, KeyIndex::L, KeyIndex::R)
    }
    /// tri_fire is 1 when A is pressed and -1 when B is.  It's a bit silly.
    pub fn tri_fire(&mut self) -> i32 {
        bit_tribool(self.curr, KeyIndex::A, KeyIndex::B)
    }

    /// This family of functions work just like the above, but only
    /// on the first frame the key is pressed, returning to 0 afterwards.
    pub fn tri_horz_hit(&mut self) -> i32 {
        bit_tribool(self.curr & !self.prev, KeyIndex::Left, KeyIndex::Right)
    }
    pub fn tri_vert_hit(&mut self) -> i32 {
        bit_tribool(self.curr & !self.prev, KeyIndex::Up, KeyIndex::Down)
    }
    pub fn tri_shoulder_hit(&mut self) -> i32 {
        bit_tribool(self.curr & !self.prev, KeyIndex::L, KeyIndex::R)
    }
    pub fn tri_fire_hit(&mut self) -> i32 {
        bit_tribool(self.curr & !self.prev, KeyIndex::A, KeyIndex::B)
    }
}

