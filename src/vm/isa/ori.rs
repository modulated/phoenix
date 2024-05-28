use log::{error, trace};

use crate::{
    types::{AddressingMode, Size},
    util::{get_size, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR, Vector,
};

impl<'a> Cpu<'a> {
    pub(super) fn ori_family(&mut self, inst: u16) {
        if inst == 0b0000_0000_0011_1100 {
            return self.ori_to_ccr();
        }
        if inst == 0b0000_0000_0111_1100 {
            return self.ori_to_sr();
        }
        self.ori(inst)
    }

    fn ori_to_ccr(&mut self) {
        let val = self.fetch_word() & 0xFF;
        let old = self.read_sr();
        trace!("ORI to CCR {val:#010b}");
        self.write_sr((old & 0xFF00) + ((old & 0xFF) | val));
    }

    fn ori_to_sr(&mut self) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
        let val = self.fetch_word();
        let old = self.read_sr();
        trace!("ORI to SR {val:#018b}");
        self.write_sr(old | (val & 0b1010_0111_1111_1111));
    }

    fn ori(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let mut val = self.read_ea(ea, size);
        let imm = match size {
            Size::Byte => (self.fetch_word() & 0xFF) as u32,
            Size::Word => self.fetch_word() as u32,
            Size::Long => self.fetch_long(),
        };
        val |= imm;
        self.write_ea(ea, size, val);
        self.write_ccr(SR::N, val.is_bit_set(-1));
        self.write_ccr(SR::Z, val == 0);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);

        trace!("ORI {ea:?}: {val} | {imm:#X} {size:?}");
    }
}
