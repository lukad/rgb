pub enum Instruction {
    Nop,
    Add(AddType),
    Inc(IncDecType),
    Jp(JumpCondition),
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
            0xC2 => Instruction::Jp(JumpCondition::NotZero),
            0xC3 => Instruction::Jp(JumpCondition::Always(JumpTarget::Immediate)),
            0xC6 => Instruction::Add(AddType::ImmediateByte),
            0xCA => Instruction::Jp(JumpCondition::Zero),
            0xD2 => Instruction::Jp(JumpCondition::NotCarry),
            0xDA => Instruction::Jp(JumpCondition::Carry),
            0xE9 => Instruction::Jp(JumpCondition::Always(JumpTarget::HLI)),
            _ => todo!("Could not decode instruction: {:#X}", byte),
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

pub enum JumpCondition {
    NotZero,
    Always(JumpTarget),
    NotCarry,
    Zero,
    Carry,
}

pub enum JumpTarget {
    Immediate,
    HLI,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => f.write_str("NOP"),
            Instruction::Add(add_type) => f.write_fmt(format_args!("ADD A, {:?}", add_type)),
            Instruction::Inc(inc_dec_type) => f.write_fmt(format_args!("INC {:?}", inc_dec_type)),
            Instruction::Jp(jump_condition) => f.write_fmt(format_args!("JP {:?}", jump_condition)),
        }
    }
}

impl std::fmt::Debug for AddType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AddType::Arithmetic(ArithmeticTarget::A) => "A",
            AddType::Arithmetic(ArithmeticTarget::B) => "B",
            AddType::Arithmetic(ArithmeticTarget::C) => "C",
            AddType::Arithmetic(ArithmeticTarget::D) => "D",
            AddType::Arithmetic(ArithmeticTarget::E) => "E",
            AddType::Arithmetic(ArithmeticTarget::H) => "H",
            AddType::Arithmetic(ArithmeticTarget::L) => "L",
            AddType::Arithmetic(ArithmeticTarget::HLI) => "(HLI)",
            AddType::ImmediateByte => "d8",
        };
        f.write_str(value)
    }
}

impl std::fmt::Debug for IncDecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            IncDecType::Byte(IncDecByteTarget::A) => "A",
            IncDecType::Byte(IncDecByteTarget::B) => "B",
            IncDecType::Byte(IncDecByteTarget::C) => "C",
            IncDecType::Byte(IncDecByteTarget::D) => "D",
            IncDecType::Byte(IncDecByteTarget::E) => "E",
            IncDecType::Byte(IncDecByteTarget::H) => "H",
            IncDecType::Byte(IncDecByteTarget::L) => "L",
            IncDecType::Byte(IncDecByteTarget::HLI) => "(HLI)",
            IncDecType::Word(IncDecWordTarget::BC) => "BC",
            IncDecType::Word(IncDecWordTarget::DE) => "DE",
            IncDecType::Word(IncDecWordTarget::HL) => "HL",
            IncDecType::Word(IncDecWordTarget::SP) => "SP",
        };
        f.write_str(value)
    }
}

impl std::fmt::Debug for JumpCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpCondition::NotZero => f.write_str("NZ, a16"),
            JumpCondition::Always(target) => f.write_fmt(format_args!("{:?}", target)),
            JumpCondition::NotCarry => f.write_str("NC, a16"),
            JumpCondition::Zero => f.write_str("Z, a16"),
            JumpCondition::Carry => f.write_str("C, a16"),
        }
    }
}

impl std::fmt::Debug for JumpTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpTarget::Immediate => f.write_str("a16"),
            JumpTarget::HLI => f.write_str("(HLI)"),
        }
    }
}
