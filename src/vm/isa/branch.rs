use crate::{
    types::ConditionCode,
    util::{get_bits, sign_extend_8_to_32},
    vm::cpu::Cpu,
};
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
        let val = get_bits(inst, 0, 8) as u8;
        let pc = self.read_pc();
        let displacement = if val == 0 {
            self.fetch_signed_word() as i32
        } else {
            sign_extend_8_to_32(val) as i32
        };
        trace!("BRA {displacement:#X}");
        self.write_pc((pc as i32 + displacement) as u32);
    }

    fn bsr(&mut self, inst: u16) {
        let val = get_bits(inst, 0, 8);
        let pc = self.read_pc();
        let displacement = if val == 0 {
            self.fetch_signed_word() as i64
        } else {
            val as i64
        };
        trace!("BSR {displacement:#X}");
        self.push_long(pc);
        self.write_pc((pc as i64 + displacement) as u32);
    }

    fn bcc(&mut self, inst: u16) {
        let cc = ConditionCode::from(get_bits(inst, 8, 4) as u8);
        let pc = self.read_pc();
        let displacement = inst as u8;
        let displacement = if displacement == 0 {
            self.fetch_signed_word()
        } else {
            displacement as i16
        };
        trace!("B{cc} {displacement:#X}");
        if self.test_cc(cc) {
            self.write_pc((pc as i64 + displacement as i64) as u32);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{types::ConditionCode, vm::cpu::Cpu};

//     #[test]
//     fn test_bmi() {
//         let cpu = Cpu::default();
//         assert!(cpu.test_cc(ConditionCode::Minus));
//     }
// }
