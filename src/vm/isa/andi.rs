use log::{error, trace};

use crate::{types::AddressingMode, util::get_size, vm::cpu::Cpu, Vector};

impl<'a> Cpu<'a> {
    pub(super) fn andi_family(&mut self, inst: u16) {
        if inst == 0b0000_0010_0011_1100 {
            return self.andi_to_ccr();
        }
        if inst == 0b0000_0010_0111_1100 {
            return self.andi_to_sr();
        }

        self.andi(inst);
    }

    fn andi_to_ccr(&mut self) {
        let val = self.fetch_word() & 0xFF;
        let old = self.read_sr();
        trace!("ANDI to CCR {val:#010b}");
        self.write_sr((old & 0xFF00) + ((old & 0xFF) & val));
    }

    fn andi_to_sr(&mut self) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
        let val = self.fetch_word();
        let old = self.read_sr();
        trace!("ANDI to SR {val:#018b}");
        self.write_sr(old & (val & 0b1010_0111_1111_1111));
    }

    fn andi(&mut self, inst: u16) {
        let size = get_size(inst, 6, crate::util::SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        trace!("ANDI.{size} {ea} ({val})");
        todo!()
    }
}
