use log::{error, trace};

use crate::{
    types::{AddressingMode, ExtensionMode, Size, Value},
    util::{
        get_reg, get_size, is_bit_set, is_negative, sign_extend_16_to_32, sign_extend_8_to_16,
        SizeCoding,
    },
    vm::cpu::Cpu,
    StatusRegister as SR, Vector,
};

impl<'a> Cpu<'a> {
    pub(super) fn util_family(&mut self, inst: u16) {
        if (inst & 0b0000_1111_1011_1000) == 0b0000_1000_1000_0000 {
            return self.ext(inst);
        }
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
            0b0100_0110_0000_0000..=0b0100_0110_1011_1111 => self.not(inst),
            0b0100_1000_0000_0000..=0b0100_1000_0011_1111 => self.nbcd(inst),
            0b0100_1000_0100_0000..=0b0100_1000_0100_0111 => self.swap(inst),
            0b0100_1000_0100_1000..=0b0100_1000_0111_1111 => self.pea(inst),
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
            _ => unreachable!("{inst:018b}"),
        }
    }
    fn move_from_sr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.read_sr();
        self.write_ea_word(ea, val);
        trace!("MOVE from SR {ea}: {val:#018b}")
    }

    fn move_to_ccr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = 0b0001_1111 & self.read_ea_word(ea);
        trace!("MOVE to CCR {ea} ({val:#05b})");
        let new = (self.read_sr() & 0xFF00) + val;
        self.write_sr(new);
    }

    fn move_to_sr(&mut self, inst: u16) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
        let ea = AddressingMode::from(inst);
        let val = 0b1010_0111_1111_1111 & self.read_ea_word(ea);
        trace!("MOVE TO SR {ea} ({val:#018b})");
        self.write_sr(val);
    }

    fn illegal(&mut self) {
        trace!("ILLEGAL");
        self.trap_vec(Vector::IllegalInstruction as u32);
    }

    fn tst(&mut self, _inst: u16) {
        todo!()
    }

    fn tas(&mut self, _inst: u16) {
        todo!()
    }

    fn trap(&mut self, inst: u16) {
        let vec = inst as u32 & 0b1111;
        error!("TRAP {vec}");
        self.trap_vec(vec * 4 + Vector::Trap as u32);
        todo!();        
    }

    fn link(&mut self, inst: u16) {
        let reg = get_reg(inst, 0);
        let val = self.read_ar(reg);
        let displacement = self.fetch_signed_word();
        self.push_long(val);
        let new_sp = (self.read_sp() as i64 + displacement as i64) as u32;
        self.write_ar(reg, self.read_sp());
        self.write_sp(new_sp);
        trace!("LINK {reg} {displacement}");
    }

    fn unlk(&mut self, inst: u16) {
        let reg = get_reg(inst, 0);
        trace!("UNLK A{reg}");
        self.write_sp(self.read_ar(reg));
        let new = self.pop_long();
        self.write_ar(reg, new);
    }

    fn reset(&mut self) {
        todo!()
    }

    fn nop(&mut self) {}

    fn stop(&mut self) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
        todo!()
    }

    fn rte(&mut self) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
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
                // Data
                if is_bit_set(mask, reg) {
                    let val = self.mmu.read_long(cur);
                    self.write_dr(reg, Size::Long, val);
                    cur += 4;
                }
            }
            for reg in 0..8 {
                // Addr
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

    fn move_usp(&mut self, inst: u16) {
        if !self.is_supervisor_mode() {
            error!("Not supervisor");
            self.trap_vec(Vector::PrivilegeViolation as u32);
        }
        let reg = get_reg(inst, 0);
        if is_bit_set(inst, 3) {
            // USP => An
            self.write_ar(reg, self.read_usp());
        } else {
            // An => USP
            self.write_usp(self.read_ar(reg));
        }
        trace!("MOVE USP A{reg}");
    }

    fn lea(&mut self, inst: u16) {
        let reg = get_reg(inst, 9);
        let ea = AddressingMode::from(inst);
        let val = self.get_ea(ea);
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

    fn not(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        let res: u32 = !(u32::from(val));
        self.write_ea(ea, size, Value::Long(res));
        trace!("NOT.{size} {ea} ({val})");
        self.write_ccr(SR::N, is_negative(res, size));
        self.write_ccr(SR::Z, res == 0);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);
    }

    fn ext(&mut self, inst: u16) {
        let reg = get_reg(inst, 0);
        let val = self.read_dr(reg);
        let (res, size) = if is_bit_set(inst, 6) {
            // Word to Long
            (sign_extend_16_to_32(val as u16), Size::Word)
        } else {
            // Byte to Word
            (sign_extend_8_to_16(val as u8) as u32, Size::Byte)
        };
        trace!("EXT D{reg}");
        self.write_dr(reg, size, res);
        self.write_ccr(SR::N, is_negative(val, size));
        self.write_ccr(SR::Z, res == 0);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);
    }

    fn nbcd(&mut self, _inst: u16) {
        todo!()
    }

    fn swap(&mut self, inst: u16) {
        let reg = get_reg(inst, 0);
        let val = self.read_dr(reg);
        let high = val >> 16;
        let low = 0xFFFF & val;
        let new = (low << 16) + high;
        trace!("SWAP D{reg}");
        self.write_dr(reg, Size::Long, new);
    }

    fn pea(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.get_ea(ea);
        trace!("PEA {ea} ({val:#X})");
        self.push_long(val);
    }

    fn chk(&mut self, inst: u16) {
        let size = get_size(inst, 7, SizeCoding::Purple);
        let reg = get_reg(inst, 9);
        let val1 = self.read_dr(reg);
        let ea = AddressingMode::from(inst);
        let val2 = u32::from(self.read_ea(ea, size)) as i32;
        trace!("CHK.{size} {ea} ({val2:#X}) D{reg}");
        if (val1 as i32) < 0 {
            self.write_ccr(SR::N, true);
            self.trap_vec(Vector::Chk as u32);
        }
        if (val1 as i32) > val2 {
            self.write_ccr(SR::N, false);
            self.trap_vec(Vector::Chk as u32);
        }        
    }

    fn jsr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let addr = self.get_ea(ea);
        trace!("JSR {ea} ({addr:#X})");
        self.push_long(self.read_pc());
        self.write_pc(addr);
    }

    fn jmp(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let addr = self.get_ea(ea);
        trace!("JMP {ea} ({addr:#X})");
        self.write_pc(addr);
    }

    pub(crate) fn halt(&mut self) {
        self.decrement_pc(2);
    }
}
