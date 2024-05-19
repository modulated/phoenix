use std::{
    fmt::{Display, UpperHex},
    ops::Sub,
};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Byte(u8),
    Word(u16),
    Long(u32),
}

impl Sub<u8> for Value {
    type Output = Value;

    fn sub(self, rhs: u8) -> Self::Output {
        match self {
            Value::Byte(v) => Value::Byte(v - rhs),
            Value::Word(v) => Value::Word(v - rhs as u16),
            Value::Long(v) => Value::Long(v - rhs as u32),
        }
    }
}

impl UpperHex for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Byte(v) => write!(f, "{v:#04X}"),
            Value::Word(v) => write!(f, "{v:#06X}"),
            Value::Long(v) => write!(f, "{v:#010X}"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Byte(x) => write!(f, "{x:#04X}"),
            Value::Word(x) => write!(f, "{x:#06X}"),
            Value::Long(x) => write!(f, "{x:#010X}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    DataRegisterDirect(u8),                   // Dn
    AddressRegisterDirect(u8),                // An
    AddressRegisterIndirect(u8),              // (An)
    AddressRegisterIndirectPostIncrement(u8), // (An)+
    AddressRegisterIndirectPreDecrement(u8),  // -(An)
    AddressRegisterIndirectDisplacement(u8),  // (d16,An)
    AddressRegisterIndirectIndex(u8),         // (d8,An,Xn)
    Extension(ExtensionMode),
}

impl AddressingMode {
    pub fn from(inst: u16) -> Self {
        let mode = (0b0000_0000_0011_1000 & inst) >> 3;
        let reg = 0b111 & inst;
        match mode {
            0b000 => Self::DataRegisterDirect(reg.try_into().unwrap()),
            0b001 => Self::AddressRegisterDirect(reg.try_into().unwrap()),
            0b010 => Self::AddressRegisterIndirect(reg.try_into().unwrap()),
            0b011 => Self::AddressRegisterIndirectPostIncrement(reg.try_into().unwrap()),
            0b100 => Self::AddressRegisterIndirectPreDecrement(reg.try_into().unwrap()),
            0b101 => Self::AddressRegisterIndirectDisplacement(reg.try_into().unwrap()),
            0b110 => Self::AddressRegisterIndirectIndex(reg.try_into().unwrap()),
            0b111 => match reg {
                0b000 => Self::Extension(ExtensionMode::Word),
                0b001 => Self::Extension(ExtensionMode::Long),
                0b010 => Self::Extension(ExtensionMode::PcRelativeDisplacement),
                0b011 => Self::Extension(ExtensionMode::PcRelativeIndex),
                0b100 => Self::Extension(ExtensionMode::Immediate),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExtensionMode {
    Word = 0b000,                   // <addr>.w
    Long = 0b001,                   // <addr>.l
    PcRelativeDisplacement = 0b010, // d16(PC)
    PcRelativeIndex = 0b011,        // (d8, PC, Xn)
    Immediate = 0b100,              // #<data>
}
