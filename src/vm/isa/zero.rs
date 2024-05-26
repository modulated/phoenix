use crate::vm::cpu::Cpu;

impl<'a> Cpu<'a> {
    pub fn zero_family(&mut self, inst: u16) {
        match inst {
            0b0000_0000_0000_0000..=0b0000_0000_1011_1111 => self.ori_family(inst),
            0b0000_0010_0011_1100..=0b0000_0010_1111_1111 => self.andi_family(inst),
            0b0000_0100_0000_0000..=0b0000_0100_1111_1111 => self.subi(inst),
            0b0000_0110_0000_0000..=0b0000_0110_1111_1111 => self.addi(inst),
            _ => unreachable!()
        }
    }
}