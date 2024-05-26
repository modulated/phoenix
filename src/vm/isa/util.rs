use log::trace;

use crate::{
    types::{AddressingMode, ExtensionMode, Size, Value},
    util::{get_reg, get_size, is_bit_set, SizeCoding},
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
        if (inst & 0b0000_1011_1100_0000) == 0b0000_1000_1000_0000 {
            return self.movem_word(inst);
        }
        if (inst & 0b0000_1011_1100_0000) == 0b0000_1000_1100_0000 {
            return self.movem_long(inst);
        }
        match inst {
            0b0100_0000_1100_0000..=0b0100_0000_1111_1111 => self.move_from_sr(inst),
            0b0100_0100_1100_0000..=0b0100_0100_1111_1111 => self.move_to_ccr(inst),
            0b0100_0110_1100_0000..=0b0100_0110_1111_1111 => self.move_to_sr(inst),
            0b0100_0000_0000_0000..=0b0100_0000_1011_1111 => self.negx(inst),
            0b0100_0010_0000_0000..=0b0100_0010_1111_1111 => self.clr(inst),
            0b0100_0100_0000_0000..=0b0100_0100_1011_1111 => self.neg(inst),
            0b0100_0110_0000_0000..=0b0100_0110_1011_1111 => self.neg(inst),
            0b0100_1000_1000_0000..=0b0100_1000_1100_0111 => self.ext(inst),
            // nbcd
            // swap
            // pea
            0b0100_1010_1111_1100 => self.illegal(),
            0b0100_1010_0000_0000..=0b0100_1010_1011_1111 => self.tst(inst),
            0b0100_1010_1100_0000..=0b0100_1010_1111_1111 => self.tas(inst),
            0b0100_1110_0100_0000..=0b0100_1110_0100_1111 => self.trap(inst),
            0b0100_1110_0101_0000..=0b0100_1110_0101_0111 => self.link(inst),
            0b0100_1110_0101_1000..=0b0100_1110_0101_1111 => self.unlk(inst),
            0b0100_1110_0110_0000..=0b0100_1110_0110_1111 => self.move_usp(inst),
            0b0100_1110_0111_0000 => self.reset(),
            0b0100_1110_0111_0001 => self.nop(),
            0b0100_1110_0111_0010 => self.stop(),
            0b0100_1110_0111_0011 => self.rte(),
            0b0100_1110_0111_0101 => self.rts(),
            0b0100_1110_0111_0110 => self.trapv(),
            0b0100_1110_0111_0111 => self.rtr(),
            0b0100_1110_1000_0000..=0b0100_1110_1011_1111 => self.jsr(inst),
            0b0100_1110_1100_0000..=0b0100_1110_1111_1111 => self.jmp(inst),
            _ => panic!("Instruction Not Found"),
        }
    }
    fn move_from_sr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.read_sr();
        self.write_ea_word(ea, val);
        trace!("MOVE from SR {ea:?}: {val:X}")
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

    fn tst(&mut self, _inst: u16) {
        todo!()
    }

    fn tas(&mut self, _inst: u16) {
        todo!()
    }

    fn trap(&mut self, _inst: u16) {
        todo!()
    }

    fn link(&mut self, inst: u16) {
        let reg = get_reg(inst, 0);
        let val = self.read_ar(reg);
        let disp = self.fetch_signed_word();
        self.push_long(val);
        self.write_ar(reg, self.read_sp());
        let new_sp = (self.read_sp() as i64 + disp as i64) as u32;
        self.write_sp(new_sp);
        trace!("{} LINK {reg} {disp}", self.read_pc());
    }

    fn unlk(&mut self, _inst: u16) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn nop(&mut self) {}

    fn stop(&mut self) {
        todo!()
    }

    fn rte(&mut self) {
        todo!()
    }

    fn rts(&mut self) {
        let pc = self.pop_long();
        trace!("{} RTS", self.read_pc());
        self.write_pc(pc);
    }

    fn trapv(&mut self) {
        todo!()
    }

    fn rtr(&mut self) {
        todo!()
    }

    fn movem_long(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let mask = self.fetch_word();

        let start = self.read_ea_long(ea);
        let mut cur = start;

        if is_bit_set(inst, 10) {
            // Memory to Register
            trace!("{} MOVEM.l {ea:?}: {start} => {mask:018b}", self.read_pc());
            assert!(match ea {
                AddressingMode::DataRegisterDirect(_) => false,
                AddressingMode::AddressRegisterDirect(_) => false,
                AddressingMode::AddressRegisterIndirect(_) => true,
                AddressingMode::AddressRegisterIndirectPostIncrement(_) => true,
                AddressingMode::AddressRegisterIndirectPreDecrement(_) => false,
                AddressingMode::AddressRegisterIndirectDisplacement(_) => true,
                AddressingMode::AddressRegisterIndirectIndex(_) => true,
                AddressingMode::Extension(e) => match e {
                    ExtensionMode::Word => true,
                    ExtensionMode::Long => true,
                    ExtensionMode::PcRelativeDisplacement => true,
                    ExtensionMode::PcRelativeIndex => true,
                    ExtensionMode::Immediate => false,
                },
            });

            for reg in 0..8 {
                // D
                if is_bit_set(mask, reg) {
                    let val = self.mmu.read_long(cur);
                    self.write_dr(reg, val);
                    cur += 4;
                }
            }
            for reg in 0..8 {
                // A
                if is_bit_set(mask >> 8, reg) {
                    let val = self.mmu.read_long(cur);
                    self.write_ar(reg, val);
                    cur += 4;
                }
            }
        } else {
            // Register to Memory
            trace!(
                "{}: MOVEM.l {mask:#X} => {ea:?}: {start:#X}",
                self.read_pc()
            );
            assert!(match ea {
                AddressingMode::DataRegisterDirect(_) => false,
                AddressingMode::AddressRegisterDirect(_) => false,
                AddressingMode::AddressRegisterIndirect(_) => true,
                AddressingMode::AddressRegisterIndirectPostIncrement(_) => false,
                AddressingMode::AddressRegisterIndirectPreDecrement(_) => true,
                AddressingMode::AddressRegisterIndirectDisplacement(_) => true,
                AddressingMode::AddressRegisterIndirectIndex(_) => true,
                AddressingMode::Extension(e) => match e {
                    ExtensionMode::Word => true,
                    ExtensionMode::Long => true,
                    ExtensionMode::PcRelativeDisplacement => false,
                    ExtensionMode::PcRelativeIndex => false,
                    ExtensionMode::Immediate => false,
                },
            });

            for reg in 0..8 {
                // D
                if is_bit_set(mask, reg) {
                    let val = self.read_dr(reg);
                    self.mmu.write_long(cur, val);
                    cur += 4;
                }
            }
            for reg in 0..8 {
                // A
                if is_bit_set(mask >> 8, reg) {
                    let val = self.read_ar(reg);
                    self.mmu.write_long(cur, val);
                    cur += 4;
                }
            }
            // TODO - finish logic
        }
    }

    fn movem_word(&mut self, _inst: u16) {
        todo!()
    }

    fn move_usp(&mut self, _inst: u16) {
        todo!()
    }

    fn lea(&mut self, inst: u16) {
        let reg = get_reg(inst, 9);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea_long(ea);
        trace!("LEA A{reg} {ea} ({val:#010X})");
        self.write_ar(reg, val);
    }

    fn clr(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = match size {
            Size::Byte => Value::Byte(0),
            Size::Word => Value::Word(0),
            Size::Long => Value::Long(0),
        };
        self.write_ea(ea, size, val);
        trace!("CLR.{size} {ea}");
        self.write_ccr(SR::N, false);
        self.write_ccr(SR::Z, true);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);
    }

    fn negx(&mut self, _inst: u16) {
        todo!()
    }

    fn neg(&mut self, _inst: u16) {
        todo!()
    }

    fn ext(&mut self, _inst: u16) {
        todo!()
    }

    fn chk(&mut self, _inst: u16) {
        todo!()
    }

    fn jsr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let addr = self.get_jmp_address(ea);
        trace!("JSR {ea} ({addr:#X})");
        self.push_long(self.read_pc());
        self.write_pc(addr);
    }

    fn jmp(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let addr = self.get_jmp_address(ea);
        trace!("JMP {ea} ({addr:#X})");
        self.write_pc(addr);
    }
}
