use ::memmap;
use ::memdef;
use ::volatile_load;

#[repr(C)]
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
    pub fn down(&mut self, k : Keys) -> bool {
         self.curr & (k as u32) != 0
    }
    pub fn up(&mut self, k : Keys) -> bool {
        (self.prev & !self.curr) & (k as u32) != 0
    }
}

