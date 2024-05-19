use crate::{
    types::{AddressingMode, Size, Value},
    util::{get_size, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn util_family(&mut self, inst: u16) {
        if (inst & 0b0000_0001_1100_0000) == 0b0000_0001_1100_0000 {
            return self.lea(inst);
        }
        if (inst & 0b0000_0001_1100_0000) == 0b0000_0001_1000_0000 {
            return self.chk(inst);
        }
        match inst {
            0b0100_0000_1100_0000..=0b0100_0000_1111_1111 => self.move_from_sr(inst),
            0b0100_0100_1100_0000..=0b0100_0100_1111_1111 => self.move_to_ccr(inst),
            0b0100_0110_1100_0000..=0b0100_0110_1111_1111 => self.move_to_sr(inst),
            // negx
            0b0100_0010_0000_0000..=0b0100_0010_1111_1111 => self.clr(inst),
            // neg
            // not
            // ext
            // nbcd
            // swap
            // pea
            0b0100_1010_1111_1100 => self.illegal(),
            0b0100_1010_1100_0000..=0b0100_1010_1111_1111 => self.tas(inst),
            // tst
            // trap
            // link
            // unlk
            // move_usp
            // reset
            0b0100_1110_0111_0001 => self.nop(),
            // stop
            0b0100_1110_0111_0011 => self.rte(),
            0b0100_1110_0111_0101 => self.rts(),
            0b0100_1110_0111_0110 => self.trapv(),
            0b0100_1110_0111_0111 => self.rtr(),
            0b0100_1110_1000_0000..=0b0100_1110_1011_1111 => self.jsr(inst),
            0b0100_1110_1100_0000..=0b0100_1110_1111_1111 => self.jmp(inst),
            _ => panic!("Instruction Not Found"),
        }
    }
    fn move_from_sr(&mut self, _inst: u16) {
        todo!()
    }

    fn move_to_ccr(&mut self, _inst: u16) {
        todo!()
    }

    fn move_to_sr(&mut self, _inst: u16) {
        todo!()
    }

    fn illegal(&mut self) {
        todo!()
    }

    fn tas(&mut self, _inst: u16) {
        todo!()
    }

    fn nop(&mut self) {}

    fn rte(&mut self) {
        todo!()
    }

    fn rts(&mut self) {
        todo!()
    }

    fn trapv(&mut self) {
        todo!()
    }

    fn rtr(&mut self) {
        todo!()
    }

    fn lea(&mut self, inst: u16) {
        let reg = (inst & 0b0000_1110_0000_0000) >> 9;
        let ea = AddressingMode::from(inst);
        let val = self.read_ea_long(ea);
        println!("LEA A{reg} <= {ea:?}: {val:#x}");
        self.write_ar(reg.try_into().unwrap(), val);
    }

    fn clr(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = match size {
            Size::Byte => Value::Byte(0),
            Size::Word => Value::Word(0),
            Size::Long => Value::Long(0),
        };
        println!("CLR {ea:?}: {size:?}");
        self.write_ea(ea, size, val);
        self.write_sr(SR::N, false);
        self.write_sr(SR::Z, true);
        self.write_sr(SR::V, false);
        self.write_sr(SR::C, false);
    }

    fn chk(&mut self, _inst: u16) {
        todo!()
    }

    fn jsr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.read_ea_long(ea);
        println!("JSR PC <= {ea:?}: {val:x}");
        self.decrement_ar(7, 4);
        self.mmu
            .write_long(self.read_ar(Cpu::STACK), self.read_pc());
        self.write_pc(val);
    }

    fn jmp(&mut self, _inst: u16) {
        todo!()
    }
}
