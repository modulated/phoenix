use log::{error, trace};

use crate::{
    types::{AddressingMode, Value},
    util::{get_size, is_negative},
    vm::cpu::Cpu,
    StatusRegister as SR, Vector,
};

impl<'a> Cpu<'a> {
    pub(super) fn eori_family(&mut self, inst: u16) {
        if inst == 0b0000_1010_0011_1100 {
            return self.eori_to_ccr();
        }
        if inst == 0b0000_1010_0111_1100 {
            return self.eori_to_sr();
        }
        self.eori(inst)
    }

    fn eori_to_sr(&mut self) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
        let val = self.fetch_word();
        let old = self.read_sr();
        trace!("EORI to SR {val:#018b}");
        self.write_sr(old ^ (val & 0b1010_0111_1111_1111));
    }

    fn eori_to_ccr(&mut self) {
        let val = self.fetch_word() & 0xFF;
        let old = self.read_sr();
        trace!("EORI to CCR {val:#010b}");
        self.write_sr((old & 0xFF00) + ((old & 0xFF) ^ val));
    }

    fn eori(&mut self, inst: u16) {
        let size = get_size(inst, 6, crate::util::SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val2 = u32::from(self.read_ea(ea, size));
        let val1 = match size {
            crate::types::Size::Byte => (self.fetch_word() & 0xFF) as u32,
            crate::types::Size::Word => self.fetch_word() as u32,
            crate::types::Size::Long => self.fetch_long(),
        };
        trace!("EORI.{size} {ea} ({val2:#X}) {val1:#X}");
        let res = val1 ^ val2;
        self.write_ea(ea, size, Value::Long(res));

        self.write_ccr(SR::N, is_negative(res, size));
        self.write_ccr(SR::Z, res == 0);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);
    }
}
