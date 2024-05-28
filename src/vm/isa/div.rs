use log::trace;

use crate::{
    types::{AddressingMode, Value},
    util::{get_bits, get_reg, get_size, is_bit_set, is_negative, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn div_family(&mut self, inst: u16) {
        if inst >> 4 & 0b11111 == 0b10000 {
            return self.sbcd(inst);
        }
        match get_bits(inst, 6, 3) {
            0b011 => self.divu(inst),
            0b111 => self.divs(inst),
            _ => self.or(inst),
        }
    }

    fn divu(&mut self, _inst: u16) {
        todo!()
    }

    fn divs(&mut self, _inst: u16) {
        todo!()
    }

    fn sbcd(&mut self, _inst: u16) {
        todo!()
    }

    fn or(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let reg = get_reg(inst, 9);
        let val1 = self.read_dr(reg);
        let ea = AddressingMode::from(inst);
        let val2: u32 = self.read_ea(ea, size).into();
        let result = val1 | val2;
        if is_bit_set(inst, 8) {
            // Set EA
            trace!("OR.{size} D{reg} {ea} ({val2:#X})");
            self.write_ea(ea, size, Value::Long(result));
        } else {
            // Set Dn
            trace!("OR.{size} {ea} ({val2:#X}) D{reg}");
            self.write_dr(reg, size, result);
        }

        self.write_ccr(SR::N, is_negative(result, size));
        self.write_ccr(SR::Z, result == 0);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);
    }
}
