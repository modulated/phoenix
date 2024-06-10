use log::trace;

use crate::{
    types::ConditionCode,
    util::{get_bits, get_reg},
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
