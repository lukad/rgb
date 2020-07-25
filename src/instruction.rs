pub enum Instruction {
    Nop,
    Add(AddType),
    Inc(IncDecType),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Instruction::Nop,
            0x03 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::BC)),
            0x04 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::B)),
            0x0C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::C)),
            0x13 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::DE)),
            0x14 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::D)),
            0x1C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::E)),
            0x23 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::HL)),
            0x24 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::H)),
            0x2C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::L)),
            0x33 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::SP)),
            0x34 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::HLI)),
            0x3C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::A)),
            0x80 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::B)),
            0x81 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::C)),
            0x82 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::D)),
            0x83 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::E)),
            0x84 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::H)),
            0x85 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::L)),
            0x86 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::HLI)),
            0x87 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::A)),
            0xC6 => Instruction::Add(AddType::ImmediateByte),
            _ => todo!("Could not decode instruction: {:X}", byte),
        }
    }
}

pub enum AddType {
    Arithmetic(ArithmeticTarget),
    ImmediateByte,
}

pub enum IncDecType {
    Byte(IncDecByteTarget),
    Word(IncDecWordTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum IncDecByteTarget {
    A,
    B,
    D,
    C,
    E,
    L,
    H,
    HLI,
}

pub enum IncDecWordTarget {
    BC,
    DE,
    HL,
    SP,
}
