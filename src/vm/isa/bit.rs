use std::u8;

use log::trace;

use crate::types::AddressingMode;
use crate::util::{get_bits, get_reg, is_bit_set};
use crate::vm::Cpu;
use crate::StatusRegister as SR;

impl<'a> Cpu<'a> {
    pub(crate) fn bit_family(&mut self, inst: u16) {
        if get_bits(inst, 3, 3) == 0b001 {
            return self.movep(inst);
        }
        match get_bits(inst, 6, 2) {
            0b00 => self.btst(inst),
            0b01 => self.bchg(inst),
            0b10 => self.bclr(inst),
            0b11 => self.bset(inst),
            _ => unreachable!(),
        }
    }

    fn btst(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let modulo: u8 = if let AddressingMode::DataRegisterDirect(_) = ea {
            32
        } else {
            8
        };
        let val = self.read_ea_byte(ea);

        let res = if is_bit_set(inst, 8) {
            // Reg
            let reg = get_reg(inst, 9);
            let bit = self.read_dr(reg) as u8 % modulo;
            is_bit_set(val, bit)
        } else {
            // Imm
            let bit = ((self.fetch_word() & 0xFF) as u8) % modulo;
            is_bit_set(val, bit)
        };
        self.write_ccr(SR::Z, res);
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

    fn movep(&mut self, inst: u16) {
        // movep.bin - D2: 00001234, D3: B00B9876, D4: 1234FFFF
        match get_bits(inst, 6, 2) {
            0b00 => self.movep_word_mem_to_reg(inst),
            0b01 => self.movep_long_mem_to_reg(inst),
            0b10 => self.movep_word_reg_to_mem(inst),
            0b11 => self.movep_long_reg_to_mem(inst),
            _ => unreachable!(),
        }
    }

    fn movep_word_mem_to_reg(&mut self, inst: u16) {
        let addr = get_reg(inst, 0);
        let data = get_reg(inst, 9);
        let displacement = self.fetch_signed_word();
        let target = (self.read_ar(addr) as i64 + displacement as i64) as u32;
        if displacement == 0 {
            trace!("MOVEP.w (A{addr}), D{data}");
        } else {
            trace!("MOVEP.w {displacement}(A{addr}), D{data}");
        }
        let high = self.mmu.read_byte(target);
        let low = self.mmu.read_byte(target + 2);
        self.write_dr_word(data, ((high as u16) << 8) + low as u16);
    }

    fn movep_long_mem_to_reg(&mut self, inst: u16) {
        let addr = get_reg(inst, 0);
        let data = get_reg(inst, 9);
        let displacement = self.fetch_signed_word();
        let target = (self.read_ar(addr) as i64 + displacement as i64) as u32;
        if displacement == 0 {
            trace!("MOVEP.l (A{addr}), D{data}");
        } else {
            trace!("MOVEP.l {displacement}(A{addr}), D{data}");
        }
        let high = self.mmu.read_byte(target);
        let mid_high = self.mmu.read_byte(target + 2);
        let mid_low = self.mmu.read_byte(target + 4);
        let low = self.mmu.read_byte(target + 6);
        let val = ((high as u32) << 24)
            + ((mid_high as u32) << 16)
            + ((mid_low as u32) << 8)
            + low as u32;
        self.write_dr_long(data, val);
    }

    fn movep_word_reg_to_mem(&mut self, inst: u16) {
        let addr = get_reg(inst, 0);
        let data = get_reg(inst, 9);
        let displacement = self.fetch_signed_word();
        let target = (self.read_ar(addr) as i64 + displacement as i64) as u32;
        if displacement == 0 {
            trace!("MOVEP.w D{data}, (A{addr})");
        } else {
            trace!("MOVEP.w D{data}, {displacement}(A{addr})");
        }
        let val = self.read_dr(data);
        let high = (0xFF00 & val) >> 8;
        let low = 0xFF & val;
        self.mmu.write_byte(target, high as u8);
        self.mmu.write_byte(target + 2, low as u8);
    }

    fn movep_long_reg_to_mem(&mut self, inst: u16) {
        let addr = get_reg(inst, 0);
        let data = get_reg(inst, 9);
        let displacement = self.fetch_signed_word();
        let target = (self.read_ar(addr) as i64 + displacement as i64) as u32;
        if displacement == 0 {
            trace!("MOVEP.l D{data}, (A{addr})");
        } else {
            trace!("MOVEP.l D{data}, {displacement}(A{addr})");
        }
        let val = self.read_dr(data);
        let high = (0xFF000000 & val) >> 24;
        let mid_high = (0xFF0000 & val) >> 16;
        let mid_low = (0xFF00 & val) >> 8;
        let low = 0xFF & val;
        self.mmu.write_byte(target, high as u8);
        self.mmu.write_byte(target + 2, mid_high as u8);
        self.mmu.write_byte(target + 4, mid_low as u8);
        self.mmu.write_byte(target + 6, low as u8);
    }
}
