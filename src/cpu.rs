const ZERO_FLAG_POSITION: u8 = 7;
const SUBTRACT_FLAG_POSITION: u8 = 6;
const HALF_CARRY_FLAG_POSITION: u8 = 5;
const CARRY_FLAG_POSITION: u8 = 4;

use crate::instruction::*;
use crate::memory_bus::MemoryBus;
use std::io;

#[derive(Default)]
struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
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
}

pub struct CPU {
    pc: u16,
    sp: u16,
    registers: Registers,
    bus: MemoryBus,
}

impl Registers {
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
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

    pub fn step(&mut self) {
        let byte = self.bus.read_byte(self.pc);
        let instruction = Instruction::from_byte(byte);
        trace!("{:#06X}: {:?}", self.pc, instruction);
        let (new_pc, _cycles) = self.execute(instruction);
        self.pc = new_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> (u16, usize) {
        let mut next_pc = self.pc.wrapping_add(1);
        let cycles = match instruction {
            Instruction::Nop => 4,
            Instruction::Add(target) => {
                let mut cycles = 4;
                let value = match target {
                    AddType::Arithmetic(ArithmeticTarget::A) => self.registers.a,
                    AddType::Arithmetic(ArithmeticTarget::B) => self.registers.b,
                    AddType::Arithmetic(ArithmeticTarget::C) => self.registers.c,
                    AddType::Arithmetic(ArithmeticTarget::D) => self.registers.d,
                    AddType::Arithmetic(ArithmeticTarget::E) => self.registers.e,
                    AddType::Arithmetic(ArithmeticTarget::H) => self.registers.h,
                    AddType::Arithmetic(ArithmeticTarget::L) => self.registers.l,
                    AddType::Arithmetic(ArithmeticTarget::HLI) => {
                        self.bus.read_byte(self.registers.get_hl())
                    }
                    AddType::ImmediateByte => {
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
            Instruction::Inc(IncDecType::Byte(target)) => {
                let register = match target {
                    IncDecByteTarget::A => &mut self.registers.a,
                    IncDecByteTarget::B => &mut self.registers.b,
                    IncDecByteTarget::C => &mut self.registers.c,
                    IncDecByteTarget::D => &mut self.registers.d,
                    IncDecByteTarget::E => &mut self.registers.e,
                    IncDecByteTarget::H => &mut self.registers.h,
                    IncDecByteTarget::L => &mut self.registers.l,
                    IncDecByteTarget::HLI => {
                        let hl = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
                        self.bus.get_mut_byte(hl)
                    }
                };
                let (result, overflow) = register.overflowing_add(1);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = overflow;
                self.registers.f.half_carry = (*register & 0xF) + (result & 0xF) > 0xF;
                *register = result;
                4
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
            Instruction::Jp(JumpCondition::Always(target)) => {
                next_pc = match target {
                    JumpTarget::Immediate => self.bus.read_word(next_pc),
                    JumpTarget::HLI => self.bus.read_word(self.registers.get_hl()),
                };
                16
            }
            Instruction::Jp(jump_condition) => {
                let condition = match jump_condition {
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::Always(_) => unreachable!(),
                };
                if condition {
                    next_pc = self.bus.read_word(next_pc);
                    16
                } else {
                    next_pc += 2;
                    12
                }
            }
        };

        (next_pc, cycles)
    }
}
