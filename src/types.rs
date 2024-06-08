use std::{
    cmp::Eq,
    fmt::{Display, UpperHex},
    ops::{Add, BitOrAssign, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Byte => write!(f, "b"),
            Size::Word => write!(f, "w"),
            Size::Long => write!(f, "l"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Byte(u8),
    Word(u16),
    Long(u32),
}

impl Value {
    pub fn is_bit_set(&self, idx: i8) -> bool {
        match self {
            Value::Byte(val) => {
                if idx == -1 {
                    val & (1 << 7) == (1 << 7)
                } else {
                    val & (1 << idx) == (1 << idx)
                }
            }
            Value::Word(val) => {
                if idx == -1 {
                    val & (1 << 15) == (1 << 15)
                } else {
                    val & (1 << idx) == (1 << idx)
                }
            }
            Value::Long(val) => {
                if idx == -1 {
                    val & (1 << 31) == (1 << 31)
                } else {
                    val & (1 << idx) == (1 << idx)
                }
            }
        }
    }
}

impl Sub<u8> for Value {
    type Output = Value;

    fn sub(self, rhs: u8) -> Self::Output {
        match self {
            Value::Byte(v) => Value::Byte(v.wrapping_sub(rhs)),
            Value::Word(v) => Value::Word(v.wrapping_sub(rhs as u16)),
            Value::Long(v) => Value::Long(v.wrapping_sub(rhs as u32)),
        }
    }
}

impl Add<u8> for Value {
    type Output = Value;

    fn add(self, rhs: u8) -> Self::Output {
        match self {
            Value::Byte(v) => Value::Byte(v.wrapping_add(rhs)),
            Value::Word(v) => Value::Word(v.wrapping_add(rhs as u16)),
            Value::Long(v) => Value::Long(v.wrapping_add(rhs as u32)),
        }
    }
}

impl From<Value> for u32 {
    fn from(value: Value) -> Self {
        match value {
            Value::Byte(v) => v as u32,
            Value::Word(v) => v as u32,
            Value::Long(v) => v,
        }
    }
}

impl BitOrAssign<u8> for Value {
    fn bitor_assign(&mut self, rhs: u8) {
        match self {
            Value::Byte(val) => {
                *val |= rhs;
            }
            Value::Word(val) => {
                *val |= rhs as u16;
            }
            Value::Long(val) => {
                *val |= rhs as u32;
            }
        }
    }
}

impl BitOrAssign<u16> for Value {
    fn bitor_assign(&mut self, rhs: u16) {
        match self {
            Value::Byte(val) => {
                *val |= rhs as u8;
            }
            Value::Word(val) => {
                *val |= rhs;
            }
            Value::Long(val) => {
                *val |= rhs as u32;
            }
        }
    }
}

impl BitOrAssign<u32> for Value {
    fn bitor_assign(&mut self, rhs: u32) {
        match self {
            Value::Byte(val) => {
                *val |= rhs as u8;
            }
            Value::Word(val) => {
                *val |= rhs as u16;
            }
            Value::Long(val) => {
                *val |= rhs;
            }
        }
    }
}

impl PartialEq<u32> for Value {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Value::Byte(val) => *val == *other as u8,
            Value::Word(val) => *val == *other as u16,
            Value::Long(val) => val == other,
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

impl Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingMode::DataRegisterDirect(r) => write!(f, "D{}", r),
            AddressingMode::AddressRegisterDirect(r) => write!(f, "A{}", r),
            AddressingMode::AddressRegisterIndirect(r) => write!(f, "(A{})", r),
            AddressingMode::AddressRegisterIndirectPostIncrement(r) => write!(f, "(A{})+", r),
            AddressingMode::AddressRegisterIndirectPreDecrement(r) => write!(f, "-(A{})", r),
            AddressingMode::AddressRegisterIndirectDisplacement(r) => write!(f, "(d16,A{})", r),
            AddressingMode::AddressRegisterIndirectIndex(r) => write!(f, "(d16,A{},Xn)", r),
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => write!(f, "Abs Word"),
                ExtensionMode::Long => write!(f, "Abs Long"),
                ExtensionMode::PcRelativeDisplacement => write!(f, "d16(PC)"),
                ExtensionMode::PcRelativeIndex => write!(f, "d8(PC,Xn)"),
                ExtensionMode::Immediate => write!(f, "Immediate"),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionCode {
    True,
    False,
    Higher,
    LowerOrSame,
    CarryClear,
    CarrySet,
    NotEqual,
    Equal,
    OverflowClear,
    OverflowSet,
    Plus,
    Minus,
    GreaterOrEqual,
    LessThan,
    GreatherThan,
    LessOrEqual,
}

impl From<u8> for ConditionCode {
    fn from(value: u8) -> Self {
        use ConditionCode::*;
        match value {
            0b0000 => True,
            0b0001 => False,
            0b0010 => Higher,
            0b0011 => LowerOrSame,
            0b0100 => CarryClear,
            0b0101 => CarrySet,
            0b0110 => NotEqual,
            0b0111 => Equal,
            0b1000 => OverflowClear,
            0b1001 => OverflowSet,
            0b1010 => Plus,
            0b1011 => Minus,
            0b1100 => GreaterOrEqual,
            0b1101 => LessThan,
            0b1110 => GreatherThan,
            0b1111 => LessOrEqual,
            _ => unreachable!("Not 4 bit value"),
        }
    }
}

impl Display for ConditionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionCode::True => write!(f, "T"),
            ConditionCode::False => write!(f, "F"),
            ConditionCode::Higher => write!(f, "HI"),
            ConditionCode::LowerOrSame => write!(f, "LS"),
            ConditionCode::CarryClear => write!(f, "CC"),
            ConditionCode::CarrySet => write!(f, "CS"),
            ConditionCode::NotEqual => write!(f, "NE"),
            ConditionCode::Equal => write!(f, "EQ"),
            ConditionCode::OverflowClear => write!(f, "VC"),
            ConditionCode::OverflowSet => write!(f, "VS"),
            ConditionCode::Plus => write!(f, "PL"),
            ConditionCode::Minus => write!(f, "MI"),
            ConditionCode::GreaterOrEqual => write!(f, "GE"),
            ConditionCode::LessThan => write!(f, "LT"),
            ConditionCode::GreatherThan => write!(f, "GT"),
            ConditionCode::LessOrEqual => write!(f, "LE"),
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

#[cfg(test)]
mod test {
    use crate::types::Value;

    #[test]
    fn test_is_bit_set() {
        assert!(!Value::Byte(0b1010_0010).is_bit_set(0));
        assert!(Value::Byte(0b1010_0010).is_bit_set(-1));
        assert!(Value::Word(0b1010_0010_1010_0010).is_bit_set(-1));
        assert!(Value::Long(0b1101_1000_1010_0010_1101_1000_1010_0010).is_bit_set(-1));
    }
}
