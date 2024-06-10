use crate::vm::cpu::Cpu;
use std::io::Write;

mod add;
mod andi;
mod bit;
mod branch;
mod cmp;
mod div;
mod eori;
mod io;
mod mathq;
mod r#move;
mod movem;
mod mul;
mod ori;
mod rot;
mod sub;
mod util;
mod zero;

impl<'a> Cpu<'a> {
    pub(super) fn exec(&mut self, inst: u16) {
        let _ = std::io::stdout().flush();
        match inst {
            0b0000_0000_0000_0000..=0b0000_1111_1111_1111 => self.zero_family(inst),
            0b0001_0000_0000_0000..=0b0011_1111_1111_1111 => self.move_family(inst),
            0b0100_0000_1100_0000..=0b0100_1111_1111_1111 => self.util_family(inst),
            0b0101_0000_0000_0000..=0b0101_1111_1100_1111 => self.mathq_family(inst),
            0b0110_0000_0000_0000..=0b0110_1111_1111_1111 => self.branch_family(inst),
            0b0111_0000_0000_0000..=0b0111_1110_1111_1111 => self.moveq(inst),
            0b1000_0000_1100_0000..=0b1000_1110_1111_1111 => self.div_family(inst),
            0b1001_0000_0000_0000..=0b1001_1111_1111_1111 => self.sub_family(inst),
            0b1011_0000_0000_0000..=0b1011_1111_1111_1111 => self.cmp_family(inst),
            0b1100_0000_1100_0000..=0b1100_1110_1111_1111 => self.mul_family(inst),
            0b1101_0000_0000_0000..=0b1101_1111_1111_1111 => self.add_family(inst),
            0b1110_0000_0000_0000..=0b1110_1111_1111_1111 => self.rot_family(inst),
            0xFFFF => self.halt(),
            _ => panic!("Unimplemented: {:#018b}", inst),
        }
    }
}
