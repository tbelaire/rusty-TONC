use ::memmap;
use ::memdef;
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

impl Input {
    pub fn new() -> Input {
        Input{ prev: 0, curr: 0}
    }

    pub fn poll(&mut self) {
        self.prev = self.curr;
        unsafe {
        self.curr = !(volatile_load(memmap::REG_KEYINPUT) as u32) & memdef::KEY_MASK;
        }
    }

    pub fn hit(&mut self, k: Keys) -> bool {
        (!self.prev & self.curr) & (k as u32) != 0
    }
    pub fn held(&mut self, k: Keys) -> bool {
        (self.prev & self.curr) & (k as u32) != 0
    }
    pub fn released(&mut self, k: Keys) -> bool {
        (self.prev & !self.curr) & (k as u32) != 0
    }

    pub fn is_down(&mut self, k : Keys) -> bool {
        self.curr & (k as u32) != 0
    }
    pub fn is_up(&mut self, k : Keys) -> bool {
        self.curr & (k as u32) == 0
    }

    fn bit_tribool(&mut self, negative : KeyIndex, positive : KeyIndex) -> i32{
        ((self.curr >> positive as u32) & 1) as i32
            - ((self.curr >> negative as u32) & 1) as i32
    }
    pub fn tri_horz(&mut self) -> i32 {
        self.bit_tribool(KeyIndex::Left, KeyIndex::Right)
    }
    pub fn tri_vert(&mut self) -> i32 {
        self.bit_tribool(KeyIndex::Up, KeyIndex::Down)
    }
    pub fn tri_shoulder(&mut self) -> i32 {
        self.bit_tribool(KeyIndex::L, KeyIndex::R)
    }
    pub fn tri_fire(&mut self) -> i32 {
        self.bit_tribool(KeyIndex::A, KeyIndex::B)
    }


}

