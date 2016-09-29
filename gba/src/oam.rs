use ::memdef::*;

pub fn oam_init(obj_buffer: &mut [obj_attr]) {
    for ref mut obj in obj_buffer.iter_mut() {
        obj.set_attr(ATTR0_HIDE, 0, 0);
    }
}

pub fn oam_copy(obj_target: &mut [obj_attr], obj_buffer: &[obj_attr]) {
    obj_target[0 .. obj_buffer.len()].copy_from_slice(obj_buffer)
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct obj_attr {
    pub attr0: u16,
    pub attr1: u16,
    pub attr2: u16,
    pub fill: i16,
}

impl obj_attr {
    #[inline]
    pub fn set_attr(&mut self, attr0: u16, attr1: u16, attr2: u16) {
        self.attr0 = attr0;
        self.attr1 = attr1;
        self.attr2 = attr2;
    }

    #[inline]
    pub fn set_pos_u16(&mut self, x: u16, y: u16) {
        self.attr0 = (self.attr0 & !ATTR0_Y_MASK) | (y & ATTR0_Y_MASK);
        self.attr1 = (self.attr1 & !ATTR1_X_MASK) | (x & ATTR1_X_MASK);
    }
    #[inline]
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.set_pos_u16(x as u16, y as u16)
    }
    #[inline]
    pub fn set_attr2(&mut self, tid: i32, pb: i32, prio: i32) {
        self.attr2 = ATTR2_BUILD(tid as u16, pb as u16, prio as u16)
    }
}
