use log::trace;

use crate::vm::cpu::Cpu;

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
        let operand = self.fetch_word();
        // TODO: flags - self.sr &= operand & 0b0000_0000_0001_1111;
        trace!("ANDI_TO_CCR {operand:x}");
        todo!()
    }

    fn andi_to_sr(&mut self) {
        let operand = self.fetch_word();
        // TODO: flags - self.sr &= operand & 0b0000_0000_0001_1111;
        trace!("ANDI_TO_SR {operand:x}");
        todo!()
    }

    fn andi(&mut self, _inst: u16) {
        todo!()
    }
}
