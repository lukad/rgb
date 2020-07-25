use std::io;

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
        let mut bus = Self {
            rom: [0; 0x8000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            zram: [0; 0xFF],
            gpu: GPU::new(),
        };
        bus.write_byte(0xFF05, 0x00);
        bus.write_byte(0xFF06, 0x00);
        bus.write_byte(0xFF07, 0x00);
        bus.write_byte(0xFF10, 0x80);
        bus.write_byte(0xFF11, 0xBF);
        bus.write_byte(0xFF12, 0xF3);
        bus.write_byte(0xFF14, 0xBF);
        bus.write_byte(0xFF16, 0x3F);
        bus.write_byte(0xFF17, 0x00);
        bus.write_byte(0xFF19, 0xBF);
        bus.write_byte(0xFF1A, 0x7F);
        bus.write_byte(0xFF1B, 0xFF);
        bus.write_byte(0xFF1C, 0xBF);
        bus.write_byte(0xFF1B, 0xFF);
        bus.write_byte(0xFF1C, 0x9F);
        bus.write_byte(0xFF1E, 0xBF);
        bus.write_byte(0xFF20, 0xFF);
        bus.write_byte(0xFF21, 0x00);
        bus.write_byte(0xFF22, 0x00);
        bus.write_byte(0xFF23, 0xBF);
        bus.write_byte(0xFF24, 0x77);
        bus.write_byte(0xFF25, 0xF3);
        // bus.write_byte(0xFF26, bus.read_byte(0xF1).wrapping_sub(0xGB));
        bus.write_byte(0xFF26, 0xF0);
        bus.write_byte(0xFF40, 0x91);
        bus.write_byte(0xFF42, 0x00);
        bus.write_byte(0xFF43, 0x00);
        bus.write_byte(0xFF45, 0x00);
        bus.write_byte(0xFF47, 0xFC);
        bus.write_byte(0xFF48, 0xFF);
        bus.write_byte(0xFF49, 0xFF);
        bus.write_byte(0xFF4A, 0x00);
        bus.write_byte(0xFF4A, 0x00);
        bus.write_byte(0xFFFF, 0x00);
        bus
    }
}

impl MemoryBus {
    pub fn load<R: io::Read>(&mut self, data: &mut R) {
        data.read(self.rom.as_mut()).expect("Could not load data");
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

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

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let addr = address as usize;
        match addr & 0xF000 {
            0x0000 => {
                // TODO: check for bios
                self.rom[addr] = value
            }
            // ROM0
            0x1000 | 0x2000 | 0x3000 => self.rom[addr] = value,
            // ROM1
            0x4000 | 0x5000 | 0x6000 | 0x7000 => self.rom[addr] = value,
            // VRAM
            0x8000 | 0x9000 => self.gpu.ram[addr & 0x1FFF] = value,
            // External RAM
            0xA000 | 0xB000 => self.eram[addr & 0x1FFF] = value,
            // Working RAM
            0xC000 | 0xD000 => self.wram[addr & 0x1FFF] = value,
            // Working RAM shadow
            0xE000 => self.wram[addr & 0x1FFF] = value,
            0xF000 => match addr & 0x0F00 {
                // Working RAM shadow
                a if a < 0xD00 => self.wram[addr & 0x1FFF] = value,
                // Sprite attributes - only 160 bytes long
                0xE00 if addr < 0xFEA0 => self.gpu.oam[addr & 0xFF] = value,
                0xE00 => (),
                // Zero page
                0xF00 if addr >= 0xFF80 => self.zram[addr & 0x7F] = value,
                // Unhandled
                0xF00 => (),
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
