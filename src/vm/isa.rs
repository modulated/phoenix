use crate::types::{AddressingMode, Size};
use crate::vm::cpu::Cpu;

impl<'a> Cpu<'a> {
    pub(super) fn exec(&mut self, inst: u16) {
        println!("Instruction: {inst:#018b}");
        match inst {
            0b0000_0000_0000_0000..=0b0000_0000_1011_1111 => self.ori_family(inst),
            0b0000_0010_0011_1100..=0b0000_0010_1111_1111 => self.andi_family(inst),
            0b0001_0000_0000_0000..=0b0011_1111_1111_1111 => self.move_family(inst),
            0b0100_1110_0111_0001 => self.nop(),
            0b1100_0000_0000_0000..=0b1100_1111_1111_1111 => self.abcd(inst),

            _ => panic!("Unimplemented: {:#018b}", inst),
        }
    }

    /*    ORI    */
    fn ori_family(&mut self, inst: u16) {
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

    /*    ANDI    */
    fn andi_family(&mut self, inst: u16) {
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
        self.sr &= operand & 0b0000_0000_0001_1111;
    }

    fn andi_to_sr(&mut self) {
        let operand = self.fetch_word();
        self.sr &= operand & 0b0000_0000_0001_1111;
    }

    fn andi(&mut self, _inst: u16) {
        todo!()
    }

    fn nop(&mut self) {}

    fn move_family(&mut self, inst: u16) {
        let mode = (inst & 0b0000_0001_1100_0000) >> 6;
        if mode == 0b001 {
            return self.movea(inst);
        }
        self.r#move(inst);
    }

    fn movea(&mut self, inst: u16) {
        let size = (0b0011_0000_0000_0000 & inst) >> 12;
        let size = match size {
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        };
        let _dest = (0b0000_1110_0000_0000 & inst) >> 9;
        let ea = AddressingMode::from(inst);
        let _val = self.get_ea(ea, size);
        todo!()
    }

    fn r#move(&mut self, _inst: u16) {
        todo!()
    }

    fn abcd(&mut self, _inst: u16) {
        todo!()
    }
}
