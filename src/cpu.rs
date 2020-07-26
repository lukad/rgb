const ZERO_FLAG_POSITION: u8 = 7;
const SUBTRACT_FLAG_POSITION: u8 = 6;
const HALF_CARRY_FLAG_POSITION: u8 = 5;
const CARRY_FLAG_POSITION: u8 = 4;

use crate::instruction::*;
use crate::memory_bus::MemoryBus;
use std::io;

struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    fn clear(&mut self) {
        self.zero = false;
        self.subtract = false;
        self.half_carry = false;
        self.carry = false;
    }
}

impl std::convert::From<Flags> for u8 {
    fn from(flags: Flags) -> u8 {
        (if flags.zero { 1 } else { 0 } << ZERO_FLAG_POSITION)
            | (if flags.subtract { 1 } else { 0 } << SUBTRACT_FLAG_POSITION)
            | (if flags.half_carry { 1 } else { 0 } << HALF_CARRY_FLAG_POSITION)
            | (if flags.carry { 1 } else { 0 } << CARRY_FLAG_POSITION)
    }
}

impl std::convert::From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags {
            zero: ((byte >> ZERO_FLAG_POSITION) & 0b1) != 0,
            subtract: ((byte >> SUBTRACT_FLAG_POSITION) & 0b1) != 0,
            half_carry: ((byte >> HALF_CARRY_FLAG_POSITION) & 0b1) != 0,
            carry: ((byte >> CARRY_FLAG_POSITION) & 0b1) != 0,
        }
    }
}

struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0.into(),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
        }
    }

    fn get_bc(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn get_de(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

pub struct CPU {
    pc: u16,
    sp: u16,
    registers: Registers,
    bus: MemoryBus,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x0100,
            sp: 0xFFFE,
            registers: Registers::new(),
            bus: MemoryBus::new(),
        }
    }

    pub fn load<R: io::Read>(&mut self, data: &mut R) {
        self.bus.load(data);
    }

    pub fn step(&mut self) -> Result<(), String> {
        let byte = self.bus.read_byte(self.pc);
        if let Some(instruction) = Instruction::from_byte(byte) {
            trace!("{:#06X}: {:?}", self.pc, instruction);
            let (new_pc, _cycles) = self.execute(instruction);
            self.pc = new_pc;
            Ok(())
        } else {
            Err(format!(
                "{:#06X}: {:#6X} - illegal instruction",
                self.pc, byte
            ))
        }
    }

    fn get_bca(&mut self) -> u8 {
        self.bus.read_byte(self.registers.get_bc())
    }

    fn get_dea(&mut self) -> u8 {
        self.bus.read_byte(self.registers.get_de())
    }

    fn get_hla(&mut self) -> u8 {
        self.bus.read_byte(self.registers.get_hl())
    }

    fn immediate_byte(&mut self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(1))
    }

    fn immediate_word(&mut self) -> u16 {
        self.bus.read_word(self.pc.wrapping_add(1))
    }

    fn execute(&mut self, instruction: Instruction) -> (u16, usize) {
        let mut next_pc = self.pc.wrapping_add(1);
        let cycles = match instruction {
            Instruction::Nop => 4,
            Instruction::Add(target) => {
                let mut cycles = 4;
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::HLA => self.bus.read_byte(self.registers.get_hl()),
                    ArithmeticTarget::Immediate => {
                        let value = self.bus.read_byte(self.pc);
                        next_pc = next_pc.wrapping_add(1);
                        cycles = 8;
                        value
                    }
                };
                let (result, overflow) = self.registers.a.overflowing_add(value);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = overflow;
                self.registers.f.half_carry = (self.registers.a & 0xF) + (result & 0xF) > 0xF;
                self.registers.a = result;
                cycles
            }
            Instruction::Adc(target) => {
                let mut cycles = 4;
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::HLA => self.bus.read_byte(self.registers.get_hl()),
                    ArithmeticTarget::Immediate => {
                        let value = self.bus.read_byte(self.pc);
                        next_pc = next_pc.wrapping_add(1);
                        cycles = 8;
                        value
                    }
                };
                let (result, overflow) = self.registers.a.overflowing_add(value);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = overflow;
                self.registers.f.half_carry = (self.registers.a & 0xF) + (result & 0xF) > 0xF;
                self.registers.a = result.wrapping_add(overflow as u8);
                cycles
            }
            Instruction::Or(target) => {
                let mut cycles = 4;
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::HLA => {
                        cycles = 8;
                        self.get_hla()
                    }
                    ArithmeticTarget::Immediate => {
                        cycles = 8;
                        let value = self.immediate_byte();
                        next_pc += 1;
                        value
                    }
                };
                self.registers.a |= value;
                self.registers.f.clear();
                self.registers.f.zero = self.registers.a == 0;
                cycles
            }
            Instruction::Xor(target) => {
                let mut cycles = 4;
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::HLA => {
                        cycles = 8;
                        self.get_hla()
                    }
                    ArithmeticTarget::Immediate => {
                        cycles = 8;
                        let value = self.immediate_byte();
                        next_pc += 1;
                        value
                    }
                };
                self.registers.a ^= value;
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                cycles
            }
            Instruction::Inc(IncDecType::Byte(target)) => {
                let mut cycles = 4;
                let register = match target {
                    IncDecByteTarget::A => &mut self.registers.a,
                    IncDecByteTarget::B => &mut self.registers.b,
                    IncDecByteTarget::C => &mut self.registers.c,
                    IncDecByteTarget::D => &mut self.registers.d,
                    IncDecByteTarget::E => &mut self.registers.e,
                    IncDecByteTarget::H => &mut self.registers.h,
                    IncDecByteTarget::L => &mut self.registers.l,
                    IncDecByteTarget::HLA => {
                        cycles = 12;
                        let hl = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        self.bus.get_mut_byte(hl)
                    }
                };
                let result = register.wrapping_add(1);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (*register & 0xF) + (result & 0xF) & 0x10 != 0;
                *register = result;
                cycles
            }
            Instruction::Inc(IncDecType::Word(IncDecWordTarget::SP)) => {
                self.sp = self.sp.wrapping_add(1);
                8
            }
            Instruction::Inc(IncDecType::Word(target)) => {
                let (high, low) = match target {
                    IncDecWordTarget::BC => (&mut self.registers.b, &mut self.registers.c),
                    IncDecWordTarget::DE => (&mut self.registers.d, &mut self.registers.e),
                    IncDecWordTarget::HL => (&mut self.registers.h, &mut self.registers.l),
                    IncDecWordTarget::SP => unreachable!(),
                };
                let combined = ((*high as u16) << 8) | (*low as u16);
                let result = combined.wrapping_add(1);
                *high = (result >> 8) as u8;
                *low = (result & 0xFF) as u8;
                8
            }
            Instruction::Dec(IncDecType::Byte(target)) => {
                let mut cycles = 4;
                let register = match target {
                    IncDecByteTarget::A => &mut self.registers.a,
                    IncDecByteTarget::B => &mut self.registers.b,
                    IncDecByteTarget::C => &mut self.registers.c,
                    IncDecByteTarget::D => &mut self.registers.d,
                    IncDecByteTarget::E => &mut self.registers.e,
                    IncDecByteTarget::H => &mut self.registers.h,
                    IncDecByteTarget::L => &mut self.registers.l,
                    IncDecByteTarget::HLA => {
                        cycles = 12;
                        let hl = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        self.bus.get_mut_byte(hl)
                    }
                };
                let result = register.wrapping_sub(1);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (*register & 0xF) + (result & 0xF) & 0x10 != 0;
                *register = result;
                cycles
            }
            Instruction::Dec(IncDecType::Word(IncDecWordTarget::SP)) => {
                self.sp = self.sp.wrapping_sub(1);
                8
            }
            Instruction::Dec(IncDecType::Word(target)) => {
                let (high, low) = match target {
                    IncDecWordTarget::BC => (&mut self.registers.b, &mut self.registers.c),
                    IncDecWordTarget::DE => (&mut self.registers.d, &mut self.registers.e),
                    IncDecWordTarget::HL => (&mut self.registers.h, &mut self.registers.l),
                    IncDecWordTarget::SP => unreachable!(),
                };
                let combined = ((*high as u16) << 8) | (*low as u16);
                let result = combined.wrapping_add(1);
                *high = (result >> 8) as u8;
                *low = (result & 0xFF) as u8;
                8
            }
            Instruction::Rra => {
                let lsb = self.registers.a & 1;
                self.registers.a >>= 1;
                self.registers.a &= !(1 << 7);
                self.registers.a |= (self.registers.f.carry as u8) << 7;
                self.registers.f.clear();
                self.registers.f.carry = lsb == 1;
                4
            }
            Instruction::Rla => {
                let msb = self.registers.a & (1 << 7);
                self.registers.a <<= 1;
                self.registers.a &= !1;
                self.registers.a |= self.registers.f.carry as u8;
                self.registers.f.clear();
                self.registers.f.carry = msb == 1;
                4
            }
            Instruction::Jp(Jump::Always(target)) => {
                next_pc = match target {
                    JumpTarget::Immediate => self.bus.read_word(next_pc),
                    JumpTarget::HLA => self.bus.read_word(self.registers.get_hl()),
                };
                16
            }
            Instruction::Jp(Jump::Conditional(jump_condition)) => {
                let condition = match jump_condition {
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                };
                if condition {
                    next_pc = next_pc.wrapping_add(self.bus.read_word(next_pc));
                    16
                } else {
                    next_pc = next_pc.wrapping_add(2);
                    12
                }
            }
            Instruction::Jr(JumpRelative::Always) => {
                next_pc = next_pc.wrapping_add(self.bus.read_byte(next_pc) as u16);
                12
            }
            Instruction::Jr(JumpRelative::Conditional(jump_condition)) => {
                let condition = match jump_condition {
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                };
                if condition {
                    next_pc = next_pc.wrapping_add(self.bus.read_byte(next_pc) as u16);
                    12
                } else {
                    next_pc = next_pc.wrapping_add(1);
                    8
                }
            }
            Instruction::Ld(LoadType::Byte(byte_target, byte_source)) => {
                let mut cycles = 4;
                let source = match byte_source {
                    LoadByteSource::A => self.registers.a,
                    LoadByteSource::B => self.registers.b,
                    LoadByteSource::C => self.registers.c,
                    LoadByteSource::D => self.registers.d,
                    LoadByteSource::E => self.registers.e,
                    LoadByteSource::H => self.registers.h,
                    LoadByteSource::L => self.registers.l,
                    LoadByteSource::Immediate => {
                        let data = self.immediate_byte();
                        next_pc += 1;
                        cycles += 4;
                        data
                    }
                    LoadByteSource::ImmediateAddress => {
                        let addr = self.immediate_word();
                        next_pc += 2;
                        cycles += 12;
                        self.bus.read_byte(addr)
                    }
                    LoadByteSource::CA => {
                        cycles += 4;
                        self.bus.read_byte(self.registers.a.into())
                    }
                    LoadByteSource::BCA => {
                        cycles += 4;
                        self.get_bca()
                    }
                    LoadByteSource::DEA => {
                        cycles += 4;
                        self.get_dea()
                    }
                    LoadByteSource::HLA => {
                        cycles += 4;
                        self.get_hla()
                    }
                    LoadByteSource::HLIA => {
                        cycles += 4;
                        let data = self.get_hla();
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_add(1));
                        data
                    }
                    LoadByteSource::HLDA => {
                        cycles += 4;
                        let data = self.get_hla();
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_sub(1));
                        data
                    }
                }
                .clone();
                let target = match byte_target {
                    LoadByteTarget::A => &mut self.registers.a,
                    LoadByteTarget::B => &mut self.registers.b,
                    LoadByteTarget::C => &mut self.registers.b,
                    LoadByteTarget::D => &mut self.registers.b,
                    LoadByteTarget::E => &mut self.registers.b,
                    LoadByteTarget::H => &mut self.registers.b,
                    LoadByteTarget::L => &mut self.registers.b,
                    LoadByteTarget::ImmediateAddress => {
                        cycles += 12;
                        let addr = self.immediate_word();
                        next_pc += 2;
                        self.bus.get_mut_byte(addr)
                    }
                    LoadByteTarget::BCA => {
                        cycles += 4;
                        self.bus.get_mut_byte(self.registers.get_bc())
                    }
                    LoadByteTarget::DEA => {
                        cycles += 4;
                        self.bus.get_mut_byte(self.registers.get_de())
                    }
                    LoadByteTarget::HLA => {
                        cycles += 4;
                        self.bus.get_mut_byte(self.registers.get_hl())
                    }
                    LoadByteTarget::HLIA => {
                        cycles += 4;
                        let data = self.bus.get_mut_byte(self.registers.get_hl());
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_add(1));
                        data
                    }
                    LoadByteTarget::HLDA => {
                        cycles += 4;
                        let data = self.bus.get_mut_byte(self.registers.get_hl());
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_sub(1));
                        data
                    }
                };

                *target = source;

                cycles
            }
            Instruction::Ld(LoadType::Word(source)) => {
                let value = self.immediate_word();
                next_pc += 2;
                match source {
                    LoadWordSource::BC => self.registers.set_bc(value),
                    LoadWordSource::DE => self.registers.set_de(value),
                    LoadWordSource::HL => self.registers.set_hl(value),
                    LoadWordSource::SP => self.sp = value,
                }
                12
            }
            Instruction::Di => {
                // TODO: Disable interrupts
                4
            }
        };

        (next_pc, cycles)
    }
}
