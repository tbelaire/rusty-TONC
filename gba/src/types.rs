
// See gfx for color



/// This has 4bpp 16-color palette indices.
pub struct Tile4 {
    pub data: [u32; 8]
}

/// This has 8bpp 256-color palette indices.
pub struct Tile8 {
    pub data: [u32; 16]
}

pub type ScrEntry = u16;

pub type Screenblock = [ScrEntry; 1024];

pub type Charblock = [Tile4; 512];
pub type Charblock8 = [Tile8; 256];

