use std::fmt::Display;

#[allow(dead_code)]
pub enum Size {
    Byte,
    Word,
    Long,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Byte(u8),
    Word(u16),
    Long(u32),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Byte(x) => write!(f, "{x:#04x}"),
            Value::Word(x) => write!(f, "{x:#06x}"),
            Value::Long(x) => write!(f, "{x:#08x}"),
        }
    }
}

#[allow(dead_code)]
pub fn sign_extend_8_to_32(byte: u8) -> u32 {
    if byte & 0b1000_0000 == 0 {
        byte as u32
    } else {
        0xFFFFFF00 + (byte as u32)
    }
}

pub fn sign_extend_16_to_32(word: u16) -> u32 {
    if word & 0b1000_0000_0000_0000 == 0 {
        word as u32
    } else {
        0xFFFF0000 + (word as u32)
    }
}

pub fn sign_transmute(word: u16) -> i16 {
    word as i16
}

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

pub enum ExtensionMode {
    Word = 0b000,                   // <addr>.w
    Long = 0b001,                   // <addr>.l
    PcRelativeDisplacement = 0b010, // d16(PC)
    PcRelativeIndex = 0b011,        // (d8, PC, Xn)
    Immediate = 0b100,              // #<data>
}

#[cfg(test)]
mod test {
    use super::{sign_extend_16_to_32, sign_transmute};

    #[test]
    fn test_sign_extend_word() {
        assert_eq!(sign_extend_16_to_32(0x9012), 0xFFFF9012);
        assert_eq!(sign_extend_16_to_32(0x1012), 0x00001012);
        assert_eq!(sign_extend_16_to_32(0xF231), 0xFFFFF231);
    }

    #[test]
    fn test_sign_transmute() {
        assert_eq!(sign_transmute(0xFFFF), -1);
        assert_eq!(sign_transmute(0), 0);
        assert_eq!(sign_transmute(10), 10);
        assert_eq!(sign_transmute(0xFFF6), -10);
    }
}
