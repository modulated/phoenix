use crate::types::Size;

#[allow(dead_code)]
pub fn sign_extend_8_to_16(byte: u8) -> u16 {
    if byte & 0b1000_0000 == 0 {
        byte as u16
    } else {
        0xFF00 + (byte as u16)
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

pub(crate) fn is_bit_set(val: u16, pos: u8) -> bool {
    val & (1 << pos) == (1 << pos)
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

pub enum SizeCoding {
    Pink,
    Purple,
}

#[cfg(test)]
mod test {
    use super::{get_bits, is_bit_set, sign_extend_16_to_32, sign_transmute};
    #[test]
    fn test_is_bit_set() {
        assert!(!is_bit_set(0b0010_0000_1011_0101, 15));
        assert!(is_bit_set(0b0010_0000_1011_0101, 0));
        assert!(is_bit_set(0b0010_0000_1011_0101, 7));
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
}
