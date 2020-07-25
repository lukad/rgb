use crate::gpu::GPU;

pub struct MemoryBus {
    rom: [u8; 0x8000],
    eram: [u8; 0x2000],
    wram: [u8; 0x2000],
    zram: [u8; 0xFF],
    gpu: GPU,
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x8000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            zram: [0; 0xFF],
            gpu: GPU::new(),
        }
    }
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        let addr = address as usize;
        match addr & 0xF000 {
            0x0000 => {
                // TODO: check for bios
                self.rom[addr]
            }
            // ROM0
            0x1000 | 0x2000 | 0x3000 => self.rom[addr],
            // ROM1
            0x4000 | 0x5000 | 0x6000 | 0x7000 => self.rom[addr],
            // VRAM
            0x8000 | 0x9000 => self.gpu.ram[addr & 0x1FFF],
            // External RAM
            0xA000 | 0xB000 => self.eram[addr & 0x1FFF],
            // Working RAM
            0xC000 | 0xD000 => self.wram[addr & 0x1FFF],
            // Working RAM shadow
            0xE000 => self.wram[addr & 0x1FFF],
            0xF000 => match addr & 0x0F00 {
                // Working RAM shadow
                a if a < 0xD00 => self.wram[addr & 0x1FFF],
                // Sprite attributes - only 160 bytes long
                0xE00 if addr < 0xFEA0 => self.gpu.oam[addr & 0xFF],
                0xE00 => 0,
                // Zero page
                0xF00 if addr >= 0xFF80 => self.zram[addr & 0x7F],
                // Unhandled
                0xF00 => 0,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_mut_byte(&mut self, address: u16) -> &mut u8 {
        let addr = address as usize;
        match addr & 0xF000 {
            0x0000 => {
                // TODO: check for bios
                &mut self.rom[addr]
            }
            // ROM0
            0x1000 | 0x2000 | 0x3000 => &mut self.rom[addr],
            // ROM1
            0x4000 | 0x5000 | 0x6000 | 0x7000 => &mut self.rom[addr],
            // VRAM
            0x8000 | 0x9000 => &mut self.gpu.ram[addr & 0x1FFF],
            // External RAM
            0xA000 | 0xB000 => &mut self.eram[addr & 0x1FFF],
            // Working RAM
            0xC000 | 0xD000 => &mut self.wram[addr & 0x1FFF],
            // Working RAM shadow
            0xE000 => &mut self.wram[addr & 0x1FFF],
            0xF000 => match addr & 0x0F00 {
                // Working RAM shadow
                a if a < 0xD00 => &mut self.wram[addr & 0x1FFF],
                // Sprite attributes - only 160 bytes long
                0xE00 if addr < 0xFEA0 => &mut self.gpu.oam[addr & 0xFF],
                0xE00 => panic!("can't get mutable borrow of address {:X}", addr),
                // Zero page
                0xF00 if addr >= 0xFF80 => &mut self.zram[addr & 0x7F],
                // Unhandled
                0xF00 => panic!("can't get mutable borrow of address {:X}", addr),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
