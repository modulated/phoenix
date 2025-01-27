use log::trace;

use crate::{
    types::{AddressingMode, Size},
    util::{get_bits, get_size, is_carry, is_negative, sign_extend_16_to_32, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn sub_family(&mut self, inst: u16) {
        let reg = ((inst & 0b0000_1110_0000_0000) >> 9) as u8;
        let ea = AddressingMode::from(inst);
        let opmode = (inst & 0b0000_0001_1100_0000) >> 6;
        match opmode {
            0b000 => self.sub_data(reg, ea, Size::Byte),
            0b001 => self.sub_data(reg, ea, Size::Word),
            0b010 => self.sub_data(reg, ea, Size::Long),
            0b011 => self.suba(reg, ea, Size::Word),
            0b100 => self.sub_addr(reg, ea, Size::Byte),
            0b101 => self.sub_addr(reg, ea, Size::Word),
            0b110 => self.sub_addr(reg, ea, Size::Long),
            0b111 => self.suba(reg, ea, Size::Long),
            _ => unreachable!("{inst:018b}"),
        };
    }

    fn suba(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUBA.{size} A{reg} {ea:?}");
        let val1 = self.read_ar(reg);
        let val2 = self.read_ea(ea, size);
        let res = match size {
            Size::Byte => unreachable!(),
            Size::Word => val1.wrapping_sub(sign_extend_16_to_32(u32::from(val2) as u16)),
            Size::Long => val1.wrapping_sub(val2.into()),
        };
        self.write_ar(reg, res);
        sub_set_ccr(self, val1, val2.into(), res, size);
    }

    fn sub_data(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUB.{size} D{reg} {ea}");
        let val1 = self.read_dr(reg);
        let val2 = match size {
            Size::Byte => self.read_ea_byte(ea) as u32,
            Size::Word => self.read_ea_word(ea) as u32,
            Size::Long => self.read_ea_long(ea),
        };
        let res = val1.wrapping_sub(val2);
        sub_set_ccr(self, val1, val2, res, size);
        self.write_dr(reg, size, res);
    }

    fn sub_addr(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUB.{size} {ea} D{reg}");
        todo!()
    }

    pub(crate) fn subi(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        trace!("SUBI.{size} (IMM) {ea:?} ({val})");
        todo!()
    }

    pub(crate) fn subq(&mut self, inst: u16) {
        let data = get_bits(inst, 9, 3);
        let sub = if data == 0 { 8 } else { data as u8 };
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        let res = val - sub;
        trace!("SUBQ.{size} {sub}, {ea} ({val:X})");
        self.write_ea(ea, size, res);

        sub_set_ccr(self, val.into(), sub.into(), res.into(), size);
    }
}

fn sub_set_ccr(cpu: &mut Cpu, val1: u32, val2: u32, res: u32, size: Size) {
    cpu.write_ccr(SR::X, is_carry(val1, val2, res, size));
    cpu.write_ccr(SR::C, is_carry(val1, val2, res, size));
    cpu.write_ccr(SR::N, is_negative(res, size));
    cpu.write_ccr(SR::Z, res == 0);
    cpu.write_ccr(SR::V, sub_set_overflow(val1, val2, res, size));
}

pub fn sub_set_carry(a: u32, b: u32, res: u32, size: Size) -> bool {
    let sa = is_negative(a, size);
    let sb = is_negative(b, size);
    let sc = is_negative(res, size);
    (!sa && !sb && sc) || (sa && sb && !sc)
}

pub fn sub_set_overflow(a: u32, b: u32, res: u32, size: Size) -> bool {
    let sa = is_negative(a, size);
    let sb = is_negative(b, size);
    let sc = is_negative(res, size);
    (!sa && sb && sc) || (sa && !sb && !sc)
}

#[cfg(test)]
mod tests {
    use super::sub_set_carry;

    #[test]
    fn test_carry() {
        assert!(sub_set_carry(
            0b0,
            0b1,
            0b1111_1111,
            crate::types::Size::Byte
        ));
        assert!(!sub_set_carry(
            0b1000_0000,
            0b1,
            0b0111_1111,
            crate::types::Size::Byte
        ));
    }

    #[test]
    fn test_overflow() {
        assert!(sub_set_carry(
            0b0,
            0b1,
            0b1111_1111,
            crate::types::Size::Byte
        ));
        assert!(!sub_set_carry(
            0b1000_0000,
            0b1,
            0b0111_1111,
            crate::types::Size::Byte
        ));
    }
}
