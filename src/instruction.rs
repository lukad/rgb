pub enum Instruction {
    Nop,
    Add(ArithmeticTarget),
    Inc(IncDecType),
    Dec(IncDecType),
    Jp(Jump),
    Jr(JumpRelative),
    Ld(LoadType),
    Di,
    Adc(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Rra,
    Rla,
    Or(ArithmeticTarget),
    Cp(ArithmeticTarget),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Self> {
        let ins = match byte {
            0x00 => Instruction::Nop,
            0x01 => Instruction::Ld(LoadType::Word(LoadWordSource::BC)),
            0x02 => Instruction::Ld(LoadType::Byte(LoadByteTarget::BCA, LoadByteSource::A)),
            0x03 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::BC)),
            0x04 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::B)),
            0x05 => Instruction::Dec(IncDecType::Byte(IncDecByteTarget::B)),
            0x06 => Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::Immediate)),
            0x0A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BCA)),
            0x0B => Instruction::Dec(IncDecType::Word(IncDecWordTarget::BC)),
            0x0C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::C)),
            0x0E => Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::Immediate)),
            0x17 => Instruction::Rla,
            0x1F => Instruction::Rra,
            0x11 => Instruction::Ld(LoadType::Word(LoadWordSource::DE)),
            0x12 => Instruction::Ld(LoadType::Byte(LoadByteTarget::DEA, LoadByteSource::A)),
            0x13 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::DE)),
            0x14 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::D)),
            0x15 => Instruction::Dec(IncDecType::Byte(IncDecByteTarget::D)),
            0x16 => Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::Immediate)),
            0x18 => Instruction::Jr(JumpRelative::Always),
            0x1A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DEA)),
            0x1B => Instruction::Dec(IncDecType::Word(IncDecWordTarget::DE)),
            0x1C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::E)),
            0x1E => Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::Immediate)),
            0x20 => Instruction::Jr(JumpRelative::Conditional(JumpCondition::NotZero)),
            0x21 => Instruction::Ld(LoadType::Word(LoadWordSource::HL)),
            0x22 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLIA, LoadByteSource::A)),
            0x23 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::HL)),
            0x24 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::H)),
            0x25 => Instruction::Dec(IncDecType::Byte(IncDecByteTarget::H)),
            0x26 => Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::Immediate)),
            0x28 => Instruction::Jr(JumpRelative::Conditional(JumpCondition::Zero)),
            0x2A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLIA)),
            0x2B => Instruction::Dec(IncDecType::Word(IncDecWordTarget::HL)),
            0x2C => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::L)),
            0x2E => Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::Immediate)),
            0x30 => Instruction::Jr(JumpRelative::Conditional(JumpCondition::NotCarry)),
            0x31 => Instruction::Ld(LoadType::Word(LoadWordSource::SP)),
            0x32 => Instruction::Ld(LoadType::Byte(LoadByteTarget::HLDA, LoadByteSource::A)),
            0x33 => Instruction::Inc(IncDecType::Word(IncDecWordTarget::SP)),
            0x34 => Instruction::Inc(IncDecType::Byte(IncDecByteTarget::HLA)),
            0x35 => Instruction::Dec(IncDecType::Byte(IncDecByteTarget::HLA)),
            0x36 => Instruction::Ld(LoadType::Byte(
                LoadByteTarget::HLA,
                LoadByteSource::Immediate,
            )),
            0x38 => Instruction::Jr(JumpRelative::Conditional(JumpCondition::Carry)),
            0x3A => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLDA)),
            0x3B => Instruction::Dec(IncDecType::Word(IncDecWordTarget::SP)),
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
            0x80 => Instruction::Add(ArithmeticTarget::B),
            0x81 => Instruction::Add(ArithmeticTarget::C),
            0x82 => Instruction::Add(ArithmeticTarget::D),
            0x83 => Instruction::Add(ArithmeticTarget::E),
            0x84 => Instruction::Add(ArithmeticTarget::H),
            0x85 => Instruction::Add(ArithmeticTarget::L),
            0x86 => Instruction::Add(ArithmeticTarget::HLA),
            0x87 => Instruction::Add(ArithmeticTarget::A),
            0x88 => Instruction::Adc(ArithmeticTarget::B),
            0x89 => Instruction::Adc(ArithmeticTarget::C),
            0x8A => Instruction::Adc(ArithmeticTarget::D),
            0x8B => Instruction::Adc(ArithmeticTarget::E),
            0x8C => Instruction::Adc(ArithmeticTarget::H),
            0x8D => Instruction::Adc(ArithmeticTarget::L),
            0x8E => Instruction::Adc(ArithmeticTarget::HLA),
            0x8F => Instruction::Adc(ArithmeticTarget::A),
            0xA8 => Instruction::Xor(ArithmeticTarget::B),
            0xA9 => Instruction::Xor(ArithmeticTarget::C),
            0xAA => Instruction::Xor(ArithmeticTarget::D),
            0xAB => Instruction::Xor(ArithmeticTarget::E),
            0xAC => Instruction::Xor(ArithmeticTarget::H),
            0xAD => Instruction::Xor(ArithmeticTarget::L),
            0xAE => Instruction::Xor(ArithmeticTarget::HLA),
            0xAF => Instruction::Xor(ArithmeticTarget::A),
            0xB0 => Instruction::Or(ArithmeticTarget::B),
            0xB1 => Instruction::Or(ArithmeticTarget::C),
            0xB2 => Instruction::Or(ArithmeticTarget::D),
            0xB3 => Instruction::Or(ArithmeticTarget::E),
            0xB4 => Instruction::Or(ArithmeticTarget::H),
            0xB5 => Instruction::Or(ArithmeticTarget::L),
            0xB6 => Instruction::Or(ArithmeticTarget::HLA),
            0xB7 => Instruction::Or(ArithmeticTarget::A),
            0xB8 => Instruction::Cp(ArithmeticTarget::B),
            0xB9 => Instruction::Cp(ArithmeticTarget::C),
            0xBA => Instruction::Cp(ArithmeticTarget::D),
            0xBB => Instruction::Cp(ArithmeticTarget::E),
            0xBC => Instruction::Cp(ArithmeticTarget::H),
            0xBD => Instruction::Cp(ArithmeticTarget::L),
            0xBE => Instruction::Cp(ArithmeticTarget::HLA),
            0xBF => Instruction::Cp(ArithmeticTarget::A),
            0xC2 => Instruction::Jp(Jump::Conditional(JumpCondition::NotZero)),
            0xC3 => Instruction::Jp(Jump::Always(JumpTarget::Immediate)),
            0xC6 => Instruction::Add(ArithmeticTarget::Immediate),
            0xCA => Instruction::Jp(Jump::Conditional(JumpCondition::Zero)),
            0xCE => Instruction::Adc(ArithmeticTarget::Immediate),
            0xD2 => Instruction::Jp(Jump::Conditional(JumpCondition::NotCarry)),
            0xD3 => return None,
            0xDA => Instruction::Jp(Jump::Conditional(JumpCondition::Carry)),
            0xDB => return None,
            0xDC => return None,
            0xE3 => return None,
            0xE4 => return None,
            0xE9 => Instruction::Jp(Jump::Always(JumpTarget::HLA)),
            0xEA => Instruction::Ld(LoadType::Byte(
                LoadByteTarget::ImmediateAddress,
                LoadByteSource::A,
            )),
            0xEB => return None,
            0xEE => Instruction::Xor(ArithmeticTarget::Immediate),
            0xEC => return None,
            0xED => return None,
            0xF2 => Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::CA)),
            0xF3 => Instruction::Di,
            0xF4 => return None,
            0xF6 => Instruction::Or(ArithmeticTarget::Immediate),
            0xFA => Instruction::Ld(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::ImmediateAddress,
            )),
            0xFC => return None,
            0xFD => return None,
            0xFE => Instruction::Cp(ArithmeticTarget::Immediate),
            _ => {
                error!("Could not decode instruction: {:#04X}", byte);
                todo!("Could not decode instruction: {:#04X}", byte)
            }
        };
        Some(ins)
    }
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
    Immediate,
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

