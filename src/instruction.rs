pub enum Instruction {
    Nop,
    Add(AddType),
    Inc(IncDecType),
    Jp(JumpCondition),
    Ld(LoadType),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Self> {
        let ins = match byte {
            0x00 => Instruction::Nop,
            0x02 => Instruction::Ld(LoadType::Byte(LoadByteTarget::BCA, LoadByteSource::A)),
            0x03 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::BC)),
            0x04 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::B)),
            0x06 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::Immediate)),
            0x0A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BCA)),
            0x0C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::C)),
            0x0E => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::Immediate)),
            0x12 => Instruction::Ld(LoadType::Byte(LoadByteTarget::DEA, LoadByteSource::A)),
            0x13 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::DE)),
            0x14 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::D)),
            0x16 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::Immediate)),
            0x1A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DEA)),
            0x1C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::E)),
            0x1E => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::Immediate)),
            0x22 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLIA, LoadByteSource::A)),
            0x23 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::HL)),
            0x24 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::H)),
            0x26 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::Immediate)),
            0x2A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLIA)),
            0x2C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::L)),
            0x2E => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::Immediate)),
            0x32 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLDA, LoadByteSource::A)),
            0x33 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::SP)),
            0x34 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::HLA)),
            0x36 => Instruction::Ld(LoadType::Byte(
                LoadByteTarget::HLA,
                LoadByteSource::Immediate,
            )),
            0x3A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLDA)),
            0x3C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::A)),
            0x3E => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Immediate)),
            0x40 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B)),
            0x41 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C)),
            0x42 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D)),
            0x43 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E)),
            0x44 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H)),
            0x45 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L)),
            0x46 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLA)),
            0x47 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A)),
            0x48 => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B)),
            0x49 => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C)),
            0x4A => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D)),
            0x4B => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E)),
            0x4C => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H)),
            0x4D => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L)),
            0x4E => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLA)),
            0x4F => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A)),
            0x50 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B)),
            0x51 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C)),
            0x52 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D)),
            0x53 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E)),
            0x54 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H)),
            0x55 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L)),
            0x56 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLA)),
            0x57 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A)),
            0x58 => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B)),
            0x59 => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C)),
            0x5A => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D)),
            0x5B => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E)),
            0x5C => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H)),
            0x5D => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L)),
            0x5E => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLA)),
            0x5F => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A)),
            0x60 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B)),
            0x61 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C)),
            0x62 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D)),
            0x63 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E)),
            0x64 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H)),
            0x65 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L)),
            0x66 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLA)),
            0x67 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A)),
            0x68 => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B)),
            0x69 => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C)),
            0x6A => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D)),
            0x6B => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E)),
            0x6C => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H)),
            0x6D => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L)),
            0x6E => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLA)),
            0x6F => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A)),
            0x70 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::B)),
            0x71 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::C)),
            0x72 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::D)),
            0x73 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::E)),
            0x74 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::H)),
            0x75 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::L)),
            0x77 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLA, LoadByteSource::A)),
            0x78 => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B)),
            0x79 => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C)),
            0x7A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D)),
            0x7B => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E)),
            0x7C => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H)),
            0x7D => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L)),
            0x7E => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLA)),
            0x7F => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A)),
            0x80 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::B)),
            0x81 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::C)),
            0x82 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::D)),
            0x83 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::E)),
            0x84 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::H)),
            0x85 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::L)),
            0x86 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::HLA)),
            0x87 => Instruction::Add(AddType::Arithmetic(ArithmeticTarget::A)),
            0xC2 => Instruction::Jp(JumpCondition::NotZero),
            0xC3 => Instruction::Jp(JumpCondition::Always(JumpTarget::Immediate)),
            0xC6 => Instruction::Add(AddType::ImmediateByte),
            0xCA => Instruction::Jp(JumpCondition::Zero),
            0xD2 => Instruction::Jp(JumpCondition::NotCarry),
            0xDA => Instruction::Jp(JumpCondition::Carry),
            0xE9 => Instruction::Jp(JumpCondition::Always(JumpTarget::HLA)),
            0xF2 => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::CA)),
            0xFA => Instruction::Ld(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::ImmediateAddress,
            )),
            0xD3 | 0xDB | 0xDC | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                return None
            }
            _ => todo!("Could not decode instruction: {:#X}", byte),
        };
        Some(ins)
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
    HLA,
}

pub enum IncDecByteTarget {
    A,
    B,
    D,
    C,
    E,
    L,
    H,
    HLA,
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
    HLA,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Immediate,
    ImmediateAddress,
    CA,
    BCA,
    DEA,
    HLA,
    HLIA,
    HLDA,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BCA,
    DEA,
    HLA,
    HLIA,
    HLDA,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => f.write_str("NOP"),
            Instruction::Add(add_type) => f.write_fmt(format_args!("ADD A, {:?}", add_type)),
            Instruction::Inc(inc_dec_type) => f.write_fmt(format_args!("INC {:?}", inc_dec_type)),
            Instruction::Jp(jump_condition) => f.write_fmt(format_args!("JP {:?}", jump_condition)),
            Instruction::Ld(load_type) => f.write_fmt(format_args!("LD {:?}", load_type)),
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
            AddType::Arithmetic(ArithmeticTarget::HLA) => "(HL)",
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
            IncDecType::Byte(IncDecByteTarget::HLA) => "(HL)",
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
            JumpTarget::HLA => f.write_str("(HL)"),
        }
    }
}

impl std::fmt::Debug for LoadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadType::Byte(target, source) => {
                f.write_fmt(format_args!("{:?}, {:?}", target, source))
            }
        }
    }
}

impl std::fmt::Debug for LoadByteSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LoadByteSource::A => "A",
            LoadByteSource::B => "B",
            LoadByteSource::C => "C",
            LoadByteSource::D => "D",
            LoadByteSource::E => "E",
            LoadByteSource::H => "H",
            LoadByteSource::L => "L",
            LoadByteSource::Immediate => "d8",
            LoadByteSource::ImmediateAddress => "(a16)",
            LoadByteSource::CA => "(C)",
            LoadByteSource::BCA => "(BC)",
            LoadByteSource::DEA => "(DE)",
            LoadByteSource::HLA => "(HL)",
            LoadByteSource::HLIA => "(HL+)",
            LoadByteSource::HLDA => "(HL-)",
        };
        f.write_str(value)
    }
}

impl std::fmt::Debug for LoadByteTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LoadByteTarget::A => "A",
            LoadByteTarget::B => "B",
            LoadByteTarget::C => "C",
            LoadByteTarget::D => "D",
            LoadByteTarget::E => "E",
            LoadByteTarget::H => "H",
            LoadByteTarget::L => "L",
            LoadByteTarget::BCA => "(BC)",
            LoadByteTarget::DEA => "(DE)",
            LoadByteTarget::HLA => "(HL)",
            LoadByteTarget::HLIA => "(HL+)",
            LoadByteTarget::HLDA => "(HL-)",
        };
        f.write_str(value)
    }
}
