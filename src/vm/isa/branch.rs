use crate::{types::ConditionCode, util::get_bits, vm::cpu::Cpu};
use log::trace;

impl<'a> Cpu<'a> {
    pub(super) fn branch_family(&mut self, inst: u16) {
        match get_bits(inst, 8, 4) {
            0b0000 => self.bra(inst),
            0b0001 => self.bsr(inst),
            _ => self.bcc(inst),
        }
    }

    fn bra(&mut self, inst: u16) {
        let val = get_bits(inst, 0, 8);
        let pc = self.read_pc();
        let disp = if val == 0 {
            self.fetch_signed_word() as i64
        } else {
            val as i64
        };
        trace!("BRA {disp:#X}");
        self.write_pc((pc as i64 + disp) as u32);
    }

    fn bsr(&mut self, inst: u16) {
        let val = get_bits(inst, 0, 8);
        let pc = self.read_pc();
        let disp = if val == 0 {
            self.fetch_signed_word() as i64
        } else {
            val as i64
        };
        trace!("BSR {disp:#X}");
        self.push_long(pc);
        self.write_pc((pc as i64 + disp) as u32);
    }

    fn bcc(&mut self, inst: u16) {
        let cc = ConditionCode::from(get_bits(inst, 8, 4) as u8);
        let pc = self.read_pc() - 2;
        let disp = inst as u8;
        let disp = if disp == 0 {
            self.fetch_signed_word()
        } else {
            disp as i16
        };
        trace!("B{cc} {disp:#X}");
        if self.test_cc(cc) {
            self.write_pc((pc as i64 + disp as i64) as u32);
        }
    }
}