pub enum Jump {
    Conditional(JumpCondition),
    Always(JumpTarget),
}

pub enum JumpRelative {
    Conditional(JumpCondition),
    Always,
}

pub enum JumpCondition {
    NotZero,
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
    Word(LoadWordSource),
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
    ImmediateAddress,
    BCA,
    DEA,
    HLA,
    HLIA,
    HLDA,
}

pub enum LoadWordSource {
    BC,
    DE,
    HL,
    SP,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => f.write_str("NOP"),
            Instruction::Add(target) => f.write_fmt(format_args!("ADD A, {:?}", target)),
            Instruction::Inc(inc_dec_type) => f.write_fmt(format_args!("INC {:?}", inc_dec_type)),
            Instruction::Dec(inc_dec_type) => f.write_fmt(format_args!("DEC {:?}", inc_dec_type)),
            Instruction::Rra => f.write_str("RRA"),
            Instruction::Rla => f.write_str("RLA"),
            Instruction::Jp(jump) => f.write_fmt(format_args!("JP {:?}", jump)),
            Instruction::Jr(jump) => f.write_fmt(format_args!("JR {:?}", jump)),
            Instruction::Ld(load_type) => f.write_fmt(format_args!("LD {:?}", load_type)),
            Instruction::Di => f.write_str("DI"),
            Instruction::Adc(target) => f.write_fmt(format_args!("ADC A, {:?}", target)),
            Instruction::Xor(target) => f.write_fmt(format_args!("XOR {:?}", target)),
            Instruction::Or(target) => f.write_fmt(format_args!("OR {:?}", target)),
            Instruction::Cp(target) => f.write_fmt(format_args!("CP {:?}", target)),
        }
    }
}

