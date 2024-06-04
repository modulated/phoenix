use crate::types::Size;

#[allow(dead_code)]
pub fn sign_extend_8_to_16(byte: u8) -> u16 {
    if byte & 0b1000_0000 == 0 {
        byte as u16
    } else {
        0xFF00 + (byte as u16)
    }
}

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

pub(crate) fn is_bit_set<T: Into<u32>>(val: T, pos: u8) -> bool {
    val.into() & (1 << pos) as u32 == (1 << pos) as u32
}

pub(crate) fn is_negative<T: Into<u32>>(val: T, size: Size) -> bool {
    match size {
        Size::Byte => val.into() & (1 << 7) == (1 << 7),
        Size::Word => val.into() & (1 << 15) == (1 << 15),
        Size::Long => val.into() & (1 << 31) == (1 << 31),
    }
}

// TODO: this needs fixing
pub(crate) fn is_carry(val1: u32, val2: u32, res: u32, size: Size) -> bool {
    match size {
        Size::Byte => val1 as u8 > res as u8 || val2 as u8 > res as u8,
        Size::Word => val1 as u16 > res as u16 || val2 as u16 > res as u16,
        Size::Long => val1 > res || val2 > res,
    }
}

// TODO: this needs fixing
pub(crate) fn is_overflow(val1: u32, val2: u32, res: u32, size: Size) -> bool {
    match size {
        Size::Byte => (val1 & 0x80) == (val2 & 0x80) && (val1 & 0x80 != res & 0x80),
        Size::Word => (val1 & 0x8000) == (val2 & 0x8000) && (val1 & 0x8000 != res & 0x8000),
        Size::Long => {
            (val1 & 0x80000000) == (val2 & 0x80000000) && (val1 & 0x80000000 != res & 0x80000000)
        }
    }
}

pub(crate) fn get_bits(inst: u16, idx: u8, len: u8) -> u16 {
    assert!(len > 0 && (idx + len) < 17);
    let mask = (1u16 << (len)).wrapping_sub(1);
    let mask = mask << idx;
    (mask & inst) >> idx
}

pub(crate) fn get_reg(inst: u16, idx: u8) -> u8 {
    get_bits(inst, idx, 3) as u8
}

pub(crate) fn get_size(inst: u16, idx: u8, coding: SizeCoding) -> Size {
    match coding {
        SizeCoding::Pink => match get_bits(inst, idx, 2) {
            0b00 => Size::Byte,
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        },
        SizeCoding::Purple => match get_bits(inst, idx, 2) {
            0b01 => Size::Byte,
            0b11 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        },
    }
}

#[allow(dead_code)]
pub fn nibble_to_bcd(byte: u8) -> u8 {
    match byte {
        0b0000 => 0b0000, // 0
        0b0001 => 0b0001, // 1
        0b0010 => 0b0010, // 2
        0b0011 => 0b0011, // 3
        0b0100 => 0b0000, // 4
        0b0101 => 0b0001, // 5
        0b0110 => 0b0010, // 6
        0b0111 => 0b0011, // 7
        0b1000 => 0b1001, // 8
        0b1001 => 0b1001, // 9
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn byte_to_packed_bcd(byte: u8) -> u8 {
    let lo = nibble_to_bcd(byte & 0xF);
    let hi = nibble_to_bcd(byte >> 4);
    hi + lo
}

pub enum SizeCoding {
    Pink,
    Purple,
}

#[cfg(test)]
mod test {
    use crate::{
        types::Size,
        util::{byte_to_packed_bcd, is_negative, is_overflow},
    };

    use super::{get_bits, is_bit_set, sign_extend_16_to_32, sign_transmute};
    #[test]
    fn test_is_bit_set() {
        assert!(!is_bit_set(0b0010_0000_1011_0101u16, 15));
        assert!(is_bit_set(0b0010_0000_1011_0101u16, 0));
        assert!(is_bit_set(0b0010_0000_1011_0101u16, 7));
    }

    #[test]
    fn test_get_bits() {
        assert_eq!(get_bits(0b0000_1001_0101_0110, 0, 3), 0b110);
        assert_eq!(get_bits(0b0010_1001_0101_0110, 8, 8), 0b0010_1001);
        assert_eq!(get_bits(0b1010_1001_0101_0110, 12, 4), 0b1010);
    }

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

    #[test]
    fn test_is_neg() {
        assert!(is_negative(0b1000_1011u8, crate::types::Size::Byte));
        assert!(!is_negative(0b0100_1011u8, crate::types::Size::Byte));
        assert!(is_negative(
            0b1010_1010_0100_1011u16,
            crate::types::Size::Word
        ));
        assert!(!is_negative(
            0b0101_0101_1000_1011u16,
            crate::types::Size::Word
        ));
        assert!(is_negative(
            0b1000_1011_1000_1011_1000_1011_1000_1011u32,
            crate::types::Size::Long
        ));
        assert!(!is_negative(
            0b0100_1011_1000_1011_1000_1011_1000_1011u32,
            crate::types::Size::Long
        ));
    }

    #[test]
    fn test_is_overflow() {
        let a = 0x0Fu32;
        let b = 0xFAu32;
        assert!(is_overflow(a, b, a + b, Size::Byte));
    }

    #[test]
    fn test_bcd() {
        assert_eq!(byte_to_packed_bcd(35), 0b0011_0101);
        assert_eq!(byte_to_packed_bcd(95), 0b1001_0101);
    }
}
