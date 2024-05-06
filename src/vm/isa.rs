use std::io::Write;

use crate::types::{sign_extend_16_to_32, AddressingMode, Size, Value};
use crate::vm::cpu::{Cpu, StatusRegister as SR};

const STACK: u8 = 7;

impl<'a> Cpu<'a> {
    pub(super) fn exec(&mut self, inst: u16) {
        print!("Op: {inst:#018b}\t");
        let _ = std::io::stdout().flush();
        match inst {
            0b0000_0000_0000_0000..=0b0000_0000_1011_1111 => self.ori_family(inst),
            0b0000_0010_0011_1100..=0b0000_0010_1111_1111 => self.andi_family(inst),
            0b0001_0000_0000_0000..=0b0011_1111_1111_1111 => self.move_family(inst),
            0b0100_0000_1100_0000..=0b0100_1111_1111_1111 => self.util_family(inst),
            0b1001_0000_0000_0000..=0b1001_1111_1111_1111 => self.sub_family(inst),
            0b1110_0000_1100_0000..=0b1110_1111_1111_1111 => self.rot_family(inst),
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
        // TODO: flags - self.sr &= operand & 0b0000_0000_0001_1111;
        println!("ANDI_TO_CCR {operand:x}");
    }

    fn andi_to_sr(&mut self) {
        let operand = self.fetch_word();
        // TODO: flags - self.sr &= operand & 0b0000_0000_0001_1111;
        println!("ANDI_TO_SR {operand:x}");
    }

    fn andi(&mut self, _inst: u16) {
        todo!()
    }

    /*    MOVE    */
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
        let dst = ((0b0000_1110_0000_0000 & inst) >> 9) as usize;
        let ea = AddressingMode::from(inst);
        let val = self.get_ea(ea, size);
        println!("MOVEA A{dst} <= {ea:?}: {val}");
        match val {
            Value::Word(v) => self.write_ar(dst.try_into().unwrap(), sign_extend_16_to_32(v)),
            Value::Long(v) => self.write_ar(dst.try_into().unwrap(), v),
            Value::Byte(_) => unreachable!(),
        }
    }

    fn r#move(&mut self, inst: u16) {
        let size = (0b0011_0000_0000_0000 & inst) >> 12;
        let size = match size {
            0b01 => Size::Byte,
            0b11 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        };
        let dst = AddressingMode::from(inst >> 6);
        let src = AddressingMode::from(inst);
        let val = self.get_ea(src, size);
        println!("MOVE {dst:?} <= {src:?}: {val}");
        self.write_ea(dst, size, val);

        // TODO: status flags
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

    /*    UTIL    */
    fn util_family(&mut self, inst: u16) {
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
            // clr
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
        let val = self.get_ea_long(ea);
        println!("LEA A{reg} <= {ea:?}: {val:#x}");
        self.write_ar(reg.try_into().unwrap(), val);
    }

    fn chk(&mut self, _inst: u16) {
        todo!()
    }

    /*    JUMP    */
    fn jsr(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.get_ea_long(ea);
        println!("JSR PC <= {ea:?}: {val:x}");
        self.decrement_ar(7, 4);
        self.mmu.write_long(self.read_ar(STACK), self.read_pc());
        self.write_pc(val);
    }

    fn jmp(&mut self, _inst: u16) {
        todo!()
    }

    /*    SUB    */
    fn sub_family(&mut self, inst: u16) {
        let reg = ((inst & 0b0000_1110_0000_0000) >> 9) as u8;
        let ea = AddressingMode::from(inst);
        let opmode = (inst & 0b0000_0001_1100_0000) >> 6;
        match opmode {
            0b000 => self.sub_dn(reg, ea, Size::Byte),
            0b001 => self.sub_dn(reg, ea, Size::Word),
            0b010 => self.sub_dn(reg, ea, Size::Long),
            0b011 => self.suba(reg, ea, Size::Word),
            0b100 => self.sub_ea(reg, ea, Size::Byte),
            0b101 => self.sub_ea(reg, ea, Size::Word),
            0b110 => self.sub_ea(reg, ea, Size::Long),
            0b111 => self.suba(reg, ea, Size::Long),
            _ => unreachable!(),
        };
    }

    fn suba(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        println!("SUBA {size:?} An:{reg} EA:{ea:?}");
        todo!()
    }

    fn sub_dn(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        println!("SUB {size:?} Dn:{reg} EA:{ea:?}");
        let val = match size {
            Size::Byte => self.get_ea_byte(ea) as u32,
            Size::Word => self.get_ea_word(ea) as u32,
            Size::Long => self.get_ea_long(ea),
        };
        let res = self.read_dr(reg) - val;
        // TODO: status flag
        self.write_dr(reg, res);
    }

    fn sub_ea(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        println!("SUB {size:?} EA:{ea:?} Dn:{reg}");
        todo!()
    }

    /*    ROT    */
    fn rot_family(&mut self, inst: u16) {
        if (inst & 0b0000_0000_1100_0000) == 0b0000_0000_1100_0000 {
            match (inst & 0b0000_1110_0000_0000) >> 9 {
                0b000 => self.asd_mem(inst),
                0b001 => self.lsd_mem(inst),
                0b010 => self.roxd_mem(inst),
                0b011 => self.rod_mem(inst),
                _ => unreachable!(),
            }
        } else {
            match (inst & 0b0000_0000_0001_1000) >> 3 {
                0b00 => self.asd_reg(inst),
                0b01 => self.lsd_reg(inst),
                0b10 => self.roxd_reg(inst),
                0b11 => self.rod_reg(inst),
                _ => unimplemented!(),
            }
        }
    }

    fn asd_reg(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("ASD {ea:?}");
        todo!()
    }

    fn lsd_reg(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("LSD {ea:?}");
        todo!()
    }

    fn roxd_reg(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("ROXD {ea:?}");
    }

    fn rod_reg(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("ROD {ea:?}");
        todo!()
    }

    fn asd_mem(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("ASD {ea:?}");
        todo!()
    }

    fn lsd_mem(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("LSD {ea:?}");
        todo!()
    }

    fn roxd_mem(&mut self, inst: u16) {
        if (inst & 0b0000_0001_0000_0000) == 0 {
            self.roxr_mem(inst);
        } else {
            self.roxl_mem(inst);
        }
    }

    fn roxl_mem(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.get_ea_word(ea);
        println!("ROXL {ea:?}: {val:#X}");
        todo!()
    }

    fn roxr_mem(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.get_ea_word(ea);
        println!("ROXR {ea:?}: {val:#X}");
        let out_bit = (0b1 & val) == 0b1;
        let mut val = val >> 1;
        if self.read_sr(SR::X) {
            val |= 0b1000_0000_0000_0000;
        }
        self.write_ea_word(ea, val);

        // SR
        self.write_sr(SR::C, out_bit);
        self.write_sr(SR::V, false);
        self.write_sr(SR::Z, val == 0);
        self.write_sr(
            SR::N,
            (val & 0b1000_0000_0000_0000) == 0b1000_0000_0000_0000,
        );
        self.write_sr(SR::X, out_bit);
    }

    fn rod_mem(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        println!("ROD {ea:?}");
        todo!()
    }
}