impl std::fmt::Debug for ArithmeticTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ArithmeticTarget::A => "A",
            ArithmeticTarget::B => "B",
            ArithmeticTarget::C => "C",
            ArithmeticTarget::D => "D",
            ArithmeticTarget::E => "E",
            ArithmeticTarget::H => "H",
            ArithmeticTarget::L => "L",
            ArithmeticTarget::HLA => "(HL)",
            ArithmeticTarget::Immediate => "a8",
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

impl std::fmt::Debug for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Jump::Conditional(condition) => f.write_fmt(format_args!("{:?}", condition)),
            Jump::Always(target) => f.write_fmt(format_args!("{:?}", target)),
        }
    }
}

impl std::fmt::Debug for JumpRelative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpRelative::Conditional(condition) => f.write_fmt(format_args!("{:?}", condition)),
            JumpRelative::Always => f.write_str("r8"),
        }
    }
}

impl std::fmt::Debug for JumpCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpCondition::NotZero => f.write_str("NZ, a16"),
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
            LoadType::Word(source) => f.write_fmt(format_args!("{:?}, d16", source)),
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
            LoadByteTarget::ImmediateAddress => "(a16)",
            LoadByteTarget::BCA => "(BC)",
            LoadByteTarget::DEA => "(DE)",
            LoadByteTarget::HLA => "(HL)",
            LoadByteTarget::HLIA => "(HL+)",
            LoadByteTarget::HLDA => "(HL-)",
        };
        f.write_str(value)
    }
}

impl std::fmt::Debug for LoadWordSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LoadWordSource::BC => "BC",
            LoadWordSource::DE => "DE",
            LoadWordSource::HL => "HL",
            LoadWordSource::SP => "SP",
        };
        f.write_str(value)
    }
}
