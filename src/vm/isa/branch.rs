use crate::{util::get_bits, vm::cpu::Cpu};

impl<'a> Cpu<'a> {
    pub(super) fn branch_family(&mut self, inst: u16) {
        match get_bits(inst, 8, 4) {
            0b0000 => self.bra(inst),
            0b0001 => self.bsr(inst),
            _ => self.bcc(inst),
        }
    }

    fn bra(&mut self, _inst: u16) {
        todo!()
    }

    fn bsr(&mut self, _inst: u16) {
        println!("BSR ");
        todo!()
    }

    fn bcc(&mut self, _inst: u16) {
        println!("BCC ");
        todo!()
    }
}
