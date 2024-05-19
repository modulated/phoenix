use crate::{
    types::{AddressingMode, Size, Value},
    util::{get_reg, get_size, is_bit_set, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn rot_family(&mut self, inst: u16) {
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
        if is_bit_set(inst, 8) {
            self.lsr_reg(inst);
        } else {
            self.lsl_reg(inst);
        }
    }

    fn lsr_reg(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let count = get_reg(inst, 9);
        let a_reg = get_reg(inst, 0);
        let shift_count = if is_bit_set(inst, 5) {
            (self.read_dr(count) % 64) as u8
        } else if count == 0 {
            8u8
        } else {
            count
        };
        println!("LSR {count} Ar{a_reg} {size:?}");
        let res = self.read_ar(a_reg) >> shift_count; // TODO: 24 bit mask?
        self.write_ar(a_reg, res);
        // TODO flags
    }

    fn lsl_reg(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let count = get_reg(inst, 9);
        let a_reg = get_reg(inst, 0);
        let shift_count = if is_bit_set(inst, 5) {
            (self.read_dr(count) % 64) as u8
        } else if count == 0 {
            8u8
        } else {
            count
        };
        let val = self.read_ar(a_reg);
        let res = val << shift_count; // TODO: 24 bit mask?
        println!("LSL {shift_count} Ar{a_reg}: {val:#X} {size:?}");
        self.write_ar(a_reg, res);
        // TODO flags
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
        if is_bit_set(inst, 8) {
            self.lsr_mem(inst);
        } else {
            self.lsl_mem(inst);
        }
    }

    fn lsr_mem(&mut self, inst: u16) {
        let size = match (inst & 0b0000_0000_1100_0000) >> 6 {
            0b00 => Size::Byte,
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        };
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        println!("LSR {ea:?}: {val}");
        let _rot = match val {
            Value::Byte(v) => v % 64,
            Value::Word(v) => (v % 64) as u8,
            Value::Long(v) => (v % 64) as u8,
        };
        todo!()
    }

    fn lsl_mem(&mut self, inst: u16) {
        let size = match (inst & 0b0000_0000_1100_0000) >> 6 {
            0b00 => Size::Byte,
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        };
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        println!("LSL {ea:?}: {val}");
        let _rot = match val {
            Value::Byte(v) => v % 64,
            Value::Word(v) => (v % 64) as u8,
            Value::Long(v) => (v % 64) as u8,
        };
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
        let val = self.read_ea_word(ea);
        println!("ROXL {ea:?}: {val:#X}");
        todo!()
    }

    fn roxr_mem(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let val = self.read_ea_word(ea);
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
