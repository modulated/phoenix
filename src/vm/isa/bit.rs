use crate::util::get_bits;
use crate::vm::Cpu;

impl<'a> Cpu<'a> {
    pub(crate) fn bit_family(&mut self, inst: u16) {
        match get_bits(inst, 6, 2) {
            0b00 => self.btst(inst),
            0b01 => self.bchg(inst),
            0b10 => self.bclr(inst),
            0b11 => self.bset(inst),
            _ => unreachable!(),
        }
    }

    fn btst(&mut self, _inst: u16) {
        todo!()
    }

    fn bchg(&mut self, _inst: u16) {
        todo!()
    }

    fn bclr(&mut self, _inst: u16) {
        todo!()
    }

    fn bset(&mut self, _inst: u16) {
        todo!()
    }
}
