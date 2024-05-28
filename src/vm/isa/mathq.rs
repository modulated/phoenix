use log::trace;

use crate::{
    types::{AddressingMode, ConditionCode},
    util::{get_bits, get_reg, get_size, SizeCoding},
    vm::cpu::Cpu,
};

impl<'a> Cpu<'a> {
    pub(super) fn mathq_family(&mut self, inst: u16) {
        if get_bits(inst, 6, 2) == 0b11 {
            if get_bits(inst, 3, 3) == 0b001 {
                self.dbcc(inst);
            } else {
                self.scc(inst);
            }
        } else if get_bits(inst, 8, 1) == 0b1 {
            self.subq(inst);
        } else {
            self.addq(inst);
        }
    }

    fn addq(&mut self, _inst: u16) {
        todo!()
    }

    fn subq(&mut self, inst: u16) {
        let data = get_bits(inst, 9, 3);
        let sub = if data == 0 { 8 } else { data as u8 };
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        trace!("SUBQ {sub} {ea:?}: {val:X}");
        self.write_ea(ea, size, val - sub);
        // TODO: flags
    }

    fn scc(&mut self, _inst: u16) {
        todo!()
    }

    fn dbcc(&mut self, inst: u16) {
        let cc = ConditionCode::from(get_bits(inst, 8, 3) as u8);
        let reg = get_reg(inst, 0);
        let pc = self.read_pc();
        let displacement = self.fetch_signed_word();
        trace!("DB{cc} D{reg}");
        if !self.test_cc(cc) {
            trace!("Cond false");
            self.decrement_dr(reg, 1);
            if self.read_dr(reg) != 0xFFFFFFFF {
                let target = (pc as i64 + displacement as i64) as u32;
                self.write_pc(target);
            }
        } else {
            trace!("Cond true");
        }
    }
}
