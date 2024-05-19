use crate::vm::cpu::Cpu;

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
        todo!()
    }

    fn ori_to_sr(&mut self) {
        todo!()
    }

    fn ori(&mut self, _inst: u16) {
        todo!()
    }
}
