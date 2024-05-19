use crate::vm::cpu::Cpu;
use std::io::Write;

mod andi;
mod branch;
mod mathq;
mod r#move;
mod ori;
mod rot;
mod sub;
mod util;

impl<'a> Cpu<'a> {
    pub(super) fn exec(&mut self, inst: u16) {
        print!("Op: {inst:#018b}\t");
        let _ = std::io::stdout().flush();
        match inst {
            0b0000_0000_0000_0000..=0b0000_0000_1011_1111 => self.ori_family(inst),
            0b0000_0010_0011_1100..=0b0000_0010_1111_1111 => self.andi_family(inst),
            0b0001_0000_0000_0000..=0b0011_1111_1111_1111 => self.move_family(inst),
            0b0100_0000_1100_0000..=0b0100_1111_1111_1111 => self.util_family(inst),
            0b0101_0000_0000_0000..=0b0101_1111_1100_1111 => self.mathq_family(inst),
            0b0110_0000_0000_0000..=0b0110_1111_1111_1111 => self.branch_family(inst),
            0b1001_0000_0000_0000..=0b1001_1111_1111_1111 => self.sub_family(inst),
            0b1110_0000_1100_0000..=0b1110_1111_1111_1111 => self.rot_family(inst),
            _ => panic!("Unimplemented: {:#018b}", inst),
        }
    }
}
