const VIDEO_RAM_SIZE: usize = 0x2000;
const PIXEL_COUNT: usize = 160 * 144;
const OAM_SIZE: usize = 0xA0;
const TILE_SIZE: usize = 64;
const TILE_SET_SIZE: usize = 384;

type Tile = [u8; TILE_SIZE];

pub struct GPU {
    pub ram: [u8; VIDEO_RAM_SIZE],
    pub tile_set: [Tile; TILE_SET_SIZE],
    pub canvas_buffer: [u8; PIXEL_COUNT],
    pub oam: [u8; OAM_SIZE],
}

impl GPU {
    pub fn new() -> Self {
        Self {
            ram: [0; VIDEO_RAM_SIZE],
            tile_set: [[0; TILE_SIZE]; TILE_SET_SIZE],
            canvas_buffer: [0; PIXEL_COUNT],
            oam: [0; OAM_SIZE],
        }
    }
}
